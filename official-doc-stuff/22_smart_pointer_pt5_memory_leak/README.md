# Reference Cycles

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch15-06-reference-cycles.html>

## Intro

- Memory leaks rare but possible.
- Rust guarantees no data race, so the memory leaks are memory safe.
- Items may refer to each other in a cycle and reference count would never reaceh 0 and value would not be dropped.

```rust
let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
```

- Above example, b points to a, and then a points to b. This creates a reference cycle. (More detailed code in `main.rs`)

## Preventing (Using Rc<T\> to Weak\<T\>)

- `Rc::clone` creates strong count. `Rc<T>` is cleaned up if strong count is 0.
- `Rc::downgrade` creates weak reference and gives smart pointer of type `Weak<T>`.
- weak_count does not have to be 0 for instance to be cleaned up
- strong references express ownership. Weak references do not and will not cause cycle as weak references are broken once strong count reaches 0.
- Getting value from weak reference is done using `upgrade` which returns `Option<T>` some if value has not been dropped else None. So even if strong count is 0, compiler makes sure use of any weak references handles non existent references using `Option<T>`.

##Tree Data structure

