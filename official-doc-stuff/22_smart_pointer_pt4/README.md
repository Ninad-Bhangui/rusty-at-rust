# RefCell\<T\> and the Interior Mutability Pattern

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch15-05-interior-mutability.html>

## Interior Mutability

- A design pattern in Rust that allows you to mutate data even when there are immutable references to that data.
- uses `unsafe` code

## RefCell\<T\>

- In `Refcell` borrow rules are enforced at runtime not compile time. Panics instead of compiler error
- Used in cases where code is memory safe but rust compiler does not know that and errors anyway to be safe.
- Only used in single threaded scenarios
- you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

Rest of the documentation explains an example. Look at the code [here](./src)
