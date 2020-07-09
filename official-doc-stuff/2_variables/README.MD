# Variables/DataTypes

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

#### Scalar Types

##### Integer

- Number without a fractional component
- Either signed on unsigned.
- Signed Range : -(2<sup>n</sup> - 1) to 2<sup>n-1</sup> - 1
- Unsigned Range : 0 to 2<sup>n</sup> - 1
   n -> number of bits. For example in `u8` n=8 and is unsigned.
   So Range : 0 - 255

```rust
    let mut overflow: u8 = 255;
    overflow = overflow + 1;
    println!("The value of overflow is: {}", overflow);
```

Above code will panic on `cargo run` but on `cargo run --release`, will perform two's complement wrapping and value would be 0.

##### Floating-point

- Types are `f32` ([single precision](https://en.wikipedia.org/wiki/Single-precision_floating-point_format)) and `f64` ([double precision](https://en.wikipedia.org/wiki/Single-precision_floating-point_format))

#### Arithematic operations

- Unlike python where you can add an innteger and floating point directly, this throws an [error](https://stackoverflow.com/questions/39677410/why-do-i-get-an-error-when-adding-an-integer-to-a-floating-point) in rust.

- To do so, either type will have to be converted

```rust
let mismatch_sum = f64::from(5) + 34.0; //convert int to float
let mismatch_sum = 5 as f64 + 34.0; //Another way to do the same


let mismatch_sum = 5 + 34.0 as i32; //Convert float to int.
let mismatch_sum = f64::from(5) + 34.0; //Does not work. No implementation for `{integer} + {float}`
```

Difference between [from](https://doc.rust-lang.org/std/convert/trait.From.html) and [as](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions) expalined [here](https://stackoverflow.com/questions/48795329/what-is-the-difference-between-fromfrom-and-as-in-rust).

In short,

- `from` is only implemented for lossless conversions. You can convert from `i32` to `i64` but not other way around.
- `as` works for both lossy and lossless.

#### Boolean Type

- `true`/`false`

#### Character Type

- 4 bytes in size
- `char` literals in single quotes and string literals double quotes

### Compound Types

### Tuple

- group of values of *different* types
- fixed length.
- `(30,1,20)`

```rust
tup = (30,1,20)
let x: (i32, f64, u8) = (500, 6.4, 1);
(x,y,z) = tup;
println!("y is {}",y); // 1
println!("Third element is {}",tup.2); //Access
```

### Array

- Group of values of *same* type
- Fixed length (Differnt from most languages in this)
- Data is on stack not heap. (Vectors are on heap with variable length)

```rust
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];

let a = [3; 5]; // [3,3,3,3,3];

let first = a[0];
let second = a[1];
```

- Accessing invalid index runs but panics with *runtime* error and exits

```rust
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:5:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

- In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed.
