#[derive(Debug)]
struct User {
    username: String,
    password: String,
}
#[derive(Debug)]
struct Numbers(isize, isize, isize, isize, isize);

fn main() {
    let mut u = User {
        username: String::from("admin"),
        password: String::from("Password123"),
    };
    println!("{:?}", u);
    u.username = String::from("test");
    println!("{:?}", u);

    let test_user = build_user(String::from("username"), String::from("password"));
    println!("{:?}", test_user);

    let new_user = User {
        username: String::from("new_username"),
        ..u
    };

    println!("{:?}", new_user);

    let x = Numbers(1, 2, 3, 4, 5);
    println!("{:?}", x);
    println!("{:?}", Numbers(1, 3, 5, 7, 9))
}

fn build_user(username: String, password: String) -> User {
    User { username, password }
}
