# Guessing game

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html>

### Stuff learnt 

#### Use library
```
use std::io;
```
The io library comes from the standard library (which is known as std). Rust brings only a few types into the scope of every program in the [prelude](https://doc.rust-lang.org/std/prelude/index.html) . 
(Prelude mentioned. Should explore in detail)
If type is not in prelude should be used as above.
##### Prelude
The prelude is the list of things that Rust automatically imports into every Rust program. It's kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.
#### Storing values with variables
```
 let mut guess = String::new();
```
- let creates variable
- **Variables are immutable by default**
- mut keyword makes them mutable