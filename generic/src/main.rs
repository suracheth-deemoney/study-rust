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

impl<T, U> Point<T, U> {
    fn mixup<T1, U1>(self, other: Point<T1, U1>) -> Point<T, U1> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
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

impl SingleTypePoint<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);

    println!("The largest number is {result}");

    let point_both_integer = Point { x: 5, y: 10 };
    let _point_both_float = Point { x: 1.0, y: 4.0 };
    let point_string_and_char = Point { x: "Hello", y: 'c' };
    let _point_integer_and_float = Point { x: 5, y: 4.0 };

    let point_mix_up = point_both_integer.mixup(point_string_and_char);

    println!(
        "point_mix_up.x = {:?}, point_mix_up.y = {}",
        point_mix_up.x, point_mix_up.y
    );

    let p = SingleTypePoint { x: 5.0, y: 10.0 };

    println!("p.x = {}", p.x());
    println!("distnce from origin = {}", p.distance_from_origin());
}
