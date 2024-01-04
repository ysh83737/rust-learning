pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(add(3, 1), 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail!");
    }

    #[test]
    fn another_fail() {
        assert_eq!(add(3, 3), 7);
    }

    #[test]
    fn wrong_result() {
        assert_ne!(add(3, 3), 7);
    }
}
