use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let a = 5;
    let b = Box::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);

    let x = 5;
    let y = MyBox(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox(String::from("Rust"));

    hello(&m);
}
