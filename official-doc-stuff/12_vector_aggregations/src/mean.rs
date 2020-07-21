pub fn mean_basic(li: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for i in li {
        sum += i;
    }
    sum as f32 / li.len() as f32
}
