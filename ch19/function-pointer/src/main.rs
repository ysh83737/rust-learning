fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

fn main() {
    let answer = add_twice(add_one, 1);

    println!("answer = {}", answer);

    let list_of_numbers = vec![1, 2, 3, 4];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|x| x.to_string()).collect();

    println!("list_of_strings = {:?}", list_of_strings);

    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings = {:?}", list_of_strings);

    let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list_of_status = {:?}", list_of_status);
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}
