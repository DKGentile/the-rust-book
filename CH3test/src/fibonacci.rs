pub fn fib() {
    println!("{}", calc_fib(get_num()));
}

fn get_num() -> u32 {
    println!("Enter your Fibonacci N: ");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("error");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => get_num(),
    };
    choice
}

fn calc_fib(x: u32) -> u32 {
    if x <= 1 {
        return x;
    }
    calc_fib(x - 1) + calc_fib(x - 2)
}
