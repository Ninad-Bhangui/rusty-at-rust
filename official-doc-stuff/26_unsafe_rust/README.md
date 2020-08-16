# Unsafe rust

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch18-00-patterns.html>

## Unsafe Rust

- underlying computer hardware is inherently unsafe.
- unsafe lets you break rules but explicitly so finding an issue is easier as you know where unsafe blocks are

## Unsafe Superpowers

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

## Dereferencing a Raw Pointer

- compiler ensures references are always valid.
- Unsafe Rust has two new types called raw pointers that are similar to references.
- Raw pointers can be immutable or mutable and are written as `*const T` and `*mut T`
- In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced

Differences between references and smart pointers:

- can have both immutable and mutable pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

- Creating does not require unsafe block. Only dereferencing does.

```rust
let address = 0x012345usize;
let r = address as *const i32;
```
