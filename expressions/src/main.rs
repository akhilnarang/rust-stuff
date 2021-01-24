fn main() {
    println!("Value is {}", with_variable());
    println!("Value is {}", without_variable());
    println!("Value is {}", test_return());
}

fn with_variable() -> isize {
    let x = {
        let y = 5;
        y + 1
    };
    return x;
}

fn without_variable() -> f64 {
    let x = {
        let _ = false;
        3.14
    };
    return x;
}

fn test_return() -> bool {
    false
}
