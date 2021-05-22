//! Rust iterator extensions to operate on `Result`s effectively.
//!
//! ## [`try_map_all`](crate::TryMapAll::try_map_all)
//! _and [`try_map_all_opt`](crate::TryMapAllOption::try_map_all_opt)_
//!
//! Applies a closure on all items of the iterator until one fails (or all succeed).
//!
//! ```rust
//! # use crate::try_all::*;
//! fn all_numbers_x2(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
//! 	Ok(strs.iter().try_map_all(|s| Ok(s.parse::<u64>()?*2))?.collect())
//! }
//! ```
//!
//! Respectively, for [`Option`]s:
//! ```rust
//! # use crate::try_all::*;
//! fn not_zero(is: Vec<u64>) -> Option<Vec<u64>> {
//! 	Some(is.into_iter().try_map_all_opt(|i| if i > 0 { Some(i) } else { None })?.collect())
//! }
//! ```
//!
//! _Note: once [#42327](https://github.com/rust-lang/rust/issues/42327) is merged, `try_map_all` will be implemented\* under one name for all try types._
//!
//! ## [`try_all`](crate::TryAll::try_all(self))
//!
//! Tries all items of the iterator until one fails (or all succeed).
//!
//! ```
//! # use crate::try_all::*;
//! fn parse_all_numbers(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
//! 	Ok(strs.iter().map(|s| s.parse()).try_all()?.collect())
//! }
//! ```
//!

mod iter;
pub use iter::*;
mod map;
pub use map::*;
