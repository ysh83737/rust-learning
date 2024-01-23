fn main() {
    let s = Some(5);

    match s {
        Some(num) if num % 2 == 0 => println!("The number {} is even", num),
        Some(num) => println!("The number {} is odd", num),
        None => println!("Wrong number"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = y;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
