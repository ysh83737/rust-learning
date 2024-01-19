use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handler = thread::spawn(move || {
        for item in v {
            println!("item = {}", item);
        }
    });

    // drop(v);

    handler.join().unwrap();
}
