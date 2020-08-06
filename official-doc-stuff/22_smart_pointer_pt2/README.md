# Smart Pointers

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch15-03-drop.html>

## Running Code on Cleanup with the Drop Trait

- `Drop` lets you customize what happens when value goes out of scope.
- In many programming languages, programmer must call code to free memory.
- In rust you can specify a bit of code that must be run whenever value goes out of scope. (`Drop` Trait)

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping instance of CustomSmartPointer : {}", self.data);
    }
}
```
