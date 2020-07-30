struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl Point<i32> {
    fn distance_from_origin(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }
}
fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    println!("Distance from origin : {}", p.distance_from_origin());

    let p1 = Point { x: 2.5, y: 3.5 };
    println!("Distance from origin : {}", p1.distance_from_origin());
}
