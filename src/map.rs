type Recollector<T> = std::vec::IntoIter<T>;

pub trait TryMapAll {
	type Item;
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

pub trait TryMapAllOption {
	type Item;
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
