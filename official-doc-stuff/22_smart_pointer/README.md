# Smart Pointers

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>

## Intro

- A pointer is a variable containing address to memory.
- Rust pointers don't have any overhead or extra capabilities other than pointing.
- Smart pointers are data structures which not only point but have additional metadata and capabilities.
- References only borrow while in many cases smart pointers own data.
- `String` and `Vec` are smart pointers as they own some memory and allow you to manipulate it. They have metadata like capacity, and capabilities like `String` making sure data is valid UTF-8.
- Implemented as Structs and implement `Deref` and `Drop` traits.
- `Deref` trait - allows instance of smart pointer to be usable by code as reference or smart pointer.
- `Drop` trait - customize code run when instance goes out of scope

## Box

- `Box<T>` - A smart pointer. Let's you store data on heap. pointer to heap data is stored on stack.
- No performance overhead but no extra capabilities.

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

- Here 5 is allocated on heap, and b points to it. No use allocating i32 on heap.

`Box` is explained below using *Cons* data structure.

## Cons List

- *Cons* is data structure from LISP and dialects. Used commonly in functional programming.

![](https://upload.wikimedia.org/wikipedia/commons/thumb/1/1b/Cons-cells.svg/350px-Cons-cells.svg.png)

```
(cons 42 (cons 69 (cons 613 nil)))
```

Example of cons above.

- Each pair has one value and another pair, making it recursive till it reaches `nil`.

Rust implementation

```rust
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

- Above code will not compile as rust would not know size of list at compile time.

- Enum sizes are calculated using the maximum space any variant might need. It looks for the largest variant.

![](https://doc.rust-lang.org/book/img/trpl15-01.svg)

- Above is how rust would try to allocate memory for Cons. Note the infinity.

- Box can be used to store *cons*. Instead of storing `List`, it would store a `Box` which would be a pointer to the actual list.

- Now all rust needs is space for one `i32` and one pointer whose size is static.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

- Above will compile

![](https://doc.rust-lang.org/book/img/trpl15-02.svg)

- Above is how rust would allocate now. Note that it is static and not infinite.
