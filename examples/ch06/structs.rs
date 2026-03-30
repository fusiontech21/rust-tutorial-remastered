// Chapter 6: Structs & Methods - Example Code

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Struct instance
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };
    println!("User: {:?}", user);
    
    // Mutable struct
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        active: false,
    };
    user2.active = true;
    println!("User2: {:?}", user2);
    
    // Struct update syntax
    let user3 = User {
        username: String::from("charlie"),
        ..user2
    };
    println!("User3: {:?}", user3);
    
    // Rectangle methods
    let rect = Rectangle::new(30, 50);
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    
    // Mutable method
    let mut rect2 = Rectangle::new(10, 20);
    rect2.scale(2);
    println!("Scaled: {:?}", rect2);
    
    // Associated function
    let square = Rectangle::square(10);
    println!("Square: {:?}", square);
}
