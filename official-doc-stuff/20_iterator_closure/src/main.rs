fn main() {
    let x = 4;

    let equal_to_x = |z| z == x; //Closure using x works which is not in function scope

    let y = 4;
    println!("{}", x);
    assert!(equal_to_x(y));
    assert!(equal_to_x(y));


    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    //Produces only 4 pairs as last one would be (5,None) and zip returns None if any input iterator returns None.
    let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
}
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count +=1;
            Some(self.count)
        }
        else {
            None
        }
    }
}