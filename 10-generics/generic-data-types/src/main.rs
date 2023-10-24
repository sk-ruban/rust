// function largest is generic over some type T
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest { // Only some types are comparable so T must be restricted
            largest = item;
        }
    }

    largest
}

// struct can hold values of any but the same type
struct Point<T> {
    x: T,
    y: T,
}

// we declare T right after impl because we're implementing methods on Point<T>
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// constraints on f32 ONLY
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 }; This will not work

    println!("p.x = {}", float.x());
}