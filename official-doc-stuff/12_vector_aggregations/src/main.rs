fn main() {
    println!("Hello world!")
}

//Have implemented each function in varios ways in a file with same name like mean.Result
// Use aliases while trying out each function

mod mean;
mod median;
mod mode;
use mean::mean_basic as mean;
use median::median;
use mode::mode;

//const test1: Vec<i32> = vec![13, 18, 13, 14, 13, 16, 14, 21, 13];
/*
mean: 15
median: 14
mode: 13
range: 8
*/

//const test2: Vec<i32> = vec![1, 2, 4, 7];
/*
mean: 3.5
median: 3
mode: none
range: 6
*/

//const test3: Vec<i32> = vec![8, 9, 10, 10, 10, 11, 11, 11, 12, 13];
/*
mean: 10.5
median: 10.5
modes: 10 and 11
range: 5
*/
#[test]
fn test_mean() {
    let test1: Vec<i32> = vec![13, 18, 13, 14, 13, 16, 14, 21, 13];
    let test2: Vec<i32> = vec![1, 2, 4, 7];
    let test3: Vec<i32> = vec![8, 9, 10, 10, 10, 11, 11, 11, 12, 13];
    assert_eq!(15.0, mean(&test1));
    assert_eq!(3.5, mean(&test2));
    assert_eq!(10.5, mean(&test3));
}
#[test]
fn test_median() {
    let test1: Vec<i32> = vec![13, 18, 13, 14, 13, 16, 14, 21, 13];
    let test2: Vec<i32> = vec![1, 2, 4, 7];
    let test3: Vec<i32> = vec![8, 9, 10, 10, 10, 11, 11, 11, 12, 13];
    assert_eq!(45, median(&test1));
    assert_eq!(45, median(&test2));
    assert_eq!(45, median(&test3));
}

#[test]
fn test_mode() {
    let test1: Vec<i32> = vec![13, 18, 13, 14, 13, 16, 14, 21, 13];
    let test2: Vec<i32> = vec![1, 2, 4, 7];
    let test3: Vec<i32> = vec![8, 9, 10, 10, 10, 11, 11, 11, 12, 13];
    assert_eq!(45, mode(&test1));
    assert_eq!(45, mode(&test2));
    assert_eq!(45, mode(&test3));
}
