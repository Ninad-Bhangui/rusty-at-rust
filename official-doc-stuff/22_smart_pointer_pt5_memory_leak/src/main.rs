use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}
fn main() {
    
    //Create a with (5,Nil)
    let a =Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count {}",Rc::strong_count(&a));
    println!("a next item {:?}",a.tail());

    //Create b with (10,a)
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation {}",Rc::strong_count(&a));
    println!("b initial rc count {}",Rc::strong_count(&b));
    println!("b next item {:?}",b.tail());

    //Update a by changing Nil to b
    //So now we have created a cycle where b points to a, and a points to b
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a {}",Rc::strong_count(&a));
    println!("a rc count after changing a {}",Rc::strong_count(&b));

    //Below line gives stack overflow core dumped
    // println!("a next item = {:?}",a.tail());


}
