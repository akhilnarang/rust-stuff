#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }

    fn perimeter(&self) -> u32 {
        2 * (self.length + self.breadth)
    }
}

fn main() {
    let rect = Rectangle {
        length: 5,
        breadth: 4,
    };
    println!("Area is {}", rect.area());
    println!("Perimeter is {}", rect.perimeter());
}
