use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 2} + Point { x: 2, y: 1 },
        Point { x: 3, y: 3 },
    );

    assert_eq!(
        Millimeters(345) + Meters(1),
        Millimeters(1345),
    );
}

#[derive(Debug, PartialEq)]
struct Meters(u32);
#[derive(Debug, PartialEq)]
struct Millimeters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Self::Output {
        Millimeters(self.0 + other.0 * 1000)
    }
}

