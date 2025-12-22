fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

struct SingleTypePoint<T> {
    x: T,
    y: T,
}

impl<T> SingleTypePoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest number is {result}");

    let _point_both_integer = Point { x: 5, y: 10 };
    let _point_both_float = Point { x: 1.0, y: 4.0 };
    let _point_integer_and_float = Point { x: 5, y: 4.0 };

    let p = SingleTypePoint { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
