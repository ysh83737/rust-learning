use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        println!("m = {:?}", m);
        *num = 6;
        println!("m = {:?}", m);
    }

    println!("m = {:?}", m);

    let counter = Rc::new(Mutex::new(0));
    let mut handlers = vec![];

    for i in 0..10 {
        let counter = Rc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap())
}
