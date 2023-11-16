fn prints_and_returns_10(a: i32) -> i32 {
    // for print values to appear we use cargo test -- --show-output
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // pass the name of the test to cargo test to only run the test
    // cargo test one_hundred
    // cargo test add will run the following two tests (matches both tests)
    // to only run ignored tests, use cargo test -- --ignored
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    #[ignore] // this test will be excluded
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}