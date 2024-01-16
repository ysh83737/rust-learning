struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(*y, 5);
}
