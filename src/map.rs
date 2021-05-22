type Recollector<T> = std::vec::IntoIter<T>;

/// Trait providing try map (result) extensions to Iterators.
///
/// See [`TryMapAll::try_map_all`].
///
/// Once `Try` (#42327) is stabilized, hopefully the `Result` and `Option` variants can be merged and generalized.
pub trait TryMapAll {
	type Item;
	/// Applies a closure on all items of the iterator until one fails (or all succeed).
	///
	/// # Arguments
	///
	/// - `f`: fallible mapping function
	///
	/// # Returns
	///
	/// The iterator of all successes, or the first failure.
	/// 
	/// # Examples
	///
	/// Useful for propagating failures from within closures with `?` operator:
	/// ```rust
	/// # use crate::try_all::*;
	/// fn all_numbers_x2(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
	/// 	Ok(strs.iter().try_map_all(|s| Ok(s.parse::<u64>()?*2))?.collect())
	/// }
	/// ```
	///
	/// # Equivalence
	///
	/// `iter.try_map_all(f)` is equivalent to `iter.map(f).try_all()`, except the latter works with `Option`s just as well (or check out [`TryMapAllOption::try_map_all_opt()`]).
	///
	/// Additionally, it is equivalent to
	/// ```rust
	/// # fn hehe<U, T, E>(iter: impl Iterator<Item=U>, f: impl Fn(U) -> Result<T, E>) -> Result<Vec<T>, E> {
	///	let mut acc = Vec::new();
	/// for item in iter {
	/// 	acc.push(f(item)?);
	/// }
	/// Ok(acc)
	/// # }
	/// ```
	///
	/// Due to the nature of operation, the function has to collect intermediate results.
	/// In other words, if `f` has side effects, expect them for all applications until \[including\] first failure.
	/// Additionally, this won't work on infinite sequences :o.
	fn try_map_all<T, E>(self, f: impl Fn(Self::Item) -> Result<T, E>) -> Result<Recollector<T>, E>;
}

impl<I: Iterator> TryMapAll for I {
	type Item = I::Item;
	fn try_map_all<T, E>(self, f: impl Fn(Self::Item) -> Result<T, E>) -> Result<Recollector<T>, E> {
		let mut ok = Vec::new();
		for t in self {
			ok.push(f(t)?);
		}
		Ok(ok.into_iter())
	}
}

/// Trait providing try map (option) extensions to Iterators.
///
/// See [`TryMapAllOption::try_map_all_opt`].
///
/// Once `Try` (#42327) is stabilized, hopefully the `Result` and `Option` variants can be merged and generalized.
pub trait TryMapAllOption {
	type Item;
	/// Applies a closure on all items of the iterator until one fails (or all succeed).
	///
	/// This is [`TryMapAll::try_map_all`] - `Option` edition.
	///
	/// # Arguments
	///
	/// - `f`: fallible mapping function
	///
	/// # Returns
	///
	/// The iterator of all successes, or the first failure.
	/// 
	/// # Examples
	///
	/// Useful for propagating failures from within closures with `?` operator:
	/// ```rust
	/// # use crate::try_all::*;
	/// fn not_zero(is: Vec<u64>) -> Option<Vec<u64>> {
	/// 	Some(is.into_iter().try_map_all_opt(|i| if i > 0 { Some(i) } else { None })?.collect())
	/// }
	/// ```
	///
	/// Due to the nature of operation, the function has to collect intermediate results.
	/// In other words, if `f` has side effects, expect them for all applications until \[including\] first failure.
	/// Additionally, this won't work on infinite sequences :o.
	fn try_map_all_opt<T>(self, f: impl Fn(Self::Item) -> Option<T>) -> Option<Recollector<T>>;
}

impl<I: Iterator> TryMapAllOption for I {
	type Item = I::Item;
	fn try_map_all_opt<T>(self, f: impl Fn(Self::Item) -> Option<T>) -> Option<Recollector<T>> {
		let mut ok = Vec::new();
		for t in self {
			ok.push(f(t)?);
		}
		Some(ok.into_iter())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_try_map_all(){
		assert_eq!(vec![Ok(0), Ok(1), Ok(2), Err("no u!")].into_iter().take(3).try_map_all(|i| i).map(|v| v.collect()), Ok(vec![0, 1, 2]));
		assert_eq!(vec![Ok(0), Ok(1), Err("no u!")].into_iter().try_map_all(|i| i).map(|v| v.collect::<Vec<_>>()), Err("no u!"));
		assert_eq!(vec![Err("me is 1st"), Ok(1), Err("no u!")].into_iter().try_map_all(|i| i).map(|v| v.collect::<Vec<_>>()), Err("me is 1st"));
	}
	#[test]
	fn test_try_map_all_opt(){
		assert_eq!(vec![Some(0), Some(1), Some(2)].into_iter().try_map_all_opt(|i| i).map(|v| v.collect::<Vec<_>>()), Some(vec![0, 1, 2]));
		assert_eq!(vec![Some(0), None, Some(2)].into_iter().try_map_all_opt(|i| i).map(|v| v.collect::<Vec<_>>()), None);
	}
}
