fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("My favorite color is {}", color);
    } else if is_tuesday {
        println!("Today is tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Age is more than 30");
        } else {
            println!("Age is less than 30");
        }
    } else {
        println!("Nothing happened");
    }
}
