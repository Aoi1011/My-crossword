#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub a: i32,
    pub b: i32,
}

impl Point {
    // x.is_none() && y.is_none() -> the point at infinity
    pub fn new(x: Option<i32>, y: Option<i32>, a: i32, b: i32) -> Self {
        if x.is_some() && y.is_some() {
            if y.unwrap().pow(2) != x.unwrap().pow(3) + (a * x.unwrap()) + b {
                panic!("({}, {}) is not on the curve", x.unwrap(), y.unwrap());
            }
        }

        Self { a, b, x, y }
    }

    pub fn equal(&self, other: Option<Point>) -> bool {
        *self == other.unwrap()
    }

    pub fn not_equal(&self, other: Option<Point>) -> bool {
        *self != other.unwrap()
    }

    pub fn add(&self, other: Point) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        match (self.x, other.x) {
            (Some(self_x), Some(other_x)) if self_x == other_x && self.y != other.y => {
                return Self {
                    x: None,
                    y: None,
                    a: self.a,
                    b: self.b,
                }
            }
            (Some(_), None) => return *self,
            (None, Some(_)) => return other,
            _ => Self {
                x: self.x,
                y: self.y,
                a: self.a,
                b: self.b,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    #[test]
    fn test_equal() {
        let point1 = Point::new(Some(-1), Some(-1), 5, 7);
        let point2 = Point::new(Some(-1), Some(-1), 5, 7);

        assert!(point1.equal(Some(point2)));
    }

    #[test]
    fn test_add() {
        let point1 = Point::new(Some(-1), Some(-1), 5, 7);
        let point2 = Point::new(Some(-1), Some(1), 5, 7);
        let inf = Point::new(None, None, 5, 7);

        assert_eq!(point1.add(point2), inf);
    }
}
