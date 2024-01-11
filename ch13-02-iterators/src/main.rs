fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // for item in v1_iter {
    //     println!("Got: {}", item);
    // }

    // v1_iter moved
    // println!("v1_iter = {:#?}", v1_iter);

    println!("v1_iter = {:?}", v1_iter);
    println!("Next: {:?}", v1_iter.next());
    println!("v1_iter = {:?}", v1_iter);
    println!("Next: {:?}", v1_iter.next());
    println!("v1_iter = {:?}", v1_iter);
    println!("Next: {:?}", v1_iter.next());
    println!("v1_iter = {:?}", v1_iter);
    println!("Next: {:?}", v1_iter.next());
    println!("v1_iter = {:?}", v1_iter);

    println!("v1 = {:?}", v1);


    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    // println!("v1_iter = {:?}", v1_iter);
    println!("total={}", total);


    let v1_iter = v1.iter();
    let m2 = v1_iter.map(|x| x + 1);
    let v2: Vec<i32> = m2.collect();
    println!("v2 = {:?}", v2);
}
