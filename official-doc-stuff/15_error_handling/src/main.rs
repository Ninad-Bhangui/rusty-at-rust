use std::fs::File;
fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 3];
    // v[99]; //Run with RUST_BACKTRACE=1 for backtrace and panic

    let f: u32 = File::open("hello.txt");
}
