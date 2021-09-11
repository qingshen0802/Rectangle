
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // These are methods of struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is associated function of struct
    fn square(size: u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size,
        }
    }
}

fn main() {
    normal();
    tuple();

    use_struct();
    use_struct_method();

    can_hold();
}

fn normal() {
    let x = 50;
    let y = 30;

    println!("The area of rect is {} square pixels.", area(x, y));
}

fn tuple() {
    let dimension = (20, 30);
    println!("The area of rect is {} square pixels.", area_tuple(dimension));
}

fn use_struct() {
    let rect = Rectangle {
        width: 60,
        height: 30,
    };

    println!("The area of rectangle is {} square pixel.", area_struct(&rect));
}

fn use_struct_method() {
    let rect = Rectangle {
        width: 60,
        height: 40,
    };

    println!("The area of rectangle is {} square pixel.", rect.area());
}

fn can_hold() {
    let rect1 = Rectangle {
        width: 80,
        height: 60,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 90,
        height: 70,
    };

    let square = Rectangle::square(50);

    println!("Can Rect 1 hold the Rect 2? {}", rect1.can_hold(&rect2));
    println!("Can Rect 1 hold the Rect 3? {}", rect1.can_hold(&rect3));
    println!("Can Rect 1 hold the Square? {}", rect1.can_hold(&square));
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
