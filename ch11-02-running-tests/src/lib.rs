pub fn print_and_return_10(value: i32) -> i32 {
    println!("I got value {}", value);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let result = print_and_return_10(4);
        assert_eq!(result, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let result = print_and_return_10(8);
        assert_eq!(result, 5);
    }
}