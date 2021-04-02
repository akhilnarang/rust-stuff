#[derive(Debug)]
enum Test {
    Lol
}
#[derive(Debug)]
enum Vowel {
    A,
    E, 
    I,
    O,
    U,
    No(Test),
}

fn position(vowel: Vowel) -> i8 {
    match vowel {
        Vowel::A => 1,
        Vowel::E => 2,
        Vowel::I => 3,
        Vowel::O => 4,
        Vowel::No(temp) => {
            println!("{:?}", temp);
            0
        },
        _ => {
            println!("Unmatched case - {:?}", vowel);
            -1
        }
    }
}

fn main() {
    println!("{}", position(Vowel::U));
    println!("{}", position(Vowel::O));
    println!("{}", position(Vowel::I));
    println!("{}", position(Vowel::E));
    println!("{}", position(Vowel::A));
    println!("{}", position(Vowel::No(Test::Lol)));
}
