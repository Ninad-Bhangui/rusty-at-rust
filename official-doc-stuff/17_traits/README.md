# Generic Types, Traits and Lifetimes

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch10-02-traits.html>

## Intro

- A trait tells the Rust compiler about functionality a particular type has and can share with other types.
- Define shared behavior in an abstract way.
- Traits are similar to a feature often called interfaces in other languages (Which I do not know of)

## Defining Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

- Declare Trait as shown above.
- Method is put without the implementation.
- Each type implementing above trait must provide it's own implementation
- Compiler will ensure implementation exists for all type implementing the trait
- Multiple methods allowed

## Implementing a Trait on a Type

```rust

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//Implementing Summary for type NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
//Implementing Summary for type Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Calling the method from trait

```rust
tweet.summarize()
```

- We can implement trait on a type only if either trait or type is local to our library
- For example we can implement `Display` Trait on `Tweet` as `Tweet` is local
- We can implement `Summary` trait on `Vec` as `Summary` is local.
- We cannot implement `Display` on `Vec` as neither are local but part of standard library.
- This property is called *Coherence* or *Orphan Rule*
- Ensures others can't break your code.

## Default Implementation

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

- Implementation can be specified in the trait itself
- Specific type implementation will override the default

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

- Default implementations can call other methods in same trait even if they dont have default implementations
- Cannot call default implementation from overriding implementation of same method. For example, summarize of Tweet which has overriden default `summarize` cannot call the default `summarize` function

## Traits as Parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

- Any type implementing `Summary` Trait can be passed as a parameter.
- Any trait methods can be called inside this function

## Trait Bound Syntax

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

- Above is the actual syntax for traits as parameters, while the previous one was syntax sugar for this

- `pub fn notify(item1: &impl Summary, item2: &impl Summary) {` can take different types
- `pub fn notify<T: Summary>(item1: &T, item2: &T) {` forces the same type. (This cannot be achieved with syntax sugar version above)

## Multiple Traits

We can specify more than one trait which means the item would have to implement both traits to be eligible

```rust
pub fn notify(item: &(impl Summary + Display)) {
```

```rust
pub fn notify<T: Summary + Display>(item: &T) {
```

- item should implement both Summary and Display to be used in notify

## Clearer Trait Bounds with where Clauses

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

- Cleaner way to represent multiple traits

## Returning Types that Implement Traits

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

- Ability to return Traits are useful in closures and iterators. We just have to write our function to return something that implements iterator instead of bothering about some weird iterator type

- The same function that returns Traits cannot return different types.
- Cannot return `Tweet` or `NewsArticle` conditionally in above snippet
