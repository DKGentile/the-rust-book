fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*

    First, notice that all the tuple code in the variable declaration and the function return value is gone.
    Second, note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String.
    These ampersands represent references, and they allow you to refer to some value without taking ownership of it.

THIS DOESNT WORK THOUGH. CAN NOT MODIFY A REFERENCE:
    fn main() {
        let s = String::from("hello");

        change(&s);
    }

    fn change(some_string: &String) {
        some_string.push_str(", world");
    }

*/
