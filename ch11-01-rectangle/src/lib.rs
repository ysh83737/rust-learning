#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 6,
            height: 6,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 6,
            height: 6,
        };
        let smaller = Rectangle {
            width: 5,
            height: 5,
        };

        assert!(!smaller.can_hold(&larger));
    }
}