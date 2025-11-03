pub fn conversion() {
    let choice = get_choice();
    let temp = get_temp();
    if choice == 1 {
        println!("{}", ct_f(temp));
    } else {
        println!("{}", ft_c(temp));
    }
}

fn get_temp() -> f32 {
    println!("Enter your temp: ");
    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp).expect("Error");
    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => get_temp(),
    };
    temp
}

fn get_choice() -> u8 {
    println!("Celcius->Fahrenheit or Celcius<-Fahrenheit?: [1/2]");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Error");
    let mut choice: u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => get_choice(),
    };
    if choice != 1 && choice != 2 {
        println!("incorrect choice");
        choice = get_choice();
    }
    choice
}

fn ct_f(x: f32) -> f32 {
    x * (9.0 / 5.0) + 32.0
}

fn ft_c(x: f32) -> f32 {
    (x - 32.0) * (5.0 / 9.0)
}
