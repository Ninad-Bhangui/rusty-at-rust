# Enum

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html>

## Define

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- Enum values can be only one of it's variants

```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
```

Another way of storing enums with string data.

The [standard library already has a definition for ip addresses](https://doc.rust-lang.org/std/net/enum.IpAddr.html).

They store it as below

```rust
struct Ipv4Addr {
}

struct Ipv6Addr {
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    fn call(&self) {

    }
}
```

- Any type of data (String/Struct/enum etc) can be put inside an enum variant.
- We can also define methods on structs using impl

### Option Enum

- `Option` type is used to handle scenarios where there's something or nothing.
- Rust does not have null type. (Because of numerous errors caused due to it's common implementation) whenever null is used in a non null context.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

- Option enum is frequently used and is included in prelude. So are each of it's variants.
- \<T\> is generic type. explained later

```rust
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
```

- Above are examples of using Option to hold different types
- Type needs to be specified for None as compiler can't infer the type.

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;    //Error
}
```

- Reason why Option is better than null is because Rust will not let `let sum = x + y;` happen. So we can be sure `i8` will never be null.
- Only when we use `Option` do we worry about nulls. And rust will make sure all variants are handled when using match.

### Match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Patterns with values

```rust
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

- Matches in rust are exhaustive. All possible scenarios must be handled
- `_ => ()` can be used as placeholder for matching any value(remaining)

### `if let`

- Can be used in situations where only one match needs to be handled.
- less code

##### `match`

```rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

##### `if let`

```rust
if let Some(3) = some_u8_value {
    println!("three");
}
```

- Can add `else` as well.
