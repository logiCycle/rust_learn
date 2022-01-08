#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hole(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.length >= other.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }

    fn new(width: u32, length: u32) -> Rectangle {
        Rectangle { width, length }
    }
}
fn main() {
    let rectangle = Rectangle {
        width: 30,
        length: 20,
    };
    let rec2 = Rectangle::new(20, 10);
    println!("rectangle1: {:?}, area is {}", rectangle, rectangle.area());
    println!("rectangle2: {:#?}, area is {}", rec2, rec2.area());
    println!("rec1 can hold rec2 ? {}", rectangle.can_hole(&rec2));
    let square = Rectangle::square(40);

    println!("square: {:?} area is {}",square, square.area());
}
