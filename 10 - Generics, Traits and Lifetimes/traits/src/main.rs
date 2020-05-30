trait Description { // for external import of this trait: `use <crate>::Description;`
    fn get_organism_type(&self) -> String;

    fn describe(&self) -> String {
        format!("I am an {}!", self.get_organism_type()) // default method calls non-default method
    }
}

#[derive(Debug)]
struct Cat {
    skin_color: String,
    sound: String,
}

#[derive(Debug)]
struct Dog {
    kind: String,
    sound: String,
}

impl Description for Cat {
    fn describe(&self) -> String {
        format!("I am a {} cat. {}!", self.skin_color, self.sound)
    }

    fn get_organism_type(&self) -> String {
        format!("feline")
    }
}

impl Description for Dog {
    // use default for describe

    fn get_organism_type(&self) -> String {
        format!("mammal")
    }
}

/* fn works for any type which impl Description trait */
fn alert(animal: &impl Description) {
    println!("Animal says: {}", animal.describe());
}

fn yell<T: Description>(animal: &T) {
    println!("Animal yells: {}", animal.describe());
}

fn main() {
    let cat = Cat{ skin_color: format!("white"), sound: format!("meow") };
    let dog = Dog{ kind: format!("Labdrador"), sound: format!("bhow") };

    println!("Cat says: {}", cat.describe());
    println!("Dog says: {}", dog.describe());

    alert(&dog);
    yell(&dog);
}
