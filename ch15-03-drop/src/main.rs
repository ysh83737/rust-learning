use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let mut customSmartPointer1 = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let customSmartPointer2 = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers is created!");
    drop(customSmartPointer1);
    println!("CustomSmartPointer dropped before the end of main.");
}
