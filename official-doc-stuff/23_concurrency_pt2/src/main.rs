use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn section1(){
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //Below line gives error. val borrowed after move
        // println!("val is {}", val);
    });

    let receieved = rx.recv().unwrap();
    println!("Got : {}", receieved);
}

fn section2() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //Each time recv is called on reciever, it gets the next message transmitted
    // let receieved1 = rx.recv().unwrap(); // Gets "hi"
    // let receieved2 = rx.recv().unwrap(); //Gets "from"
    // let receieved3 = rx.recv().unwrap(); //Gets "the"
    // let receieved4 = rx.recv().unwrap(); //Gets "thread"
    
    // Can be used as iterator as well. recv is not called here
    for recieved in rx {
        println!("Got : {}", recieved);
    }

}

fn section3(){
    let (tx,rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx {
        println!("Got : {}", recieved);
    }

}

fn main() {
    // section1();
    // section2();
    section3();
}
