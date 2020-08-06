# Iterators and Closures

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch13-00-functional-features.html>

## Closures

- Anonymous functions you can save in a variable or pass as arguments to other functions

```rust
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

- `expensive_closure` Contains definition of anonymous function not resulting value
- Closures don't need type annotations as Compiler infers it. Compiler will throw an error if same closure is called with two different types.

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;    //Closure using x works which is not in function scope

    let y = 4;

    assert!(equal_to_x(y));
}
```

- Closures can access variables from the scope they're defined in

**Note** I do not understand the below clearly. Attempt to understand clearly later and update

- All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`
`Fn` - Borrow values in scope immutably. (`x` would be borrowed immutably in above snippet)
`FnMut` - Borrow values mutably.
`FnOnce` - Consume the variable. It can take ownership only once. So this closure can be called only once. (`x` will not be usable after closure call)

Explained more in detail in [link](https://youtu.be/9PIn4suU3jM) and [link](https://huonw.github.io/blog/2015/05/finding-closure-in-rust/)

## Iterator

- Iterators are *lazy*. No effect until you call methods to consume (return something i guess) the iterator.
  
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

```

- Iterators need to be mutable when `next` is called as it changes the internal state to keep track of it's position in sequence
- This means the code consumes the iterator

## Consuming Adaptors

- Methods that call `next`
- For example `sum`, takes ownership of iterator and iterates by calling `next`

```rust
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
```

- Cannot use `v1_iter1` after first use as `sum` takes ownership of it.

## Iterator Adaptors

- Change iterator into different iterators

```rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

- Above code is valid but does nothing as iterators are lazy.
- A consuming adaptor is needed which consumes it and returns. `collect`

- Closures and iterators do not incur any performance loss.
