// Chapter 13: Smart Pointers - Example Code

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    // Box
    let b = Box::new(5);
    println!("Box value: {}", *b);
    
    // Rc - Reference Counted
    let rc1 = Rc::new(String::from("hello"));
    let rc2 = Rc::clone(&rc1);
    let rc3 = Rc::clone(&rc1);
    println!("Reference count: {}", Rc::strong_count(&rc1));
    drop(rc2);
    println!("After drop: {}", Rc::strong_count(&rc1));
    
    // RefCell - Interior Mutability
    let data = RefCell::new(5);
    *data.borrow_mut() = 10;
    println!("RefCell value: {}", data.borrow());
    
    // Rc<RefCell<T>> - Shared mutable state
    let shared = Rc::new(RefCell::new(5));
    let s1 = Rc::clone(&shared);
    let s2 = Rc::clone(&shared);
    
    *s1.borrow_mut() = 15;
    println!("s1: {}", s1.borrow());
    println!("s2: {}", s2.borrow());
    
    // Arc - Thread-safe reference counting
    let arc = Arc::new(Mutex::new(0));
    let arc_clone = Arc::clone(&arc);
    
    let handle = std::thread::spawn(move || {
        let mut num = arc_clone.lock().unwrap();
        *num = 42;
    });
    
    handle.join().unwrap();
    println!("Arc value: {}", *arc.lock().unwrap());
}
