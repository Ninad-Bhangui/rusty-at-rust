# Reference Cycles

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch15-06-reference-cycles.html>

## Intro

- Memory leaks rare but possible.
- Rust guarantees no data race, so the memory leaks are memory safe.
- Items may refer to each other in a cycle and reference count would never reaceh 0 and value would not be dropped.
