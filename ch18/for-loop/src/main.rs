fn main() {
    let v = vec!["a", "b", "c"];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    for item in v.iter() {
        println!("item = {}", item);
    }
}
