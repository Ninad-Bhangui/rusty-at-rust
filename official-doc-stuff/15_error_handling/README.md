# Error Handling

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch09-00-error-handling.html>

## Intro

Errors are of two types:

- Recoverable (You want to handle these) and Non recoverable (You want to panic and abort)
- Rust uses `Result<T, E>` for recoverable errors and the `panic!` macro for non recoverable

## Unrecoverable

```rust
fn main() {
    panic!("crash and burn");
}
```

- By default program `unwinds` on panic.
- `unwinding` means to walk back up stack and clean data from each function.
- Add `panic = 'abort'` to `[profile]` section in *Cargo.toml* to avoid this and make the resulting binary smaller. OS will clean it up later.

```bash
RUST_BACKTRACE=1 cargo run
```

- Above let's you print *BACKTRACE* which is list of functions called to reach the `panic`.
- Used when `panic` occurs in a function you are calling (core rust/crates/standard library etc).

## Recoverable

Example: Reading a file that does not exist. You can create a file instead of panicking.

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Above is Result Enum.

```rust
File::open("hello.txt")
```

The above line returns enum:
 `std::result::Result<std::fs::File, std::io::Error>`

## Different ways to handle file open error

### Use `match`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

### Use nested `match`. Handle file not found by creating the file and `panic` for other errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

- `Error` struct has a method `kind` which returns enum `io::ErrorKind`.
- Handle variant for *File not found* by creating the file. Panic at the other variants.
- Also handle file creation using match.

### Using `Result` method `unwrap` or `unwrap_or_else`

```rust
let f = File::open("hello.txt").unwrap();
```

- From my understanding, `unwrap` converts Option/Result success variant to original value and panics for error variant.

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

- `unwrap_or_else` let's us pass a closure to handle error variant.

### Using `expect`

```rust
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```

- simple `unwrap` does not let us customize the error message. `unwrap_or_else` is more complicated for just passing custom error message.
- `expect` is same as `unwrap` but let's us pass custom error message.

## Propagating errors

If error occurs in a function, instead of handling there, you might want to let the calling code decide how to handle it.

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

`Result<String, io::Error>` is returned by the function. Calling code will decide how to handle

### Using `?` Operator

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

- Shorter way of writing the previous snippet. `?` does a similar job as the `match` defined above.
- `?` uses `from` trait which converts error type into the error type specified in return value

```rust
File::open("hello.txt")?.read_to_string(&mut s)?;
```

- Above is Even shorter
- `?` can be used in functions that return `Result`.
- Can be used in main function only if main returns `Result`

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

- For now `Box<dyn Error>` can be understood as error of any type before more details are given later

## Panic or not

- Most cases returning `Result` is better as the calling code can decide how to handle it.

### Using Panic

- In Examples, Prototypes and Tests, panic is used more.
- If we have more information than compiler. For example if a function worked with hardcoded values, we would know it would never panic. But in case of user input values we should handle it.
- Unexpected Failures.

### Handling Panic

- Expected failures

## Custom  Validation

- Rust type system handles many situations allowing cleaner code and lack of repeated error checks.
- One can make custom validations to emulate this behaviour for other conditions than type.

For example in guessing game problem, one can use a struct with methods to validate value lying between 1-100 instead of using `u32`

```rust

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
