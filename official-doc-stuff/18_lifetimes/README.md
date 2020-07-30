# Generic Types, Traits and Lifetimes

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html>

## Intro

- Lifetime is scope for which a reference is valid
- Usually implicit and inferred
- Can annotate lifetimes to be explicit

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

- Above code would give an error on compiling that `x` does not live long enough.
- Compiler knows this by using borrow checking

## Borrow Checker

```rust
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }
```

- Lifetime of r `'a` is larger than lifetime of x `'b`. So the code will not compile as the *subject of the reference doesn’t live as long as the reference*.

```rust
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
```

- Above code will compile

## Generic Lifetimes in Function

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- Above will not compile with error expected lifetime parameter.
- It returns borrowed value and we don't know which one (x or y) and cannot predict if the reference returned will always be valid
- Need to specify lifetimes

## Annotating Lifetimes

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

- Annotations do not change lifetimes
- They just describe how lifetimes of multiple references are related

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- Above works
- Above specifies that both x and y parameters and return value should have the same lifetime `'a`
- We are not changing lifetimes but rather specifying that the borrow checker should reject any values that don’t adhere to these constraints.
- Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

- Above works as well as `string2` has the smaller of the lifetimes, result also exists in this smaller lifetime and will never point to something that goes out of scope sooner

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

- Above will not compile as there is a chance if string2 was larger, then result would point to it while having a larger lifetime
