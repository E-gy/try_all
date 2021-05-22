/// Trait providing try extensions to Iterators.
pub trait TryAll {
	type AllOrFirst;
	/// Ensures that all items of the iterator are Ok, otherwise returns the first failure.
	///	///
	/// # Returns
	///
	/// The iterator of all successes, or the first failure.
	/// 
	/// # Examples
	///
	/// Useful to ensure that all items are valid.
	/// ```rust
	/// # use crate::try_all::*;
	/// fn parse_all_numbers(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
	/// 	Ok(strs.iter().map(|s| s.parse()).try_all()?.collect())
	/// }
	/// ```
	///
	/// # Equivalence
	///
	/// `iter.try_all` is equivalent to `iter.try_map_all(|t| t)`, except all try-iable variants are implemented under a single name.
	///
	/// Additionally, it is equivalent to `collect`ing the iterator into [`Vec`], checking that `all()` are okay, and turning Vec back [`Vec::into_iter()`] (plus additional logic if the offending error needs to be retrieved). 
	///
	/// Due to the nature of operation, the function has to collect intermediate results.
	/// In other words, if production has side effects, expect them for all items until \[including\] first failure.
	/// Additionally, this won't work on infinite sequences :o.
	fn try_all(self) -> Self::AllOrFirst;
}

impl<I> TryAll for I where I: Iterator, (I, <I as Iterator>::Item): TryAllHack<Iter=I> {
	type AllOrFirst = <(I, <I as Iterator>::Item) as TryAllHack>::AllOrFirst;
	fn try_all(self) -> Self::AllOrFirst {
		<(I, <I as Iterator>::Item) as TryAllHack>::try_all(self)
	}
}

/// A hack trait to bypass limitations of type checker.
///
/// We have to hack until [rust-lang/rust#20400](https://github.com/rust-lang/rust/issues/20400) is closed.
/// [CC: paholg](https://stackoverflow.com/a/40408431).
pub trait TryAllHack {
	type Iter;
	type AllOrFirst;
	fn try_all(iter: Self::Iter) -> Self::AllOrFirst;
}

impl<T, I> TryAllHack for (I, Option<T>) where I: Iterator<Item=Option<T>> {
	type Iter = I;
	type AllOrFirst = Option<std::vec::IntoIter<T>>;
	fn try_all(iter: Self::Iter) -> Self::AllOrFirst {
		let mut ok = Vec::new();
		for t in iter {
			ok.push(t?);
		}
		Some(ok.into_iter())
	}
}

impl<T, E, I> TryAllHack for (I, Result<T, E>) where I: Iterator<Item=Result<T, E>> {
	type Iter = I;
	type AllOrFirst = Result<std::vec::IntoIter<T>, E>;
	fn try_all(iter: Self::Iter) -> Self::AllOrFirst {
		let mut ok = Vec::new();
		for t in iter {
			ok.push(t?);
		}
		Ok(ok.into_iter())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_try_all(){
		assert_eq!(vec![Some(0), Some(1), Some(2)].into_iter().try_all().map(|v| v.collect::<Vec<_>>()), Some(vec![0, 1, 2]));
		assert_eq!(vec![Some(0), None, Some(2)].into_iter().try_all().map(|v| v.collect::<Vec<_>>()), None);
		assert_eq!(vec![Ok(0), Ok(1), Ok(2), Err("no u!")].into_iter().take(3).try_all().map(|v| v.collect()), Ok(vec![0, 1, 2]));
		assert_eq!(vec![Ok(0), Ok(1), Err("no u!")].into_iter().try_all().map(|v| v.collect::<Vec<_>>()), Err("no u!"));
		assert_eq!(vec![Err("me is 1st"), Ok(1), Err("no u!")].into_iter().try_all().map(|v| v.collect::<Vec<_>>()), Err("me is 1st"));
	}
}
