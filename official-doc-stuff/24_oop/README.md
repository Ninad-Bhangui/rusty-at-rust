# Object Oriented Programming in rust

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html>

## Objects containing data and behavious

This is achieved by structs and enums with impl blocks.

## Encapsulation that Hides Implementation Details

This is achieved by structs and enums having many private methods and data, with public methods being explicitly written as such and exposed.

## Inheritance as a Type System and as Code Sharing

- Rust does not have exact inheritence where an object would inherit properties of another object.
- Rust has traits and trait objects to share implementation between objects. Although it's not inheritence, it kind of achieves the same purpose in a different way.

## Trait Objects

- Trait Objects can be created by specifying a pointer like `&` and `Box` and the `dyn` keyword.
- Can be used instead of Generics and concrete types.

For example,

Trait defined below

```rust
pub trait Draw {
    fn draw(&self);
}
```

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

`Box<dyn Draw>>` is a trait object below. It's a standin for any type implementing `Draw` trait.

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```

Let's say there was a method for screen,

```rust
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

- Above would take any type that implements `Draw`.
- One might wonder how is this different from Generic with trait bounds.
- In case of Generics, compiler would look at types being passed and create multiple implementations of same method for those types. Only one type could be used at a time. This would force the Vector to have all components of the same type. Something like below:

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

- `T` could be just one type at a time.

- This restriction does not apply for trait bounds which handles various types at runtime.
- We could now do something like below

```rust
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

- In above code, Vector has one `SelectBox` and another `Button`. This would not work with generics.

## Trait Objects Perform Dynamic Dispatch

- Unlike generics where monomorphization happens and compiler generates non generic functions from generic one, in trait objects what happens is *dynamic dispatch*
- Compiler emits code which at runtime would figure out method to call.
- Rust uses pointers to know which method to call. (Do not understand this clearly. Need more research)
- Has runtime performance overhead.
- Dynamic dispatch prevents rust from applying certain compiler optimizations

## Object Safety Is Required for Trait Objects

- Only *object safe* traits can be made into trait objects.
- Traits require the following properties:
     1. The return type isn't `self`
     2. There are no generic type parameters

I do not understand the reason right now. But it can be found [here](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects). Will update after I understand

## Implementing an Object-Oriented Design Pattern

- *State Pattern* is an object oriented design pattern.
- Value has an internal state and behaviour changes based on that state.
- State is responsible for it's own behaviour.
- value holding a state object knows nothing about behaviour.

Example , Implementing blog post workflow with following functionality:

- Blog post starts as empty draft.
- When draft is done, review is requested.
- After approval, gets published.
- Only published posts return content to print. Unapproved posts cannot be accidentally published.

