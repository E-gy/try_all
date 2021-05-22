[![crates.io](https://img.shields.io/crates/v/try_all.svg)](https://crates.io/crates/try_all)

Rust iterator extensions to operate on `Result`s effectively.

## `try_map_all`
_and, for now, its friend `try_map_all_opt`_

Applies a closure on all items of the iterator until one fails (or all succeed).

**Arguments**:
- `f`: fallible mapping function

**Returns**: The iterator of all successes, or the first failure.

### Examples

Useful for propagating failures from within closures with `?` operator:
```rust
fn parse_all_numbers(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
	Ok(strs.iter().try_map_all(|s| s.parse())?.collect())
}
```
or for `Option`s:
```rust
fn not_zero(is: Vec<u64>) -> Option<Vec<u64>> {
	Some(is.iter().try_map_all_opt(|i| if i > 0 { Some(i) } else { None })?.collect())
}
```

## `try_all`

Ensures that all items of the iterator are Ok, otherwise returns the first failure.

**Returns**: The iterator of all successes, or the first failure.

### Examples:
Useful for propagating failures from within closures with `?` operator
```rust
fn parse_all_numbers(strs: &Vec<&str>) -> Result<Vec<u64>, std::num::ParseIntError> {
	Ok(strs.iter().map(|s| s.parse()).try_all()?.collect())
}
```
