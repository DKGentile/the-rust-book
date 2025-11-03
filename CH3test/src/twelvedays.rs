pub fn carol() {
    let days: i8 = 12;
    let mut ticker: i8 = 0;
    while ticker < days {
        println!(
            "On the {} day of Christmas my true love gave to me",
            ORDINALS[ticker as usize]
        );
        all_and_before(ticker);
        ticker += 1;
    }
}

fn all_and_before(mut x: i8) {
    while x >= 0 {
        print!("{}, ", GIFTS[x as usize]);
        x -= 1;
    }
    println!();
}

const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 12] = [
    "a Partridge in a Pear Tree",
    "two Turtle Doves",
    "three French Hens",
    "four Calling Birds",
    "fiiiiive Gold Rings",
    "six Geese a-Laying",
    "seven Swans a-Swimming",
    "eight Maids a-Milking",
    "nine Ladies Dancing",
    "ten Lords a-Leaping",
    "eleven Pipers Piping",
    "twelve Drummers Drumming",
];
