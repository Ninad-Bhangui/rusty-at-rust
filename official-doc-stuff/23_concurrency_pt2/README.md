# Message passing

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch16-02-message-passing.html>

## Channels

- *Message passing* - Threads communicate by sending each other messages containing data. Helps in ensuring safe concurrency.
- Go lang Documentation states - "Do not communicate by sharing memory; instead, share memory by communicating."
- One tool rust has for message passing is *channel* (A general programming concept).
- Channels have two halves: transmitter and receiver.
- Transmitter is upstream location
- Receiver is downstream.
- One part of code calls methods on transmitter with data to send.
- Other part checks receiver for arriving messages.
- If either half is dropped, channel is said to be closed.

```rust
let (tx,rx) = mpsc::channel();
```

- `mpsc` - multiple producer, single consumer.
- Cannel can have multiple sending ends but only one receiving end.
- `mpsc::channel()` returns transmitter,reciever tuple


```rust
let (tx,rx) = mpsc::channel();

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    //Below line gives error. val borrowed after move
    // println!("val is {}", val);
});
let receieved = rx.recv().unwrap();
println!("Got : {}", receieved);
```

- Spawned thread needs to own transmitter to send messages.
- Returns `Result<T,E>` type so if reciever end has dropped, send returns Error.
- Received has two methods `recv` and `try_recv`. 
- `recv` blocks main threads execution waits for value.
- Once transmitter closes, `recv` returns error.
- `try_recv` does not block but returns `Result<T, E>` immediately.
- `Ok` if message available else `Err`.
- Useful to avoid blocking thread to do other work.
- Could write a loop that calls `try_recv` often, handles if message exists else do other work.
- Try printing variable after sending from transmitter. Ownership rules would prevent that

## Sending multiple values and seeing receiver waiting

```rust
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
    for recieved in rx {
        println!("Got : {}", recieved);
    }
```

- Here `recv` is used as itertor to get multiple values from `tx`

## Multiple producers by cloning transmitter

```rust
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
```

- `tx1` is cloned from `tx`. Multiple producers and one consumer implemented above.
- Order can be different based on OS.