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

## Explicit Drop

```rust
let c = CustomSmartPointer {
    data: String::from("new_string"),
};
drop_instance.drop();    //Not allowed.
drop(c); // std::mem::drop Allowed early drop
```

- We can’t disable the automatic insertion of drop when a value goes out of scope.
- We can't call `drop` explicitly.
- Calling `drop` explicitly is not allowed as rust would automatically call `drop` at the end of main. Leads to *double free error*
- to force it, we can use `std::mem::drop`

**Note** : I was not sure why `std::mem::drop` is different from calling `drop` method. On further research I found,

`drop` method does not take ownership.(`fn drop(&mut self) {`) It cannot really drop the value, it is only used to clean up stuff when it goes out of scope

`std::mem::drop` takes ownership. [Reference](https://doc.rust-lang.org/std/mem/fn.drop.html). Infact it is literally defined as,

```rust
pub fn drop<T>(_x: T) { }
```

- Because `_x` is moved here, it automatically dropped before the function returns.