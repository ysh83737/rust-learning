trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Fly as `Pilot`");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Fly as `Wizard`");
    }
}

impl Human {
    fn fly(&self) {
        println!("Fly as `Human`");
    }
}

fn main() {
    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
