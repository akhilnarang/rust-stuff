fn main() {
    let mut s = String::from("test string");

    let len = calculate_length(&s);

    println!("The length of {} is {}", s, len);

    modify_string(&mut s);
    println!("The new string is {}", s);
    let x = &mut s;
    println!("x is {}", x);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn modify_string(s: &mut String) {
    s.push_str("#")
}
