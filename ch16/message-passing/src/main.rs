use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hello");
        tx.send(msg).unwrap();
        // println!("Sent: {msg}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
