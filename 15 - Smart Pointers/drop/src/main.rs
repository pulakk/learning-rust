use std::mem;

struct MyType {
    data: String,
}

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Dropping .. data={}", self.data);
    }
}

fn main() {
    {

        let d = MyType { data: String::from("Water") };
        // drop(d.data); // cannot move `data` out of d, because MyType implements drop trait

        mem::drop(d); // d explicitly dropped here

        // println!("{}", d.data); // d moved into drop fn
        println!("Going out of scope");

    } // d would have been dropped here usually
}
