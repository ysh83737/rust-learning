use std::slice;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("num = {}", num);

    unsafe {
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    assert_eq!(a, [1, 2, 3]);
    assert_eq!(b, [4, 5, 6]);


    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    assert_eq!(a, [1, 2, 3]);
    assert_eq!(b, [4, 5, 6]);
}

unsafe fn dangerous() {
    println!("dangerous fn");
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    // (&mut values[..mid], &mut values[mid..])

    let ptr = values.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
