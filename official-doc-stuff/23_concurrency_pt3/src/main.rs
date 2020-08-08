use std::sync::Mutex;
use std::thread;
use std::rc::Rc;

fn section1() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
        //Note that this prints Mutex {data : <locked>}
        println!("m = {:?}",m);
    }

    println!("m = {:?}",m);
}

//Will not compile as counter is moved in first iteration and cannot be used by multiple threads.
// Can be fixed by multiple Ownership using Rc<T>
// fn section2() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

//Will not compile as Rc cannot be used in concurrent scenarios.
// Rc does not manage counts in thread safe way.
// fn section3() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }
fn main() {
    // section1();
    // section2();
}
