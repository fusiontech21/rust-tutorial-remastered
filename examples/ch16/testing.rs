// Chapter 16: Testing - Example Code

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, -3), -5);
    }
    
    #[test]
    fn test_add_mixed() {
        assert_eq!(add(-2, 3), 1);
    }
    
    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(10, 2), Ok(5));
    }
    
    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_panic() {
        divide(10, 0).unwrap();
    }
    
    #[test]
    fn test_multiple_cases() {
        let test_cases = vec![
            (0, 0, 0),
            (1, 2, 3),
            (-1, 1, 0),
            (100, 200, 300),
        ];
        
        for (a, b, expected) in test_cases {
            assert_eq!(add(a, b), expected, "add({}, {}) failed", a, b);
        }
    }
}

fn main() {
    println!("Run 'cargo test' to run tests!");
}
