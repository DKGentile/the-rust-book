use std::collections::HashMap;
use std::fmt;

enum DateInCoolType {
    Month(String),
    Day(i8),
    Year(i16),
}

impl fmt::Display for DateInCoolType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DateInCoolType::Month(m) => write!(f, "{m}"),
            DateInCoolType::Day(d) => write!(f, "{d}"),
            DateInCoolType::Year(y) => write!(f, "{y}"),
        }
    }
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}!");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("{}, right?", third),
        None => println!("There is no third element!"),
    }

    println!("Well, let's see: ");
    for i in &v {
        print!("{} ", i);
    }

    println!("\nWhat if we doubled each element?");
    let mut t = Vec::new();
    for i in &v {
        t.push(i * 2);
    }
    for i in t {
        print!("{} ", i);
    }

    println!("\nNow I want a vector of different types!\nYou've gone mad!");
    let calander = vec![
        DateInCoolType::Month("Jan".to_string()),
        DateInCoolType::Day(30),
        DateInCoolType::Year(2032),
    ];
    for i in &calander {
        print!("{} ", i);
    }
    println!("\nCan we move onto the next chapter now?\nNope, onto Hash Maps!");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Here are all the scores:");
    for (key, value) in &scores {
        //if we don't put an & here,   ^ we destroy the hashmap for some reason :DD
        println!("{}: {}", key, value);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("What was {}?", team_name);
    println!("I just said {} scored: {}", team_name, score);

    let new_team = String::from("Red");
    let new_team_score = 67;
    println!("Did you know {} scored {}?", new_team, new_team_score);

    scores.insert(new_team, new_team_score);
    println!("Let me add it to the list:");
    //println!("{scores:?}"); prints the whole hash map at once
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    println!("Now we can move on to CH9!");
}
