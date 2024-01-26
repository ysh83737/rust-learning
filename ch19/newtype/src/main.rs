use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

type Kilometers = i32;

fn main() {
    let w = Wrapper(vec![
        String::from("Hello"),
        String::from("world"),
    ]);

    println!("w = {}", w);

    let x: i32 = 5;
    let y: Kilometers = 6;

    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));

    take_long_type(f);

    let f = return_long_type();

    take_long_type(f)
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn take_long_type(f: Thunk) {
    f();
}

fn return_long_type() -> Thunk {
    Box::new(|| println!("hello"))
}
