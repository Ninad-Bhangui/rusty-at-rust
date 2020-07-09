# Functions/Comments/Control Flow

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch03-03-how-functions-work.html>
- <https://doc.rust-lang.org/book/ch03-04-comments.html>
- <https://doc.rust-lang.org/book/ch03-05-control-flow.html>

### Functions

```rust
fn main() {
    another_function(5, 6);
}
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

Above seems self explanatory for what functions are.

- you must declare the type of each parameter.
- Functions contain statements and expressions
- Statements don't return value `let a = 5;`
- Expressions return values. Dont end with semicolon. `let a = b+1`, `b+1` is expression.

### Return in Function

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Can return as shown above. Or like in normal languages use `return` keyword.

### Comments

```rust
//Single Line

//For
//Multi
//Line
//Repeat
//single
//again
//and
//again
//Till you are left soulless

//
```

- Block comments exist but based on my understanding so far are reserved for docstrings.

### Condition

```rust
let x = 1
if x < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}
```

Above is Valid

```rust
if x {
    println!("Non boolean condition does not work ");
}
```

Above is invalid. Unlike python where non zero values evaluate to True, it expects boolean here.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

- Return type of if and else must be same in this case.
- Cannot be `if condition { 5 } else { "six" };`

### Loop

#### Using `loop`

- Check Basic loop in guessing game. [Chapter 1](../1_guessing_game/)

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

Return value from loop after break.

#### Using `while`

```rust
while number != 0 {
    println!("{}!", number);

    number -= 1;
}
```

Self explanatory

#### Using `for`

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

Self explanatory

```Rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number); //prints 1 2 3 4
    }
    println!("LIFTOFF!!!");
}
```

Self explanatory
