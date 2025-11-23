trait Bark {
    fn bark(&self);               // "I promise I have this method"
}

struct Dog;
struct Cat;
struct Robot;

impl Bark for Dog {
    fn bark(&self) { println!("Woof!"); }
}

impl Bark for Cat {
    fn bark(&self) { println!("Meow... fine, woof."); }
}

fn make_it_bark<T: Bark>(thing: T) {  // or: fn make_it_bark(thing: &impl Bark)
    thing.bark();
}

make_it_bark(Dog);     // Woof!
make_it_bark(Cat);     // Meow... fine, woof.