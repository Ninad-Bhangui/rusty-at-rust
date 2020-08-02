# Iterators and Closures

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch14-01-release-profiles.html>

## Release profiles

- Rust has 2 main profiles - `dev` and `release`
- `dev` => `cargo build`
- `release` => `cargo build --release`
- `opt-level` makes rust decide optimizations to apply. level 0 being least number of optimizations
- `dev` => `opt-level 0` default
- `release` => `opt-level 3` default

```rust
[profile.dev]
opt-level = 1
```

- Overide defaults as above in `Cargo.toml`