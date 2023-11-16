// methods are just functions defined within the context of a struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// everything under impl will be associated with Rectangle - associated functions
impl Rectangle {
    // & borrows the struct rather than take ownership of it, so that main() can continue using rect1
    // self must be the first parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    // this is often used for creating new instance of a struct
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// each struct can have multiple impl blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(3);

    // :? allows for an output format known as debug
    println!("rect1 is {:#?}", rect1);

    // dbg! macro points to the standard error console stream
    dbg!(&rect1);

    if rect1.width() {
        println!("The square has a nonzero width; it is {}", sq.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let mut r = Rectangle {
        width: 1,
        height: 2
    };
    let area1 = r.area();
    let area2 = Rectangle::area(&r);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    println!("r is {:#?}", rect1);
}