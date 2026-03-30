// Chapter 3: Data Types - Example Code

fn main() {
    // Integer types
    let a: i8 = 127;
    let b: i16 = 32_767;
    let c: i32 = 2_147_483_647;
    let d: i64 = 9_223_372_036_854_775_807;
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let f: u8 = 255;
    
    println!("Integers: {}, {}, {}, {}, {}, {}", a, b, c, d, e, f);
    
    // Floating point
    let float1: f32 = 3.14159;
    let float2: f64 = 3.14159265358979;
    println!("Floats: {}, {}", float1, float2);
    
    // Boolean
    let is_rust_great: bool = true;
    println!("Is Rust great? {}", is_rust_great);
    
    // Character
    let char1: char = 'A';
    let emoji: char = '🦀';
    println!("Char: {}, Emoji: {}", char1, emoji);
    
    // Tuple
    let tuple: (i32, f64, &str) = (50, 3.14, "hello");
    println!("Tuple: {:?}", tuple);
    
    // Destructure tuple
    let (x, y, z) = tuple;
    println!("Destructured: x={}, y={}, z={}", x, y, z);
    
    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    
    // Array with repeated values
    let zeros = [0; 10];
    println!("Zeros: {:?}", zeros);
    
    // Array access
    println!("First element: {}", arr[0]);
    println!("Array length: {}", arr.len());
}
