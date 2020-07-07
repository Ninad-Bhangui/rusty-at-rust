# Hello World

## Links from docs covered here

I'm linking parts of the documentation which this particular code was created to understand.

- <https://doc.rust-lang.org/book/ch01-01-installation.html>
- <https://doc.rust-lang.org/book/ch01-03-hello-cargo.html>

### Installation (Linux)

Install the latest stable version of Rust using rustup
  `$ curl --proto '=https' --tlsv1.2 <https://sh.rustup.rs> -sSf | sh
  
Dillinger uses a number of open source projects to work properly:

### Basic Code

```
fn main() {
    println!("Hello, world!");
}
```

To run,

```
$ rustc main.rs
$ ./main
Hello, world!
```

#### Notes

- The main function is special: it is always the first code that runs in every executable Rust program.
- `println!` is a macro and not normal function. Macros to be explained later.

-

### Cargo

Rust built in package manager

Check : `cargo --version`

#### Create Project

`cargo new hello_world`

Following happens:

- `Cargo.toml` file created. (Like package.json)

```
[package]
name = "hello_worldc"
version = "0.1.0"
authors = ["nmb <b.ninad50@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

#### Build and Run

Inside project folder:

- `cargo build` - Compiles and creates executable
- `cargo run` - Compiles if not compiled before by build and runs the resulting executable. Running this again without any changes to the code will skip the build process
- `cargo check` - Checks code to make sure it compiles without creating executable. (Faster than build as it does not go through process of creating executable. Most people use check periodically and build when they're ready to use executable)

Above steps create executable in folder `target/debug`.
When ready for release, run `cargo build --release` which will create executable in folder `target/release`.
