static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: i32 = 2;

fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("Static variable = {}", HELLO_WORLD);

    add_to_count(10);

    unsafe {
        println!("COUNTER = {}", COUNTER);
    }
}
