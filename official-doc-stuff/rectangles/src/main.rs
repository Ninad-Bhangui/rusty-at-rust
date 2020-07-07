#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height*self.width
    }
    fn diamter(&self) -> u32 {
        (self.height + self.width)*2
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main() {

    let rectangle = Rectangle {
        height: 50,
        width: 30
    };
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let square = Rectangle::square(4);

    println!("rectangle : {:?}",rectangle);
    println!("Area of rectangle is : {}",rectangle.area());
    println!("Diameter of rectangle is : {}",rectangle.diamter());
    println!("Diameter of rectangle is : {}",rectangle.can_hold(&rect2));
    println!("Diameter of rectangle is : {}",rectangle.can_hold(&rect3));

    println!("square : {:?}",square);
}