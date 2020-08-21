use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        for i in 1..10 {
            thread::sleep(Duration::from_secs(1));
            println!("spawn thread says {} {:?}", i, v);
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        thread::sleep(Duration::from_secs(1));
        println!("Main thread says {}", i);
    }

}
