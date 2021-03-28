use std::usize;

fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);
    println!("x is {}", x);
    // println!("s is {}", s);

    let y = gives_ownership();
    println!("y is {}", y);

    let z = takes_and_gives_ownership(y);
    println!("z is {}", z);

    println!("{}, {}", x, z);

    let (a, b) = calculate_length(z);

    println!("{} has length {}", a, b);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    String::from("Test string")
}

fn takes_and_gives_ownership(input: String) -> String {
    input
}

fn calculate_length(input: String) -> (String, usize) {
    (input.clone(), input.len())
}
