#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rec2: &Rectangle) -> bool {
        self.width > rec2.width && self.height > rec2.height
    }
    fn zero() -> Self {
        Self {
            width: 0,
            height: 0
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 45
    };

    let rect3 = Rectangle::zero();

    println!("The area of the rectangle is {} square pixels",
             rect1.area());
    println!("Rect {:#?}", rect1);
    dbg!(&rect1);
    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    dbg!(&rect3);
}
