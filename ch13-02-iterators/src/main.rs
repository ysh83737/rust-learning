fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for item in v1_iter {
        println!("Got: {}", item);
    }
}
