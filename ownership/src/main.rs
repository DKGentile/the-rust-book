fn main() {
    // let s = String::from("hello"); //creating a string from a string literal

    let mut s = String::from("hello"); //creating a mutable string from a string literal
    s.push_str(", world!"); //appends a string literal to a string
    println!("{s}"); // hello, world!

    let h = s.clone(); // 'h' is a deep copy, a clone, of variable s; not a new pointer pointing to what 's' was pointing at
    println!("{h}");
    println!("{s}"); //because 's' was cloned is wasn't dropped/freed, meaning it still exists.

    let x = 5;
    let y = x;
    println!("\nx: {}\ny: {}\n", x, y);

    /*

        But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.

        The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make.
        That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here,
        so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.

        Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10).
        If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

        Rust won’t let us annotate a type with Copy if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type,
        we’ll get a compile-time error. To learn about how to add the Copy annotation to your type to implement the trait, see “Derivable Traits” in Appendix C.

    */
}

/*

when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:
{
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
}                                       // this scope is now over, and s is no
                                        // longer valid


Look here:
    let s1 = String::from("hello");
    let s2 = s1;

    //s2 becomes a shallow copy of 's1', and 's1' is freed

    println!("{s1}, world!"); //meaning this would cause an error.





The inverse of this is true for the relationship between scoping, ownership, and memory being freed via the drop function as well. When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately.
Consider this code, for example:

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

We initially declare a variable s and bind it to a String with the value "hello". Then we immediately create a new String with the value "ahoy" and assign it to s. At this point, nothing is referring to the original value on the heap at all.

*/
