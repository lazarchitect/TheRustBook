#[derive(Debug)]

struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {

    fn area (self: &Rectangle) -> u32 {
        self.width * self.height
    }

}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 area is {:#?}", rect1.area());
}
