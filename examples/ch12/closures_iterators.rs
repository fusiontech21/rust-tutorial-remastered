// Chapter 12: Closures & Iterators - Example Code

fn main() {
    // Closures
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));
    
    let add = |x, y| x + y;
    println!("add(3, 4) = {}", add(3, 4));
    
    // Closure with environment
    let x = 42;
    let print_x = || println!("x = {}", x);
    print_x();
    
    // Move closure
    let s = String::from("hello");
    let print_s = move || println!("{}", s);
    print_s();
    // println!("{}", s);  // ERROR: s was moved
    
    // Iterators
    let v = vec![1, 2, 3, 4, 5];
    
    // Iterator adapters
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
    
    // Consuming adapters
    let sum: i32 = v.iter().sum();
    println!("Sum: {}", sum);
    
    let max = v.iter().max();
    println!("Max: {:?}", max);
    
    // fold
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("Fold sum: {}", sum);
    
    // Custom iterator
    let counter = Counter::new(5);
    for num in counter {
        println!("Counter: {}", num);
    }
}

// Custom iterator
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
