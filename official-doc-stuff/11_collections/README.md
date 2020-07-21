# Collections

Stores multiple values. Stored on heap.

## Vectors

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

## Strings

- Strings are always valid UTF-8
- 3 components *pointer to bytes*,  *length* and *capacity*.

### Create

```rust
let mut s = String::new();
let s = "initial contents".to_string();
let s = String::from("initial contents");
```

Above are 3 different ways to create string

### Push

```rust
let mut s = String::from("foo");
s.push_str("bar");
s.push('1');
```

- `push_str` takes string slice to avoid taking ownership
- `push` takes single character

### Concat

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
```

```rust
fn add(self, s: &str) -> String {
```

- `+` signature is kind of like the above. It requires `&str`
- Rust uses *deref coercion* to turn `&s2` which is `&String` into `&s2[..]` which is `&str`
- `self` is used in `+` (`add`) not `&self` so it takes ownership of `s1`

```rust
let s = format!("{} {}", s1 s3);
```

- Above marco does not take ownership
- Works like println but returns value instead

### Indexing

- A `String` is a wrapper over a `Vec<u8>`
- All characters in the string `"Здравствуйте"` take 2 bytes each, so normal indexing will point to non character and is disabled for that reason

```rust
let s1 = String::from("hello");
let h = s1[0];
```

Above will not work as indexing is not allowed in strings.

- Strings can be looked at as bytes, scalar values and grapheme clusters

#### Grapheme clusters

"नमस्ते"
Bytes : `[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]`

Scalar : `['न', 'म', 'स', '्', 'त', 'े']`

Grapheme Clusters : `["न", "म", "स्", "ते"]`

### Slicing

```rust

let hello = "Здравствуйте";

let s = &hello[0..4];
```

Giving ranges that are invalid character boundaries will cause pani

### Iteration

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

- Iterating over grapheme clusters not available by default. Crate required.

### Extras

The functions implemented for `String` are very similar to those implemented by `Vector`

## Hash Maps

- `HashMap<K, V>` stores a mapping of keys of type K to values of type V via a *hashing function*

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

### Ownership

- Copy trait types get copied and rest are moved when inserted in HashMap.

### Accessing values

```rust
let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

- `get` returns `Option<&V>` and value is wrapped in `Some(10)`

### Iterating

```rust
for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

- Similar to Vector. Can iterate over mutable reference too.

### Updating HashMap

- By default if `insert` is called and key exists it overwrites the value
- `or_insert` can be used to insert only if key has no value
- or_insert returns mutable reference to either old value or new value depending on it's existence in the Hash Map.