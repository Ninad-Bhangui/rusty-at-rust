<https://doc.rust-lang.org/std/collections/index.html>

### Use a Vec when

- Will only be appending to (or near) the end.
- Stack
- Heap allocated resizable Array

### VecDeque

- `Vec` supports insertion at both ends
- queue
- Double-ended Queue

### Vector Basics

####

```rust
v.len()
```

```rust
vec![0; 5] //=> vec!(0,0,0,0,0)
```

```rust
v.pop()
```

```rust
let mut vec1 = Vec::with_capacity(5);
vec1.resize(5, 0);
```

- Capacity is space reserved. If length exceeds capacity, elements are reallocated.
- Recomennded to specify capacity beforehand to improve performance and reduce reallocations.

```rust
let mut vec = vec![1, 2, 3];
vec.insert(1, 4);
```

```rust
let mut vec = vec![1, 4, 2, 3];
vec.remove(1);
```

- Wondering why remove returns item/panics if not exists and not Options enum like pop

### Guarantee

- Vector always will be  (pointer, capacity, length) triplet
- Vec will never perform a "small optimization" where elements are actually stored on the stack
- Vec will never shrink itself even when empty. Emptying and filling up a vec has no calls to allocator. `shrink_to_fit` can be used to explicitly free up space by shrinking.
