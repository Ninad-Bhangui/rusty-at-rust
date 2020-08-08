# Shared state

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch16-03-shared-state.html>

## Shared-State Concurrency

- In channels, once you pass a value down the channel, you should no longer use that value.
- Shared memory concurrency is like multiple ownership.
- Multiple threads access same location at same time.

## Using Mutex to allow access to data one thread at a time

- Mutex - *Mutual Exclusion*. One thread accesses some data at any given time.
- Thread must first signal that it wants access by asking for mutex lock.
- Lock is a data structure that keeps track of who has access to data.

Mutexes can be difficult for the following reasons:

- You must attempt to acquire lock before using the data.
- When done, you must unlock data so other threads can acquire the lock.
- Rust type system and ownership rules make locking and unlocking less error prone.

```rust
let m = Mutex::new(5);
{
    let mut num = m.lock().unwrap();
    *num = 6;
    //Note that this prints Mutex {data : <locked>}
    println!("m = {:?}",m);
}

println!("m = {:?}",m);
```

- To access data in `Mutex<T>` we must use `lock` method to acquire lock.
- Returns `Result` which returns error if another thread holding the lock panicked.
- Type system ensures we have a lock before using value in `m`.
- `Mutex<T>` is a smart pointer.
- Call to lock returns a smart pointer called `MutexGuard`, wrapped in `LockResult` handled using unwrap.
- `MutexGuard` implements `Deref` to point to inner data. 
- It also has `Drop` implementation that releases lock when `MutexGuard` goes out of scope.
- So we cannot forget to release the lock and block mutex from being used by other threads.

## Sharing Mutex\<T\> between threads

```rust
let counter = Mutex::new(0);
let mut handles = vec![];

for _ in 0..10 {
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

- Above will not compile.
- counter is moved in first iteration and cannot be used by multiple threads.
- Can try using `Rc` but that would not work either.
- `Rc` is not meant for concurrent situations. It does not handle managing references in thread safe way.
- Another thread may interrupt changes to count.

## Atomic Reference Counting - Arc\<T\>

- Like `Rc<T>` but is safe in concurrent situations