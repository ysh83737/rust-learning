use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("5"),
            String::from("6"),
            String::from("7"),
        ];

        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("Got: {}", msg);
    }
}
