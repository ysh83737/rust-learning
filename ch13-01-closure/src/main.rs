fn main() {
    // let example_closure = |x| x;

    // let s = example_closure(String::from("Hello"));
    // let n = example_closure(5);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);


    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = || {
        list.push(4);
        println!("From closure: {:?}", list);
    };

    // immutable borrow
    // println!("From closure: {:?}", list);

    borrow_mutably();
    println!("After calling closure: {:?}", list);
}
