use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter_mutex = Arc::new(Mutex::new(0));
    let mut threads = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter_mutex);
        threads.push(
            thread::spawn(move || {
                let mut c = counter.lock().unwrap();
                *c += 1;
            })
        );
    }

    for t in threads {
        t.join().unwrap();
    }
    println!("{}", counter_mutex.lock().unwrap());
}
