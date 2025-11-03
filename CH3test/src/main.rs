mod conversion;
mod fibonacci;
mod twelvedays;
fn main() {
    conversion::conversion();
    println!();
    fibonacci::fib();
    println!("Press Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());
    twelvedays::carol();
}
