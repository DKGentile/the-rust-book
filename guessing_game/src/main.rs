use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() 
{
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please make a guess:");
        let mut guess = String::new();
        io::stdin()                         //or, if you want to have even more fun, remove "use std::io" and replace this line with std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
        If parse is able to successfully turn the string into a number, it will return an Ok value that contains the resultant number. 
        That Ok value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the Ok value. 
        That number will end up right where we want it in the new guess variable we’re creating.

        If parse is not able to turn the string into a number, it will return an Err value that contains more information about the error. 
        The Err value does not match the Ok(num) pattern in the first match arm, but it does match the Err(_) pattern in the second arm. The underscore, _, is a catch-all value; in this example, 
        we’re saying we want to match all Err values, no matter what information they have inside them. So the program will execute the second arm’s code, continue, 
        which tells the program to go to the next iteration of the loop and ask for another guess. So, effectively, the program ignores all errors that parse might encounter!
        */

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number)
        {
            Ordering::Equal => {
                println!("Nice.");
                break;
            }
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("TOO BIG!"),
        }
    }
}
