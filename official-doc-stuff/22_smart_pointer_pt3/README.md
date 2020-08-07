# Reference Count Smart Pointers

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch15-04-rc.html>

## Intro

- Most cases a value has single owner.
- Few cases a value may have multiple owners. (A node in a graph may have multiple edges pointing to it)
- `Rc<T>` / Reference counting type keeps track of number of references to a value. If there are 0 references, value can be cleaned up without any invalid reference.

## Rc<T>

![](https://doc.rust-lang.org/book/img/trpl15-03.svg)

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

- Above will not work as `a` is moved while creating `b` and cannot be used while creating `c`

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

- Above will work.
- `Rc::clone` increases number of references.
- Data (`a`) will not be cleaned unless there are 0 references to it.
- Does not take time unlike `a.clone()` which creates a *Deep copy*. This only increments reference count.

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

- Can see reference counts, increase and decrease when `c` goes out of scope.
- **Only mutable references** allowed in `Rc<T>`. Mutability might break borrowing rules.
