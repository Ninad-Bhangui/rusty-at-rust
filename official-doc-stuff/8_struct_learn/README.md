# Struct

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch05-01-defining-structs.html>
- <https://doc.rust-lang.org/book/ch05-02-example-structs.html>

## Intro

Structure to group similar data with named properties. Sounds like class to me.
I intend to read following links to understand why struct was used instead of class

- <https://smallcultfollowing.com/babysteps/blog/2015/05/05/where-rusts-enum-shines/>
- <https://users.rust-lang.org/t/why-not-just-add-classes/4618>
- <https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html>

## Definition and Instantiating

### Definition

```rust
struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}
```

### Instance

#### Immutable

```rust
let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
```

#### Mutable

```rust
let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
user1.email = String::from("anotheremail@example.com");
```

- Entire sturct has to be mutable not just few properties

### Ownership

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
```

- Above code will not compile
- Struct should have ownership of it's properties. We do not want the string to go out of scope before struct. The reference to the string would cause issues
- Above can be done using lifetimes (Explained later)

### Function to build instance

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username,  //Can use this instead of above as shorthand if variable name is same as property name
        active: true,
        sign_in_count: 1,
    }
}

```

### Struct update

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### Tuple struct

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

- No named property required
- Difference from a tuple would be in a function which expects Color but Point is sent. It would throw an error which would not be the case for generic tuples.

### Unit-Like Structs

- Structs without fields.
- Used for implementing traits.
- Explained more later. I do not understand this yet.

### Rectangle struct

Later an example of rectangle struct is explained in docs which are not much different from User struct here.

### Derived Traits

To print a struct using `println!()` it needs to implement `std::fmt::Display`. Most types implement this but for structs due to their ambiguity do not.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

Above 
