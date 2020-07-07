# Guessing game

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html>

### Stuff learnt

#### Use library

```rust
use std::io;
```

The io library comes from the standard library (which is known as std). Rust brings only a few types into the scope of every program in the [prelude](https://doc.rust-lang.org/std/prelude/index.html) .
(Prelude mentioned. Should explore in detail)
If type is not in prelude should be used as above.

##### Prelude

The prelude is the list of things that Rust automatically imports into every Rust program. It's kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.

#### Initializing variables

```rust
 let mut guess = String::new();
```

- let creates variable
- **Variables are immutable by default**
- mut keyword makes them mutable
- `String::new` returns new instance of string
- `::` indiciates associated function. Associated functions are implemented on types and not instances of types.

#### Input from user

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

- The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal.
- `read_line` is a method on instance of stdin(). Note the difference between associated function mentioned before `String::new`. In this case it is implemented on instance of a type.
- `read_line` accepts a mutable reference to store user input.
- `&` indicates it is a reference. Reference lets multiple parts of code access one piece of data without copying in memory. References are immutable by default.
- `&mut guess` is used to make it a mutable reference.
- `read_line` returns an io::Result which is an enum. Enums are types with fixed set of values called variants. For result( `Ok`, `Err`)
- `expect` is a method on instance of `io::Result` type. If instance is `Err` it displays error message and crashes else returns value that `Ok` is holding (User input value).

#### Generate secret number

```rust
let secret_number = rand::thread_rng().gen_range(1, 101);
```

- `rand::thread_rng` returns a generator for current thread, gen_range generates it between lower(inclusive) and upper(exclusive) bounds.

#### Convert type

```rust
let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```

- Rust has strong static [typed system](https://en.wikipedia.org/wiki/Type_system#Static_typing). It also has [type inference](https://en.wikipedia.org/wiki/Type_inference).
- So the compiler will throw an error as generated number is integer and guess is string.
- Above code is what's called shadowing a variable. Lets us reuse variable name.
- parse : parses string into number. `guess: u32` is annotating the variable type, So now rust knows which exact numeric type to parse to.
- parse returns a Result type like `read_line` does.
- Could have used expect but in this case the loop repeats execution and asks for user input again.

#### Compare

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

- `use std::cmp::Ordering` : Another enum with variants `Less`,`Greater`,`Equal`
- `cmp` method compares and returns variant of Ordering enum..
- `match` expression - kind of like switch on steroids. Contains arms. Each arm contains pattern+code.
