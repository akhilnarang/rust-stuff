fn main() {
    let test: (i16, f32, u8) = (200, 3.14, 5);
    let (a, b, c) = test;
    println!("a, b, c are: {}, {}, {}!", a, b, c);
    println!("First value is {}", test.0);
    println!("Second value is {}", test.1);
    println!("Third value is {}", test.2);

    let arr_test = [3, 7, 11];
    println!("First value is {}", arr_test[0]);
    println!("Second value is {}", arr_test[1]);
    println!("Third value is {}", arr_test[2]);
}
