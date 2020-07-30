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

## Lifetime annotations in struct

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

## Lifetime Elision

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

- Above code works without lifetimes even though it is taking and returning references.
- Here, the compiler guesses lifetimes based on few rules called *lifetime elision* rules.
- *Input lifetimes* Lifetimes on funtion/method parameters.
- *Output Lifetimes* Lifetimes on return values
- Three rules are used to figure out lifetime references

    1. Each parameter that is a refernce gets it's own lifetime parameter
    2. If there is only one input lifetime parameter, that lifetime is assigned to all output parameters.
    3. If there are multiple input lifetime parameters but one of them is `&self` or `&mut self`, that lifetime is assigned to all output lifetime parameters

### First and Second rule Pass Example

```rust
fn first_word(s: &str) -> &str {
```

- In above function since there's only one input parameter, compiler will apply first and second rule and assign both `s` and the return value the same lifetime making it equivalent to below

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

### Second and Third rule Fail Example

```rust
fn longest(x: &str, y: &str) -> &str {
```

- Here first rule is applied and x and y get different lifetimes.
- Second rule fails as there are multiple lifetimes and return value is not assigned any lifetime
- Third rule fails as it is not a method and does not have self.
- SO return lifetime is unknown at the end and compiler will force you to specify lifetimes explicitly

## Lifetime Annotations in Method Definitions

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

- Above code first rule is enough and return type and compiler can assign lifetimes

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

- Above code has multiple parameters but because one of them is `&self`, third rule applies and `&str` return gets lifetime of self

## The Static Lifetime

```rust
let s: &'static str = "I have a static lifetime.";
```

- Can specify static lifetimes where refernce would live for entire duration of program

## Summary (Everything together)

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```