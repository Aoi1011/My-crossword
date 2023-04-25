use finite_fields::FieldElement;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: i32,
    pub b: i32,
}

impl Point {
    // x.is_none() && y.is_none() -> the point at infinity
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: i32, b: i32) -> Self {
        if let (Some(x_field), Some(y_field)) = (x, y) {
            if y_field.pow(2)
                != x_field
                    .pow(3)
                    .add(Some(x_field.mul(Some(FieldElement {
                        num: a,
                        prime: x_field.prime,
                    }))))
                    .add(Some(FieldElement {
                        num: b,
                        prime: x_field.prime,
                    }))
            {
                panic!("({:?}, {:?}) is not on the curve", x_field, y_field);
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
            (Some(self_x), Some(other_x)) if self_x == other_x && self.y != other.y => Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            },
            (Some(self_x), Some(other_x)) if self_x != other_x => {
                let s = other
                    .y
                    .unwrap()
                    .sub(Some(self.y.unwrap()))
                    .true_dev(Some(other.x.unwrap().sub(Some(self.x.unwrap()))));
                let x3 = s.pow(2) - self.x.unwrap() - other.x.unwrap();
                let y3 = s * (self.x.unwrap() - x3) - self.y.unwrap();
                Self {
                    x: Some(x3),
                    y: Some(y3),
                    a: self.a,
                    b: self.b,
                }
            }
            (Some(self_x), Some(other_x)) if self_x == other_x && self.y == other.y => {
                let s = (3 * self_x.pow(2) + self.a) / (2 * self.y.unwrap());
                let x3 = s.pow(2) - 2 * self_x;
                let y3 = s * (self_x - x3) - self.y.unwrap();
                Self {
                    x: Some(x3),
                    y: Some(y3),
                    a: self.a,
                    b: self.b,
                }
            }
            (Some(_), None) => *self,
            (None, Some(_)) => other,
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
        let mut point1 = Point::new(Some(-1), Some(-1), 5, 7);
        let mut point2 = Point::new(Some(-1), Some(1), 5, 7);
        let inf = Point::new(None, None, 5, 7);

        assert_eq!(point1.add(point2), inf);

        // x1 != x2
        point1 = Point::new(Some(2), Some(5), 5, 7);
        point2 = Point::new(Some(-1), Some(-1), 5, 7);
        let point3 = Point::new(Some(3), Some(-7), 5, 7);

        assert_eq!(point1.add(point2), point3);

        // p1 == p2
        point1 = Point::new(Some(-1), Some(-1), 5, 7);
        point2 = Point::new(Some(-1), Some(-1), 5, 7);
        let point3 = Point::new(Some(18), Some(77), 5, 7);

        assert_eq!(point1.add(point2), point3);
    }
}
