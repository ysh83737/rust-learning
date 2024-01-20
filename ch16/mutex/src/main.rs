use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        println!("m = {:?}", m);
        *num = 6;
        println!("m = {:?}", m);
    }

    println!("m = {:?}", m);
}
