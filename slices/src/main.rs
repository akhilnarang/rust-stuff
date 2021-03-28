fn main() {
    let x = String::from("Hello World\nSecond Line");
    let first_word = first_word(&x);
    let first_word_index = first_word.len();
    println!("Length of first word in {} is {}", x, &first_word_index);
    println!("First word is {}", first_word);
    println!("Rest of the string is:\n{}", &x[first_word_index..])
}

fn first_word(s: &str) -> &str {
    for (i, &c) in s.as_bytes().iter().enumerate() {
        if c == b' ' {
            return &s[..i];
        }
    }
    s
}
