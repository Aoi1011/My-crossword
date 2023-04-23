#[derive(PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub a: i32,
    pub b: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Self {
        if y.pow(2) != x.pow(3) + (a * x) + b {
            panic!("({}, {}) is not on the curve", x, y);
        }

        Self { a, b, x, y }
    }

    pub fn equal(&self, other: Option<Point>) -> bool {
        *self == other.unwrap()
    }

    pub fn not_equal(&self, other: Option<Point>) -> bool {
        *self != other.unwrap()
    }
}

pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::{add, Point};

    #[test]
    fn test_add() {
        let a = 1;
        let b = 1;

        assert_eq!(2, add(a, b));
    }

    #[test]
    fn test_equal() {
        let point1 = Point::new(-1, -1, 5, 7);
        let point2 = Point::new(-1, -1, 5, 7);

        assert!(point1.equal(Some(point2)));
    }
}
