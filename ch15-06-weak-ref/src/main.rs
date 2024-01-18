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

    println!("================================");
    println!("leaf rc strong count = {}", Rc::strong_count(&leaf));
    println!("leaf rc weak count = {}", Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
    
        println!("================================");
        println!("leaf rc strong count = {}", Rc::strong_count(&leaf));
        println!("leaf rc weak count = {}", Rc::weak_count(&leaf));
        println!("branch rc strong count = {}", Rc::strong_count(&branch));
        println!("branch rc weak count = {}", Rc::weak_count(&branch));
    
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
        println!("================================");
        println!("leaf rc strong count = {}", Rc::strong_count(&leaf));
        println!("leaf rc weak count = {}", Rc::weak_count(&leaf));
        println!("branch rc strong count = {}", Rc::strong_count(&branch));
        println!("branch rc weak count = {}", Rc::weak_count(&branch));
    
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }

    println!("================================");
    println!("leaf rc strong count = {}", Rc::strong_count(&leaf));
    println!("leaf rc weak count = {}", Rc::weak_count(&leaf));

    // println!("leaf = {:?}", leaf);
    // println!("branch = {:?}", branch);

    // let leaf2 = Rc::new(Node {
    //     value: 4,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![])
    // });

    // println!("================================");
    // println!("leaf2 rc strong count = {}", Rc::strong_count(&leaf2));
    // println!("leaf2 rc weak count = {}", Rc::weak_count(&leaf2));
    // println!("branch rc strong count = {}", Rc::strong_count(&branch));
    // println!("branch rc weak count = {}", Rc::weak_count(&branch));

    // branch.children.borrow_mut().push(Rc::clone(&leaf2));

    // println!("================================");
    // println!("leaf2 rc strong count = {}", Rc::strong_count(&leaf2));
    // println!("leaf2 rc weak count = {}", Rc::weak_count(&leaf2));
    // println!("branch rc strong count = {}", Rc::strong_count(&branch));
    // println!("branch rc weak count = {}", Rc::weak_count(&branch));

    // *leaf2.parent.borrow_mut() = Rc::downgrade(&branch);
    // println!("leaf2 parent = {:?}", leaf.parent.borrow().upgrade());

    // println!("branch = {:?}", branch);
}
