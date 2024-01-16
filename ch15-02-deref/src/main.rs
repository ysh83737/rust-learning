fn main() {
    let x = 5;
    let y = &x;

    // assert_eq!(y, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(*y, 5);
}
