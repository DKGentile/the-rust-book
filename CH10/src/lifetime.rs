// lifetime_demo.rs
fn main() {
    let result;

    {
        let string1 = String::from("hello");
        let string2 = String::from("world");

        // longest() returns a reference — it MUST NOT outlive the data it points to
        result = longest(&string1, &string2);
        println!("The longest string is: {}", result);
    } // string1 and string2 die here

    // If Rust didn't have lifetimes, this next line would be a dangling pointer!
    // println!("{}", result);  // Would crash in C/C++, but Rust forbids this at compile time
}

// This function returns a reference (&str), so Rust needs to know:
// "How long is this reference valid?"
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

/*
THE 'a EXPLAINED IN ONE SENTENCE:

The 'a is a lifetime parameter. It says:
"The reference we return lives exactly as long as both input references.
Since both inputs die at the end of the block, the returned reference must die then too."

In other words:
- &'a str = "a string slice that lives for lifetime 'a"
- The 'a ties together the input lifetimes and the output lifetime
- Rust uses this to 100% guarantee at compile time that we never return a dangling pointer

Without the 'a, this function would not compile — Rust would say:
"Wait — how long does the returned &str live? We can't tell if it's safe!"

So 'a is not magic. It's just Rust asking you:
"Prove this reference doesn't outlive the data it points to."
You answer with 'a, and the compiler checks it for<'a> checks it for you.
*/
