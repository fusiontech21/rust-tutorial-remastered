// Chapter 14: Concurrency & Async - Example Code

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Thread creation
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread: {}", i);
        }
    });
    
    for i in 1..3 {
        println!("Main: {}", i);
    }
    
    handle.join().unwrap();
    
    // Message passing with channels
    use std::sync::mpsc;
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send(String::from("hello")).unwrap();
    });
    
    let msg = rx.recv().unwrap();
    println!("Received: {}", msg);
    
    // Arc with Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter: {}", *counter.lock().unwrap());
}
