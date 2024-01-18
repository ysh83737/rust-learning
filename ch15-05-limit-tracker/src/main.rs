use ch15_05_limit_tracker::{ LimitTracker, Messager };

struct Logger {}

impl Logger {
    fn new() -> Logger {
        Logger {}
    }
}

impl Messager for Logger {
    fn send(&self, msg: &str) {
        println!("[Logger]: {}", msg);
    }
}

fn main() {
    let messager = Logger::new();

    let mut tracker = LimitTracker::new(&messager, 10);
    tracker.set_value(1);
    tracker.set_value(7);
    tracker.set_value(8);
    tracker.set_value(9);
    tracker.set_value(10);
}
