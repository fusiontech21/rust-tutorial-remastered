// Chapter 10: Generics & Traits - Example Code

use std::fmt::Display;

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
}

// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Trait
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("Headline: {}", self.headline)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait bound
fn print_item<T: Display>(item: T) {
    println!("{}", item);
}

fn main() {
    // Generic struct
    let int_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 2.0);
    println!("Point x: {}", int_point.x());
    
    // Generic function
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest: {}", largest(&numbers));
    
    // Trait
    let article = NewsArticle {
        headline: String::from("Rust Released"),
    };
    println!("{}", article.summarize());
    
    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Hello!"),
    };
    println!("{}", tweet.summarize());
    
    // Trait bound
    print_item(42);
    print_item("hello");
    print_item(3.14);
}
