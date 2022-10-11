// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {   
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_test1() {
        let res = clamp(1, 10, 100);
        assert_eq!(res, 10, "test1: result should be 10");
    }
    
    #[test]
    fn clamp_test2() {
        let res = clamp(1000, 10, 20);
        assert_eq!(res, 20, "test2: result should be 20");
    }

    #[test]
    fn div_test1() {
        let res = div(10,0);
        assert_eq!(res, None,"div test1 : res should be None");
    }

    #[test]
    fn div_test2() {
        let res = div(10, 2);
        assert_eq!(res, Some(5),"div test2 result should be 5");
    }

    #[test]
    fn concat_test1() {
        let res = concat("a","b");
        assert_eq!(res, "ab".to_owned(),"concat test failed");
    }
}