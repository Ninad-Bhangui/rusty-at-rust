pub fn add_two(a: i32) -> i32 {
    println!("Adding.."); //Only gets printed when failed runs
    a + 2
}

//Private function can be tested
fn check_num(value: i32) -> i32 {
    if value < 1 || value > 100 {
        panic!("Value must be between 1 and 100, got {}", value);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2))
    }
    #[test]
    #[ignore]
    fn it_does_not_work() {
        //Run using cargo test -- --ignored
        assert_eq!(4, add_two(1), "{} does not equal {}", 4, add_two(1));
    }
    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn out_of_bounds() {
        check_num(200);
    }
    #[test]
    fn result_return_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
