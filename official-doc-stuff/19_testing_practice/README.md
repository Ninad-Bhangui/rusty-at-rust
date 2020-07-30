# Generic Types, Traits and Lifetimes

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch11-01-writing-tests.html>

## Summary

Pretty straightforward and easy chapter.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

`#[test]` required to make a function a test function

Following are few helpful testing macros

1. `assert!` Expects Boolean True
2. `assert_eq!` and `asserts_ne!` expects two arguments passed to it are equal.
*Note that anything passed to this macro must implement `PartialEq` trait for comparision and `Debug` trait for printing on failure. `#[derive(PartialEq, Debug)]` helps in most cases instead of implementing those traits yourself. [See](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html).

You can test if a function panics using
`#[should_panic]` attribute.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

One can use `Result<T, E>` as return value in Test functions. `Ok` variant would mean it passed and `Err` would mean it failed.

## Ways to run Tests

### Parallel or Sequential

Tests by default run in parallel
To run sequentially,
`cargo test -- --test-threads=1`

### Show output

By default any `print` statements in functions being tested are ignored if test passes. They are displayed on failure. To show output
`cargo test -- --show-output`

### Running specific tests

```rust
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
```

#### Running Single

`cargo test one_hundred` : Runs only `fn one_hundred`

#### Running multiple using filter

`cargo test add` : Runs `add_two_and_two` and `add_three_and_two` since both match pattern `add`

#### Ignore tests

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

- `#[ignore]` lets `cargo test` ignore that function.
- To run either specify that test using `cargo test expensive_test` or run `cargo test -- --ignored`

## Test Organization

### Unit Tests

They're put in src directory in each file with the code they're testing.
Create a module named `tests` and annotate with `#[cfg(test)]`.
`#[cfg(test)]` tells rust not to compile the module during `cargo build` but only during `cargo test`.

### Testing Private Functions

There’s debate within the testing community about whether or not private functions should be tested directly.
However rust allows you to d o so

```rust
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

## Integration Tests

- Integration tests are written in a directory `tests` next to `src`.
- Each file in this directory is treated as seperate crate
- No annotation required here as it is in a special directory
- To run all integration tests in one file `cargo test --test file_name`
- They can have submodules in `common/mod.rs`. All tests can share methods in this module.
- We cannot do integration testing `main.rs` for binary crates. Rust creates a lib.rs even for binary crates where most logic should go and `main.rs` has minimum functionality of just using `lib.rs`. In this case integration tests can test `lib.rs`
