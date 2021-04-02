fn main() {
    let x = Some(5);
    let y = Some("Hello World");
    let z: Option<isize> = None;
    println!("{:?}, {:?}, {:?}", x, y, z);
}
