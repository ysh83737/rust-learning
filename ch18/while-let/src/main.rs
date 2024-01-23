fn main() {
    let mut stack = vec![];

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("Top = {}", top);
    }
}
