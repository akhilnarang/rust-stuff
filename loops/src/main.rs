fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 3;
        println!("Loop value is {}!", counter);
        if counter == 42 {
            break counter;
        }
    };
    println!("Result is {}", result);

    while counter > 0 {
        counter -= 10;
        println!("Counter is {}", counter);
    }
    println!("Counter is now {}", counter);

    let arr = [1, 3, 5, 7, 9];
    for element in arr.iter() {
        println!("We are now at {}", element);
    }

    for element in 1..4 {
        println!("We're at {}", element);
    }
}
