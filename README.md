[![crates.io](https://img.shields.io/crates/v/try_all.svg)](https://crates.io/crates/try_all)

Rust iterator extensions to operate on `Result`s effectively.

## `try_all`

Tries all items of the iterator until one fails (or all succeed).

**Returns**: The iterator of all successes, or the first failure.

### Examples:
Useful for propagating failures from within closures with `?` operator
```
fn parse_all_numbers(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
	Ok(strs.iter().map(|s| s.parse()).try_all()?.collect())
}
```
