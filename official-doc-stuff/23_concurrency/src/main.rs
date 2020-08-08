use std::thread;
use std::time::Duration;

fn thread_section_1() {
    thread::spawn(|| {
        //Note that this prints till 4 sometimes, and sometimes till 5.
        //This is becaue this after main thread stops, the spawned thread stops too, 
        //It is only a matter of time if spawned thread reached 4 or 5
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn thread_section_2() {
    let handle = thread::spawn(|| {
        //Note that this prints till the end due to use of JoinHandle
        for i in 1..10 {
            println!("hi number {} from spawned thread using JoinHandle", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    };

    handle.join().unwrap();
}

fn thread_section_3() {
    let handle = thread::spawn(|| {
        //Note that this prints till the end due to use of JoinHandle
        for i in 1..10 {
            println!("hi number {} from spawned thread using JoinHandle", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //Note that the spawned thread runs until it completed before main thread prints anything
    //The spawned thread is blocking the main thread when using join.
    handle.join().unwrap();
    for i in 1..5 {
        println!("Hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    };
}

//Does not compile
// Uncomment to try. Gives error closure may outlive current function but borrows `v` owned by current function.vec!
// fn thread_section_4() {
//     let v = vec![1,2,3];

//     let handle = thread::spawn(|| {
//         println!("Vector : {:?}",v);
//     });
//     //dropping can cause issues as main thread may drop it before spawned runs.
//     v.drop();
//     handle.join().unwrap();
// }

fn main() {
    // thread_section_1();
    // thread_section_2();
    thread_section_3();
}
