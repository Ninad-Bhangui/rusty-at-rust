use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::List::{Cons, Nil};
//***********Reference cycle example */
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
fn ref_cycle() {
    
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
    println!("a next item = {:?}",a.tail());


}

//**********Tree example */
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

fn main() {
    // ref_cycle(); Uncomment for reference cycle stuff

    let leaf = Rc::new(Node{
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node{
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new())
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


    //*******Strong  count vs weak count */

    println!("*************strong count vs weak count********");
    let leaf = Rc::new(Node{
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    println!("Leaf Strong = {}, Weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node{
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new())
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("Branch Strong = {}, Weak = {}",Rc::strong_count(&branch),Rc::weak_count(&branch));    
        
        println!("Leaf Strong = {}, Weak = {}",Rc::strong_count(&leaf),Rc::weak_count(&leaf));    
        
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf));
}