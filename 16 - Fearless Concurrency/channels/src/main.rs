use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let values_to_send = vec![
        String::from("water"),
        String::from("bottle"),
        String::from("is"),
        String::from("empty!"),
    ];

    for i in 1..5 {
        let tx_tmp = mpsc::Sender::clone(&tx);
        let values = values_to_send.clone();
        thread::spawn(move || {
            for v in values {
                thread::sleep(Duration::from_secs(1));
                tx_tmp.send((i, v)).unwrap();
            }
        });
    }

    println!("{:?}", rx);
    let received = rx.recv().unwrap();
    println!("Message to main thread from {}: {}", received.0, received.1);

    for msg in rx {
        println!("Message to main thread from {}: {}", msg.0, msg.1);
    }
}
