//! Rust iterator extensions to operate on `Result`s effectively.
//!
//! ## [`try_all`](crate::TryAll::try_all(self))
//!
//! Tries all items of the iterator until one fails (or all succeed).
//!
//! ### Returns
//!
//! The iterator of all successes, or the first failure.
//! 
//! ### Examples
//!
//! Useful for propagating failures from within closures with `?` operator:
//! ```
//! # use crate::try_all::*;
//! fn parse_all_numbers(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
//! 	Ok(strs.iter().map(|s| s.parse()).try_all()?.collect())
//! }
//! ```
//!

mod iter;
pub use iter::*;
