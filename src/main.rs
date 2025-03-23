#[derive(Debug)]
struct Point<T> {
    _x: T,
    _y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self._x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self._x.powi(2) + self._y.powi(2)).sqrt()
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 43, 23, 56, 75, 23];
    let result = largest(&number_list);
    println!("The lareges intger is {result}");
    let char_list = vec!['a', 'z', 'p', 'o', 'w', 'v', 'P'];
    let result = largest(&char_list);
    println!("The lareges intger is {result}");
    let float = Point { _x: 1.0, _y: 23.2 };
    let p = Point { _x: 55.0, _y: 23.2 };
    let intger = Point { _x: 10, _y: 3 };
    let will_work = Point { _x: 11, _y: 10 };
    println!("{float:#?}<->{intger:#?}<-> {will_work:#?}");
    println!("{} -> {:#?}", p.x(), p.distance_from_origin());
}
