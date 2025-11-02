fn main() {
    let guess: u8 = "42".parse().expect("Not a number");
    println!("{}", guess);

    let c: char = 'c';
    println!("{}", c); //works with emojis too!

    let trvke: bool = true;
    println!("{}", trvke);

    let nuke: String = "Nuke".to_string();
    println!("{}", nuke);

    let tup: (i32, f64, u8) = (500, 6.4, 1); //cannot itterate over with for loop. can not access elements with dynamic nums. tup.0 works, tup.x does not.
    let (x, y, z) = tup; //this is how you can assign index "destructuring"
    println!(
        "\nThe value at (tuple):\ntup.0 = {}\ntup.1 = {}\ntup.2 = {}\n",
        x, y, z
    );

    let val = x; //or let val: i32 = x; or let val = tup.0;
    println!("{}", val);

    let array: [i32; 3] = [1, 2, 3]; //or just let array = [1,2,3];
    let months = ["Jan", "Feb", "Mar"];
    println!("\nMonths (Array)");
    for m in months {
        //array elements can also be accessed with months[x];
        println!("{}", m);
    }
    println!("Nums (Array):");
    for m in array {
        println!("{}", m);
    }
    println!();

    let m = add(60, 9);
    println!("{}", m);
    if m > 70 {
        println!("{m} is greater than 70");
    } else {
        println!("{m} is less than 70");
    }

    let mut ticker = 0;
    let fubak = loop {
        ticker += 1;
        if ticker > 9 {
            break ticker * 2;
        }
    };
    println!("{}\n", fubak);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}\n");

    /*
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!\n");
    */
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!\n");
}

fn add(x: i8, y: i8) -> i8 {
    x + y //no semicolon
}
