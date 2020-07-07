# Variables

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html>
- <https://doc.rust-lang.org/book/ch03-02-data-types.html>

### Stuff learnt

#### Variables are Immutable by default

```rust
let x = 5;
println!("The value of x is: {}", x);
x = 6;
println!("The value of x is: {}", x);
```

Above code will not execute as x is immutable.
Change it to

```rust
let mut x =5
```

to make it work.

Tradeoffs between mutable and immutable. Large data structures mutable may be better than copying and allocating new instance.

##### Differences Between Variables and Constants

- Immutable variables may sound like constants but they are not.
- Cannot use `mut` with constants. Always immutable.
- Declare using `const` instead of `let`.
- Example :

```rust
const MAX_POINTS: u32 = 100_000;
```

- Naming convention for constant : uppercase with underscores
- underscores can be added in numeric literals for readability.

- Valid for entire scope

#### Shadowing

```rust
    let x = 5;
    let x = x + 1; //Multiple declarations for same variable name
    let x = x * 2; //Note that this will not give immutable error as it is being redeclared.
```

Note that this will not give immutable error as it is being redeclared. The variable remains immutable after shadowing is done and so will not cause any later concurrency issues.

Another Example

```rust
    let spaces = "   ";
    let spaces = spaces.len();
```

This will work as `spaces` is being shadowed into new type.
Below would not work.

```rust
    let mut spaces = "   ";
    spaces = spaces.len();
```

This would not work as even though it's mutable, it is expecting number type by which it was declared and not int.

### Data Types

- Rust is statically typed. It must know type of all variables at compile time.
- It can usually infer type. When it can't, we must add type annontation

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

- Has Two types: Scalar and Compound
