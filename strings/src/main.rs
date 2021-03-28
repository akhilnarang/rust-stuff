fn main() {
    let mut x = String::from("Hello");
    x.push(' ');
    x.push_str("world");
    println!("{}", x);
    let y = x.clone();
    println!("{}", y);
    println!("{}", x);
}
