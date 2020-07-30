use testing_practice;
mod common;
#[test]
fn it_adds_two() {
    //Run with  cargo test -- --show-output to see that setup gets called
    common::setup();
    assert_eq!(4, testing_practice::add_two(2));
}
