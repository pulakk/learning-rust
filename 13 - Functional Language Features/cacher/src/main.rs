use std::thread;
use std::time::Duration;

// fn expensive_calc(intensity: u16) -> u16 {
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn print_workout_plan(intensity: u16, day_of_week: u16) {
    let mut expensive_cacher = Cacher::new(|num| {
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity > 25 {
        // println!("Do {} pushups!", expensive_calc(intensity));
        // println!("Do {} situps!", expensive_calc(intensity));
        println!("Do {} pushups!", expensive_cacher.get_value(intensity));
        println!("Do {} situps!", expensive_cacher.get_value(intensity));
    } else {
        if day_of_week == 7 {
            println!("Rest today!");
        } else {
            println!("Run for {} minutes", intensity);
        }
    }
}

struct Cacher<T> 
where
    T: Fn(u16) -> u16,
{
    closure: T,
    value: Option<u16>,
}

impl<T> Cacher<T>
where
    T: Fn(u16) -> u16
{
    fn new(calc: T) -> Cacher<T> {
        Cacher {
            closure: calc,
            value: None,
        }
    }

    fn get_value(&mut self, arg: u16) -> u16 {
        match self.value {
            Some(v) => v,
            None => {
                let value = (self.closure)(arg);
                self.value = Some(value);
                value
            }
        }
    }
}

fn main() {
    let intensity = 32;
    let day_of_week = 6;

    print_workout_plan(intensity, day_of_week);
}
