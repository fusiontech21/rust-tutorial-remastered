# Chapter 14: Concurrency & Async Programming

## 14.1 Threads

### Creating Threads

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // Wait for thread to finish
    handle.join().unwrap();
}
```

### Moving Data to Threads

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    // Move ownership to thread
    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
    });
    
    // println!("{:?}", v);  // ERROR: v was moved
    
    handle.join().unwrap();
}
```

### Thread with Return Value

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let result = 42;
        result
    });
    
    let value = handle.join().unwrap();
    println!("Result: {}", value);
}
```

---

## 14.2 Message Passing

### Channels

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let msg = String::from("hello");
        tx.send(msg).unwrap();
        // msg is moved, can't use it again
    });
    
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```

### Multiple Messages

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let messages = vec![
            String::from("hello"),
            String::from("from"),
            String::from("thread"),
        ];
        
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    // Receiving messages
    for received in rx {
        println!("Got: {}", received);
    }
}
```

### Multiple Producers

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    
    thread::spawn(move || {
        let messages = vec!["hi", "from", "thread1"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    thread::spawn(move || {
        let messages = vec!["more", "from", "thread2"];
        for msg in messages {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }
}
```

---

## 14.3 Shared State

### Mutex

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }  // Lock released here
    
    println!("Value: {}", *m.lock().unwrap());
}
```

### Arc with Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
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
```

### RwLock

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);
    
    // Multiple readers
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("r1: {}, r2: {}", *r1, *r2);
    }
    
    // Single writer
    {
        let mut w = lock.write().unwrap();
        *w += 1;
    }
    
    println!("Value: {}", *lock.read().unwrap());
}
```

---

## 14.4 Atomic Types

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter: {}", counter.load(Ordering::SeqCst));
}
```

### Atomic Ordering

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let ready = Arc::new(AtomicBool::new(false));
    let ready_clone = Arc::clone(&ready);
    
    let handle = thread::spawn(move || {
        // Wait until ready
        while !ready_clone.load(Ordering::Acquire) {
            thread::yield_now();
        }
        println!("Ready!");
    });
    
    thread::sleep(std::time::Duration::from_secs(1));
    ready.store(true, Ordering::Release);
    
    handle.join().unwrap();
}
```

---

## 14.5 Send and Sync

### Send Trait

```rust
// Send: can be transferred across threads
// Most types implement Send automatically
// Exceptions: Rc, raw pointers, some cell types

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<i32>();
    assert_send::<String>();
    assert_send::<Vec<i32>>();
    // assert_send::<Rc<i32>>();  // Doesn't compile
}
```

### Sync Trait

```rust
// Sync: can be referenced from multiple threads
// If T: Sync, then &T: Send

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<i32>();
    assert_sync::<String>();
    // assert_sync::<RefCell<i32>>();  // Doesn't compile
}
```

---

## 14.6 Async Programming

### Async/Await Basics

```rust
use std::future::Future;
use std::pin::Pin;

// Async function
async fn hello_async() {
    println!("Hello from async!");
}

// Async function with return
async fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::main]
async fn main() {
    hello_async().await;
    
    let result = add(3, 4).await;
    println!("Result: {}", result);
}
```

### Tokio Runtime

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }

use tokio::time::{sleep, Duration};

async fn task(name: &str) {
    for i in 1..=5 {
        println!("{}: {}", name, i);
        sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::main]
async fn main() {
    let t1 = tokio::spawn(task("Task 1"));
    let t2 = tokio::spawn(task("Task 2"));
    
    t1.await.unwrap();
    t2.await.unwrap();
}
```

### Async Combinators

```rust
use tokio::time::{sleep, Duration};

async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;
    format!("Data {}", id)
}

#[tokio::main]
async fn main() {
    // Sequential
    let d1 = fetch_data(1).await;
    let d2 = fetch_data(2).await;
    
    // Concurrent with join
    let (d1, d2) = tokio::join!(fetch_data(1), fetch_data(2));
    
    // Concurrent with try_join
    let result = tokio::try_join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3)
    );
    
    // Race
    let winner = tokio::select! {
        result = fetch_data(1) => result,
        result = fetch_data(2) => result,
    };
}
```

