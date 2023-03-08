#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> u32 {
        self.height > other.height && self.width > other.width
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 30,
        width: 50
    };

    println!(
        "The area of the Rectangle is {} square pixels.",
        area(&rect)
    );

    println!(
        "The area of the Rectangle using own fn is {} square pixels.",
        rect.area()
    );

    println!("Rectangle => {:?}", rect);
    println!("Pretty rectangle => {:#?}", rect);

    dbg!("testing", rect);

    let scale = 2;
    let scaled_rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&scaled_rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
