fn find_largest_number(numbers: &[i32]) -> i32{
    let mut largest = numbers[0];

    for &number in numbers {
        if number > largest {
            largest = number;
        }
    
    }

    largest
}

fn find_largest_char(chars: &[char]) -> char {
    let mut largest = chars[0];

    for &c in chars {
        if c > largest {
            largest = c;
        }
    }

    largest
}

fn find_largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for l in list {
        if l > largest {
            largest = &l;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

/* implement for a generic type, need <T> after impl */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/* can implement only for a specific type */
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/* can return other types, specify <T> in impl */
impl <T> Point<T> {
    /* specify <U> (extra types) after flip */
    fn flip<U>(&self, other: Point<U>) -> Point<U> {
        Point {
            x: other.x,
            y: other.y
        }
    }
}

#[derive(Debug)]
enum Color<T> {
    Rgb(T, T, T),
    Hex(T),
}

fn main() {
    /* structs */
    let p1 = Point{ x: 1, y: 2 };
    let p2 = Point{ x: 1.2, y: 3.4 };

    println!("{}, {}", p1.x, p1.y);
    println!("{}, {}", p2.x(), p2.y);
    println!("{}", p2.distance_from_origin());

    let p2 = p2.flip(p1);
    println!("{} {}", p2.x, p2.y);

    /* enums */
    println!("{:?} {:?}", Color::Rgb(255, 255, 255), Color::Hex(String::from("#000")));

    /* functions */
    let numbers: Vec<i32> = vec![2, 5, 3, 4, 1];
    let characters: Vec<char> = vec!['b', 'd', 'c', 'a'];

    println!("Largest num: {}", find_largest_number(&numbers));
    println!("Largest char: {}", find_largest_char(&characters));
    println!("Largest num: {}", find_largest(&numbers));
    println!("Largest char: {}", find_largest(&characters));
}