### Async Channels

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });
    
    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}
```

### Async Mutex

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let data = Arc::clone(&data);
        
        let handle = tokio::spawn(async move {
            let mut num = data.lock().await;
            *num += 1;
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("Counter: {}", *data.lock().await);
}
```

---

## 14.7 Parallelism with Rayon

```rust
// Cargo.toml: rayon = "1.8"

use rayon::prelude::*;

fn main() {
    let numbers: Vec<i32> = (0..1000).collect();
    
    // Parallel iterator
    let sum: i32 = numbers.par_iter()
        .map(|&x| x * 2)
        .filter(|&x| x > 100)
        .sum();
    
    println!("Sum: {}", sum);
    
    // Parallel for_each
    numbers.par_iter()
        .for_each(|x| println!("{}", x));
    
    // Parallel map-reduce
    let result = numbers.par_iter()
        .map(|&x| x * x)
        .reduce(|| 0, |a, b| a + b);
}
```

---

## 14.8 Common Patterns

### Worker Pool

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct WorkerPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: std::sync::mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl WorkerPool {
    fn new(size: usize) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        
        let mut workers = Vec::with_capacity(size);
        
        for _ in 0..size {
            let rx = Arc::clone(&rx);
            let handle = thread::spawn(move || {
                loop {
                    let job = rx.lock().unwrap().recv();
                    match job {
                        Ok(job) => job(),
                        Err(_) => break,
                    }
                }
            });
            workers.push(handle);
        }
        
        WorkerPool { workers, sender: tx }
    }
    
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}
```

### Producer-Consumer

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let queue = Arc::new(Mutex::new(Vec::new()));
    let done = Arc::new(Mutex::new(false));
    
    // Producer
    let prod_queue = Arc::clone(&queue);
    let prod_done = Arc::clone(&done);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            prod_queue.lock().unwrap().push(i);
            thread::sleep(std::time::Duration::from_millis(100));
        }
        *prod_done.lock().unwrap() = true;
    });
    
    // Consumer
    let cons_queue = Arc::clone(&queue);
    let cons_done = Arc::clone(&done);
    let consumer = thread::spawn(move || {
        loop {
            let item = cons_queue.lock().unwrap().pop();
            match item {
                Some(i) => println!("Consumed: {}", i),
                None => {
                    if *cons_done.lock().unwrap() {
                        break;
                    }
                    thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
    });
    
    producer.join().unwrap();
    consumer.join().unwrap();
}
```

---

## Chapter 14 Exercises

### Exercise 14.1: Thread Basics
```rust
// Create multiple threads
// Pass data to threads using move
// Collect results using join
```

### Exercise 14.2: Message Passing
```rust
// Implement producer-consumer with channels
// Create multiple producers, single consumer
// Handle channel disconnection
```

### Exercise 14.3: Shared State
```rust
// Implement counter with Arc<Mutex<T>>
// Compare Mutex vs RwLock performance
// Use atomic types for simple counters
```

### Exercise 14.4: Async Programming
```rust
// Create async functions with tokio
// Practice join!, select!
// Implement async producer-consumer
```

### Exercise 14.5: Parallel Processing
```rust
// Use rayon for parallel iterators
// Parallelize a computation-heavy task
// Compare sequential vs parallel performance
```

---

## Summary

In this chapter, you learned:

✅ Creating and managing threads
✅ Message passing with channels
✅ Shared state with Mutex and Arc
✅ Atomic types and ordering
✅ Send and Sync traits
✅ Async/await syntax
✅ Tokio runtime
✅ Async combinators and channels
✅ Parallel processing with Rayon
✅ Common concurrency patterns

---

## What's Next?

Concurrency gives you power, but macros give you metaprogramming abilities! In Chapter 15, we'll explore **Macros** in Rust.

**Continue to [Chapter 15: Macros](./15_macros.md)**
