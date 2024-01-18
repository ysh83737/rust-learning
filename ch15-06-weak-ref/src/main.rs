use std::rc::{ Rc, Weak };
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // println!("leaf = {:?}", leaf);
    // println!("branch = {:?}", branch);

    let leaf2 = Rc::new(Node {
        value: 4,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    branch.children.borrow_mut().push(Rc::clone(&leaf2));

    *leaf2.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf2 parent = {:?}", leaf.parent.borrow().upgrade());

    // println!("branch = {:?}", branch);
}
