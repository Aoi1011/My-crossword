use std::ops::Add;

use finite_fields::FieldElement;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Point {
    // x.is_none() && y.is_none() -> the point at infinity
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Self {
        if let (Some(x_field), Some(y_field)) = (x, y) {
            if y_field.pow(2) != x_field.pow(3).add(Some(a.mul(Some(x_field)))).add(Some(b)) {
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

    pub fn rmul(&self, coefficient: u32) -> Self {
        let mut product = Self::new(None, None, self.a, self.b);
        for _ in 0..coefficient {
            product = product + *self;
        }
        product
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, rhs);
        }

        match (self.x, rhs.x) {
            (Some(self_x), Some(other_x)) if self_x == other_x && self.y != rhs.y => Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            },
            (Some(self_x), Some(other_x)) if self_x != other_x => {
                // s = (y2 - y1) / (x2 - x1)
                let s = rhs
                    .y
                    .unwrap()
                    .sub(self.y)
                    .true_dev(Some(other_x.sub(self.x)));
                // x3 = s ^ 2 - x1 - x2
                let x3 = s.pow(2).sub(self.x).sub(rhs.x);
                // y3 = s(x1 - x3) - y1
                let y3 = s.mul(Some(self_x.sub(Some(x3)))).sub(self.y);
                Self {
                    x: Some(x3),
                    y: Some(y3),
                    a: self.a,
                    b: self.b,
                }
            }
            (Some(self_x), Some(other_x)) if self_x == other_x && self.y == rhs.y => {
                // s = (3x1 ^ 2 + a) / (2y1)
                let s = FieldElement::new(3, self_x.prime)
                    .mul(Some(self_x.pow(2)))
                    .add(Some(self.a))
                    .true_dev(Some(FieldElement::new(2, self_x.prime).mul(self.y)));
                // x3 = s ^ 2 - 2x1
                let x3 = s
                    .pow(2)
                    .sub(Some(FieldElement::new(2, self_x.prime).mul(Some(self_x))));
                // y3 = s(x1 - x3) - y1
                let y3 = s.mul(Some(self_x.sub(Some(x3)))).sub(self.y);

                Self {
                    x: Some(x3),
                    y: Some(y3),
                    a: self.a,
                    b: self.b,
                }
            }
            (Some(_), None) => self,
            (None, Some(_)) => rhs,
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
    use finite_fields::FieldElement;

    use crate::Point;

    #[test]
    fn test_equal() {
        let field_element1 = FieldElement::new(-1, 27);
        let field_element2 = FieldElement::new(-1, 27);
        let a = FieldElement::new(5, 27);
        let b = FieldElement::new(7, 27);
        let point1 = Point::new(Some(field_element1), Some(field_element1), a, b);
        let point2 = Point::new(Some(field_element2), Some(field_element2), a, b);

        assert!(point1.equal(Some(point2)));
    }

    #[test]
    fn test_add() {
        let field_element1 = FieldElement::new(-1, 27);
        let field_element2 = FieldElement::new(1, 27);
        let a = FieldElement::new(5, 27);
        let b = FieldElement::new(7, 27);
        let point1 = Point::new(Some(field_element1), Some(field_element1), a, b);
        let point2 = Point::new(Some(field_element1), Some(field_element2), a, b);
        let inf = Point::new(None, None, a, b);

        assert_eq!(point1 + point2, inf);

        // x1 != x2
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let field_element1 = FieldElement::new(170, prime);
        let field_element2 = FieldElement::new(142, prime);
        let field_element3 = FieldElement::new(60, prime);
        let field_element4 = FieldElement::new(139, prime);
        let field_element5 = FieldElement::new(220, prime);
        let field_element6 = FieldElement::new(181, prime);
        let point1 = Point::new(Some(field_element1), Some(field_element2), a, b);
        let point2 = Point::new(Some(field_element3), Some(field_element4), a, b);
        let point3 = Point::new(Some(field_element5), Some(field_element6), a, b);

        assert_eq!(point1 + point2, point3);
    }
}
