fn main() {
    println!("Printing from main");
    another_function();
    one_int_param(5);
    more_params(3.14, false);
}

fn another_function() {
    println!("Printing from another function!");
}

fn one_int_param(n: isize) {
    println!("The value of n is {}", n);
}

fn more_params(m: f64, n: bool) {
    println!("{} and {}", m, n);
}
