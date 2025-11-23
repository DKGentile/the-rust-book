struct Point<T> {
    //x and y are of generic type, but must be of the same generic type
    x: T,
    y: T,
}

impl Point<f32> {
    //methods for when Point Type is of f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/*
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    //I honestly don't know how to interpret this
    //if Point is of two different templates/types, we will return a normalized struct
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
*/

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn main() {
    println!("We will use Type/Template to find the largest value in the following vectors");
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("The largest number is: {}", largest(&number_list));
    println!("The largest character (ASCII) is: {}", largest(&char_list));

    let dot = Point { x: 5, y: 7 };
    println!("{} {}", dot.x, dot.y);
    let dot = Point { x: 't', y: 'h' };
    println!("{} {}", dot.x, dot.y);
}
