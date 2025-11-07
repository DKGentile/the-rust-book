//#[derive(Debug)] include this line above this struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn add_two(&self, rect2: &Rectangle) -> u32 {
        self.area() + rect2.area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let username = enter_username();
    let email = enter_email();

    let you = build_user(email.trim().to_string(), username.trim().to_string());

    if you.active {
        println!(
            "\nHello, {} from {}\nYou have logged in {} time(s)!",
            you.username, you.email, you.sign_in_count
        );
    }
    //println!("{you:?}"); for this to work (debugging)

    //let you2 = User { ..you }; this will copy the struct to a new variable

    let rect1 = Rectangle {
        width: 5,
        height: 4,
    };
    println!(
        "\nThe area of a {} by {} rectangle is {}. This area added to itself is: {}",
        rect1.width,
        rect1.height,
        rect1.area(),
        rect1.add_two(&rect1),
    );

    let number = get_num();
    let square = Rectangle::square(number);
    println!("The area of this square is: {}", square.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 1,
    }
}

fn enter_email() -> String {
    println!("Enter your email: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("FAILED EMAIL");
    input
}

fn enter_username() -> String {
    println!("Enter your username: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("FAILED USERNAME");
    input
}

fn get_num() -> u32 {
    println!("Enter a number: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("FAILED NUMBER");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => get_num(),
    };
    input
}
