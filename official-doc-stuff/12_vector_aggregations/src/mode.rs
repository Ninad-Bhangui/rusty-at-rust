use std::collections::HashMap;
pub fn mode_attempt_1(li: &Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for i in li {
        let mut count = counter.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut max_key = li[0];
    for (key, value) in &counter {
        if *value > max {
            max = *value;
            max_key = *key;
        }
    }
    max_key
}
