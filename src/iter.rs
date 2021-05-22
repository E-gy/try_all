/// Trait providing try extensions to Iterators.
pub trait TryAll {
	type AllOrFirst;
	/// Tries all items of the iterator until one fails (or all succeed).
	/// 
	/// Useful for propagating failures from within closures with `?` operator:
	/// ```
	/// fn parse_all_numbers(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
	/// 	Ok(strs.iter().map(|s| s.parse()).try_all()?.collect())
	/// }
	/// ```
	///
	/// Due to the nature of operation, the function has to collect intermediate results.
	/// In other words, if production has sideffects, expect them for all items until \[including\] first failure.
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

