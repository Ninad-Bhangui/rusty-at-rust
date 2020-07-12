# Ownership

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand

- <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>

## Memory Management

- Some programming languages have garbage collection. (python)
- Some require you to explicitly allocate and de-allocate memory. (C)
- Rust achieves this through ownership. This is achieved at compile time and so do not slow down the application.

## Stack and Heap

Brief explanation [here](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap).

### Stack

- last in first out
- data on stack must have fixed size known at compile time

### Heap

- Data with unknown size at compile time or size that may change
- os allocates spot in heap and returns pointer
- pointer stored on stack
- Slower access

When function is called,  values passed to function and local variables pushed on stack. After it's done they're popped from the stack.

## Variable Scope

```rust
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
```

- When s comes into scope, it is valid.
- It remains valid until it goes out of scope.

## The `String` Type

```rust
let a = "hello" //string literal
let b = String::from("hello") //string
```

- String literals are stored on stack
- String is stored on heap and is mutable as a result

Rust returns memory taken after variable goes out of scope. It calls `drop` function which returns the memory.

## Move

```rust
fn main() {
    let x = 5;
    let y = x;
    println!("{}",x);  //Works. Both are valid.

    let s1 = String::from("hello");
    let s2=s1;
    println!("{}",s1);   //Error. s1 is no longer valid
}
```

- Variables are copied when their values are stored on stack.
- The pointer is *moved* in case of heap.

![](https://doc.rust-lang.org/book/img/trpl04-04.svg)

s1 is no longer valid as shown above. If both pointers were kept, and if each tried to deallocate a *double free* error could occur

- You will have to explicitly clone using `let s2 = s1.clone();
` if you want to avoid *move* which will copy heap data and both s1 and s2 would remain valid.
- In case of assignment with stack values, Rust has a special annotation called the `Copy` trait. Anything with `Copy` trait is usable after assignment. You cannot annotate a type with `Copy` if it implements `Drop`.
- `Copy` is in `u32`,`f64`,`bool`,`char` etc

## Ownership Functions

Similar to assignment, move occus on function call for heap variable.

```rust
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    println!("{}",s);   //Error
```

- Returning values also transfer scope.
- It can be tedios if every function has to return the output and the original function if the original value needs to be used again later.

## References and Borrowing

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);    //passes reference

    println!("The length of '{}' is {}.", s1, len); //s1 can be used here
}

fn calculate_length(s: &String) -> usize {
    s.len() //Does not have ownership so not dropped here
}
```

References are immutable by default. Add `&mut` to change that.

### Mutable reference rules

- Two *simultanous* mutable references to the same piece of data is not allowed.
- Two *simultanous* immutable references to same data is allowed.
- One immutable and one mutable *simultanous* reference is not allowed. Allowing this would make the immutable reference useless as the mutable reference could change the value.
- However, if an immutable reference stops being used, a mutable reference can be used later ie., they are not simultaneously being used.
