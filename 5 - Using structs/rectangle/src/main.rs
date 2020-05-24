#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn get_area(r: &Rectangle) -> u32 {
    r.width * r.height
}

impl Rectangle {
    /* METHODS:
     (&self) equivalent to (self: &Rectangle), can even use (self) or (&mut self) */
    fn area(&self) -> u32 { 
        self.width * self.height
    }

    /* ASSOCIATED FUNCTIONS: (static functions: no object or self reference needed) */
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_fit(&self, r: &Rectangle) -> bool {
        self.width >= r.width && self.height >= r.height 
    }
}

fn main() {
    let r1 = Rectangle {
        width: 4,
        height: 5,
    };

    println!("r1: {:#?}", r1);
    println!("Area: {}", get_area(&r1));
    println!("Area: {}", r1.area());

    let r2 = Rectangle {
        width: 3,
        height: 2,
    };

    println!("r2: {:#?}", r2);
    println!("Can r2 fit into r1 ? {}", r1.can_fit(&r2));

    /* square function is namespaced by the struct Rectangle */
    println!("Square: {:#?}", Rectangle::square(2));
}
