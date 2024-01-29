fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x) + f(x)
}

fn main() {
    let answer = add_twice(add_one, 1);

    println!("answer = {}", answer);
}
