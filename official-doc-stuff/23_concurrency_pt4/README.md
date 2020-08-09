# Sync and Send Traits

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html>

## Send marker trait (`std::marker`)

- Ownership of type implementing `Send` can be transferred between threads.
- Almost every rust type is `Send` with few exceptions like `Rc<T>`
- Any type composed of `Send` types are automatically `Send`. For example a struct

```rust
struct user{
    name: String,
    age: i32
}
```

is `Send` automatically because all it's types are `Send`.

## Sync marker trait (`std::marker`)

- `Sync` types can be referenced from multiple threads
- `T` is sync if reference to it is `&T` is `Send`
- `Rc` and `RefCell` are not sync

## Implementing Send and Sync manually is unsafe

- Types made up of `Send` and `Sync` are thread safe by default.
- No implementation for either is needed.
- Implementing these requires unsafe rust code

## Extras

The tutorial asked us to do two things if we were interested. Look into atomic operations and try implementing deadlocks.

### Deadlock implementation

```rust
use std::sync::Mutex;
use std::time::Duration;
fn main() {
    let data = Mutex::new(0);
    let d1 = data.lock();
    println!("Locked using d1");
    let d2 = data.lock().expect("Failed to lock"); // cannot lock, since d1 is still active. Creates deadlock
    println!("Locked using d2");
}
```

- Above was a simple implementation of a deadlock I could come up with.
- Best strategy is to prevent deadlock itself.
- Mitigating deadlocks can be achieved through timeouts.
