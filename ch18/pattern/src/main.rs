fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("End, x = {:?}, y = {}", x, y);

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point {
        x: 0,
        y: 5,
    };

    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);

    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the x direction {x} and in the y direction {y}"),
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change the color to red {r}, green {g}, and blue {b}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Change color to hue {h}, saturation {s}, value {v}"),
    }

    let ((a, b), Point { x, y }) = ((3, 2), Point { x: 4, y: 5 });
    println!("a={a}, b={b}, x={x}, y={y}");

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customized value"),
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (1, 2, 3, 4, 5);
    match numbers {
        (one, _, three, _, five) => println!("Some numbers: {one}, {three}, {five}"),
    }

    let _x = 1;

    let s = Some(String::from("Hello"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("s = {:?}", s);

    let p = Point {
        x: 1,
        y: 2,
    };

    match p {
        Point { x, .. } => println!("x = {}", x),
    }

    let (one, .., five) = numbers;
    println!("one = {}, five = {}", one, five);
}

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn foo(_: i32, y: i32) {
    println!("y = {y}");
}
