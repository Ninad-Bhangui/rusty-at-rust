pub fn median_attempt_1(li: &Vec<i32>) -> f32 {
    let mut li_sorted = li.clone();
    li_sorted.sort();
    let length = li_sorted.len();
    match length % 2 {
        0 => (li_sorted[length / 2 - 1] + li_sorted[length / 2]) as f32 / 2.0,
        _ => li_sorted[length / 2 + 1] as f32,
    }
}
