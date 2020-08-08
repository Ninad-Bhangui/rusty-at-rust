# Concurrency

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch16-00-concurrency.html>

## Threads to run code simultaneously

### Threads

- Code is run in a process.
- OS manages multiple processes at once.
- Same program can run independant parts simultanously.
- Threads achieve the above

### Issues with threads

- Race conditions : threads accessing data/resources inconsistently
- Deadlocks : Two threads waiting for each other to finish using resource
- Other bugs which are hard to reproduce and fix reliably

Few programming languages implement threads by calling OS APIs called **1:1 model** (One os thread per one language thread)

Few programming languages provide own special implementation of threads called **green threads** called **M:N model** (M green threads per N os threads where M and N can be different)

- **Green threads** require larger runtime to manage threads.
- Rust standard library implements **1:1 model**.
- Few crates implement **M:N model** with overhead.

## Creating thread

- Create using `thread::spawn` and pass it a closure containing code to run in new thread

```rust
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
```

- Output of above code might vary based on how OS handles thread. Might also be different on each run.
- My output (which was same as outputs in docs) printed one from main, then one from spawned and one from main again and so on. THe main thread reached the number 5 but the spawned thread did not reach 10.
- After main thread stops, the spawned thread stops too.

## Waiting for all threads to finish using `join` handles

- To avoid spawned thread stopping early due to main thread or maybe never running at all we can use `JoinHandle`
- `thread::spawn` returns a value of type `JoinHandle`
- Owned value where if we call `join` method, it waits for thread to finish.

```rust
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
```

- In above code, spawned thread runs till the end.
- `join` *blocks* the main thread. Thread cannot perform work or exit
- Try moving `handle.join` above for loop in main thread.

```rust

let handle = thread::spawn(|| {
    //Note that this prints till the end due to use of JoinHandle
    for i in 1..10 {
        println!("hi number {} from spawned thread using JoinHandle", i);
        thread::sleep(Duration::from_millis(1));
    }
});

handle.join().unwrap();
for i in 1..5 {
    println!("Hi number {} from main thread", i);
    thread::sleep(Duration::from_millis(1));
};
```

- In above code spawned thread runs until completion first and only after that does the main thread run.
- This demonstrates how spawned thread is blocking the main thread.
- So details like where `join` is called can affect concurrency

## Using `move` Closures with Threads

- `move` closure lets you use data from one thread in another thread.
- `move` keyword before parameter lis of closure forces closure to take ownership of values in environment.
- Can be used to transfer ownership of values from one thread to another.

```rust
let v = vec![1,2,3];

let handle = thread::spawn(|| {
    println!("Vector : {:?}",v);
});
handle.join().unwrap();
```

- Above will not compile as closure may outlive current function but borrows v which is not owned by closure.
- Consider what would happen if we did `v.drop()` after spawning thread. It may get dropped before thread even runs. That is why rust does not compile.