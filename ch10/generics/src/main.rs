
fn main() {
    println!("largest {}", largest_element(&vec![5, 2, 7, 1, 2, 3, 9]));
    println!("largest {}", largest_element(&vec!['a', 'c', 'z', 'm']));
    println!("largest {}", largest_element(&vec!["hello", "bye", "sayounara"]));

    let point = Point{x: 4, y: 3.4};
    println!("{:?}", point);

    println!("{}", point.x())
}

fn largest_element<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &e in list {
        if e > largest {
            largest = e;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}
