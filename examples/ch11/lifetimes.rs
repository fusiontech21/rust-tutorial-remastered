// Chapter 11: Lifetimes - Example Code

// Lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn get_part(&self) -> &'a str {
        self.part
    }
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world!");
    
    let result = longest(&s1, &s2);
    println!("Longest: {}", result);
    
    // Struct with lifetime
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("Excerpt: {}", excerpt.get_part());
    
    // Static lifetime
    let static_str: &'static str = "I have a static lifetime";
    println!("Static: {}", static_str);
}
