use std::collections::HashMap;
use std::io;
fn main() {
    println!("Hello, world!");
    let mut v = Vec::new();
    let mut len_arr = String::new();
    println!("Enter length of Array");
    io::stdin().read_line(&mut len_arr).expect("Could not read line");
    let len_arr: u32 = len_arr.trim().parse().expect("Please type number");
    for i in 0..len_arr{
        let mut num = String::new();
        println!("Enter {} number",i);
        io::stdin().read_line(&mut num).expect("Could not read line");
        let num: u32 = num.trim().parse().expect("Please type number");
        v.push(num);
    }
    v.sort();
    println!("Final sorted array is : {:?}",v);
    println!("Average is : {}",mean(&v));
    println!("Median is : {}",median(&v));
    println!("Mode is : {}",mode(&v));
}
//Functions I had written on first attempt
// fn mean(v : &Vec<u32>) -> f32{
//     let l : f32 = v.len() as f32;
//     let mut avg = 0;
//     for i in v {
//         avg+=i;
//     }
//     return avg as f32/l;
// }
// fn median(v : &Vec<u32>) -> u32{
//     let l : u32 = v.len() as u32;
//     if l %2 == 0 {
//         let midpoint = l/2;
//         return (v[(midpoint-1) as usize]+v[midpoint as usize])/2
//     }
//     else{
//         return v[(l/2) as usize]
//     }
// }
// fn mode(v : &Vec<u32>) -> u32{
//     let mut count_map = HashMap::new();
//     for i in v {
//         let count = count_map.entry(i).or_insert(0);
//         *count+=1;
//     }
//     let mut max = 0;
//     for (&&k,&v) in &count_map {
//         if v>max{
//             max = v;
//         }
//     }
//     for (&&k,&v) in &count_map {
//         if v==max {
//             return k;
//         }
//     }
//     return max;
// }

//Function found online
fn mean(list: &[u32]) -> f32 {
    //One solution below
    // let l : f32 = list.len() as f32;
    // let sum: u32 = Iterator::sum(list.iter());
    // sum as f32/l

    //Even better solution below
    list.iter().sum::<u32>() as f32 / list.len() as f32
}

fn median(list: &[u32]) -> f32 {
    let l = list.len();
    let mid = l / 2;
    if l%2 ==0 {
        mean(&list[(mid-1)..(mid+1)])
    }
    else{
        list[mid] as f32
    }
}
fn mode(list: &[u32]) -> u32 {
        let mut occurrences : HashMap<u32, u32> = HashMap::new();
        for &i in list {
            *occurrences.entry(i).or_insert(0)+=1;
        }
        //One solution
        let mut max = (0,0);
        // for (&k,&v) in &occurrences {
        //     if v>max.1{
        //         max = (k,v);
        //     }
        // }
        // return max.0;
        //Better solution
        occurrences.into_iter().max_by_key(|&(_, count) | count).0
}