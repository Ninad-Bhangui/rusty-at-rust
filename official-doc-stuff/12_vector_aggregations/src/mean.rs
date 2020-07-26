pub fn mean_attempt_1(li: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for i in li {
        sum += i;
    }
    sum as f32 / li.len() as f32
}
pub fn mean_attempt_2(li: &Vec<i32>) -> f32 {
    return li.iter().sum::<i32>() as f32 / li.len() as f32;
}
