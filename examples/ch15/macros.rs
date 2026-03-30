// Chapter 15: Macros - Example Code

// Simple macro
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

// Macro with parameters
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// Macro with multiple rules
macro_rules! operation {
    (add, $a:expr, $b:expr) => {
        $a + $b
    };
    (sub, $a:expr, $b:expr) => {
        $a - $b
    };
    (mul, $a:expr, $b:expr) => {
        $a * $b
    };
}

// Macro with repetition
macro_rules! vec_of_strings {
    ($($x:expr),*) => {
        vec![$(String::from($x)),*]
    };
}

// Macro with multiple expressions
macro_rules! print_multiple {
    ($($x:expr),*) => {
        $(println!("{}", $x);)*
    };
}

fn main() {
    // Simple macro
    say_hello!();
    
    // Macro with parameters
    println!("add!(5, 3) = {}", add!(5, 3));
    
    // Multiple rules
    println!("operation!(add, 10, 5) = {}", operation!(add, 10, 5));
    println!("operation!(sub, 10, 5) = {}", operation!(sub, 10, 5));
    println!("operation!(mul, 10, 5) = {}", operation!(mul, 10, 5));
    
    // Repetition
    let v = vec_of_strings!("a", "b", "c");
    println!("{:?}", v);
    
    // Multiple expressions
    print_multiple!(1, "hello", 3.14);
}
