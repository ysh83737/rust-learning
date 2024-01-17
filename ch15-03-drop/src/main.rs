struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let customSmartPointer1 = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let customSmartPointer2 = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers is created!");
}
