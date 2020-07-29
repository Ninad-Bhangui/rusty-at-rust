# Generic Types, Traits and Lifetimes

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch10-00-generics.html>

## Intro

- We remove duplication of code by using functions.
- Generics achieve the same but for various types.
- We use `T` to represent multiple types and during compile time based on values passed to `T`, compiler makes the duplicates for those types. So there's no performance loss using Generics

## Using Generics in functions

Syntax :

```rust
fn largest<T>(list: &[T]) -> T {
```

Example :

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

Above code will not compile with error `Binary operation '>' cannot be applied to type T`
Will understand why in Traits section later

## Using Generics in Struct

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

- Since we used just one Generic Type `T`, we cannot assign x one type and y another. Like `let mix = Point {x:1, y: 4.0}`

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

Above would work as we use different type `T` and `U`

## Enum Definition

We have already used enums with Generics

```rust

enum Option<T> {
    Some(T),
    None,
}
```

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## In Method Definitions

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

- We can have a Generic implementation and a specific implementation for the same struct as shown above.
