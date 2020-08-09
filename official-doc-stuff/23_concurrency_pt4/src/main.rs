use std::sync::Mutex;
use std::time::Duration;
fn main() {
    let data = Mutex::new(0);
    let d1 = data.lock();
    println!("Locked using d1");
    let d2 = data.lock().expect("Failed to lock"); // cannot lock, since d1 is still active. Creates deadlock
    println!("Locked using d2");
}
