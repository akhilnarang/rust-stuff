fn main() {
    let x = std::f64::consts::PI;
    if x > 5.0 {
        println!("x is greater than 5.0")
    } else {
        println!("x is less than 5.0")
    }
    let a = if x == std::f64::consts::PI {
        "pi"
    } else {
        "not pi"
    };
    println!("{}", a);
}
