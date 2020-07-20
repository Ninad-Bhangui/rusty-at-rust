# Collections

Stores multiple values. Stored on heap.

### Creating Vector

- Stores values of the same type next to each other in memory.

```rust
let v: Vec<i32> = Vec::new();
```

Above would need type annotation as rust cannot infer type.
If few values were pushed later, It would not be needed as rust could infer it from the data.

```rust
let v = vec![1, 2, 3];
```

Usually Vectors are initialized with values like above using the macro. Here type can be inferred from what's passed to macro.

### Updating a Vector

```rust
let mut v = Vec::new();
v.push(5);
v.push(6);
```

Even if values are pushed after initialization, rust compiler is able to infer type.

### Vector and Scopes

```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v
} // <- v goes out of scope and is freed here
```

### Reading Elements of Vectors

```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
```

Above Panics.

```rust
    let v = vec![1, 2, 3, 4, 5];

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
```

Above will not Panic as it is handled using `Option` enum.

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
}
```

Above will not work. It implies an immutable and mutable reference exist at the same time.

Logically it should work as first element is not affected. But,

- Adding a new element to the vector might need more memory.
- Allocates new memory and copies old contents to it.
- So Reference would end up pointing to deallocated memory.

### Iteration

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

Above needs dereference operator

### Enum Vector for diff types

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

- Enums can be used to make vectors store different types.
- Since they are `enums` rust compiler would make sure with `match` all possible scenarios are handled when they're used.

### Extras

Delving into the documentation for Vector [Here](./vector_extras.md)
