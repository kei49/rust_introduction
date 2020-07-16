#[derive(Debug)]
struct Rectangle {
    name: String,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn status(&self) {
        println!("{} is {:#?}", self.name, self);
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(name: String, size: u32) -> Rectangle {
        Rectangle { name, width: size, height: size }
    }
}

fn main() {
    println!("Let's calculate the area of rectangle!");
    let rect1 = Rectangle{ name: String::from("rect1"), width: 30, height: 40 };
    let rect2 = Rectangle{ name: String::from("rect2"), width: 20, height: 15 };
    let rect3 = Rectangle{ name: String::from("rect3"), width: 500, height: 30 };
    rect1.status();
    rect2.status();
    rect3.status();
    println!("The area of {} is {} square pixels.", rect1.name, area(&rect1));
    println!("The area of {} is {} square pixels.", rect2.name, rect2.area());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(String::from("square"), 5);
    sq.status()
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
