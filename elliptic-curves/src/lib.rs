use std::ops::Add;

use finite_fields::FieldElement;
use ibig::ibig;


#[derive(Debug, PartialEq, Clone)]
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
        if let (Some(x_field), Some(y_field)) = (x.clone(), y.clone()) {
            if y_field.pow(2) != x_field.pow(3).add(&a.mul(&x_field)).add(&b) {
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
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = Self::new(None, None, self.a.clone(), self.b.clone());
        while coef != 0 {
            if coef & 1 == 1 {
                result = result + current.clone();
            }
            current = current.clone() + current.clone();
            coef >>= 1;
        }
        result
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, rhs);
        }

        match (self.x.clone(), rhs.x.clone()) {
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
                    .sub(&self.y.clone().unwrap())
                    .true_div(Some(other_x.sub(&self.x.clone().unwrap())));
                // x3 = s ^ 2 - x1 - x2
                let x3 = s.pow(2).sub(&self.x.unwrap()).sub(&rhs.x.clone().unwrap());
                // y3 = s(x1 - x3) - y1
                let y3 = s.mul(&self_x.sub(&x3)).sub(&self.y.unwrap());
                Self {
                    x: Some(x3),
                    y: Some(y3),
                    a: self.a,
                    b: self.b,
                }
            }
            (Some(self_x), Some(other_x)) if self_x == other_x && self.y == rhs.y => {
                let x_prime = self_x.clone().prime;
                // s = (3x1 ^ 2 + a) / (2y1)
                let s = FieldElement::new(ibig!(3), x_prime.clone())
                    .mul(&self_x.pow(2))
                    .add(&self.a)
                    .true_div(Some(
                        FieldElement::new(ibig!(2), x_prime.clone()).mul(&self.y.clone().unwrap()),
                    ));
                // x3 = s ^ 2 - 2x1
                let x3 = s
                    .pow(2)
                    .sub(&FieldElement::new(ibig!(2), x_prime.clone()).mul(&self_x));
                // y3 = s(x1 - x3) - y1
                let y3 = s.mul(&self_x.sub(&x3)).sub(&self.y.unwrap());

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
    use ibig::ibig;

    use crate::Point;

    #[test]
    fn test_equal() {
        let field_element1 = FieldElement::new(ibig!(-1), ibig!(27));
        let field_element2 = FieldElement::new(ibig!(-1), ibig!(27));
        let a = FieldElement::new(ibig!(5), ibig!(27));
        let b = FieldElement::new(ibig!(7), ibig!(27));
        let point1 = Point::new(
            Some(field_element1.clone()),
            Some(field_element1),
            a.clone(),
            b.clone(),
        );
        let point2 = Point::new(Some(field_element2.clone()), Some(field_element2), a, b);

        assert!(point1.equal(Some(point2)));
    }

    #[test]
    fn test_add() {
        let field_element1 = FieldElement::new(ibig!(-1), ibig!(27));
        let field_element2 = FieldElement::new(ibig!(1), ibig!(27));
        let a = FieldElement::new(ibig!(5), ibig!(27));
        let b = FieldElement::new(ibig!(7), ibig!(27));
        let point1 = Point::new(
            Some(field_element1.clone()),
            Some(field_element1.clone()),
            a.clone(),
            b.clone(),
        );
        let point2 = Point::new(
            Some(field_element1),
            Some(field_element2),
            a.clone(),
            b.clone(),
        );
        let inf = Point::new(None, None, a, b);

        assert_eq!(point1 + point2, inf);

        // x1 != x2
        let prime = ibig!(223);
        let a = FieldElement::new(ibig!(0), prime.clone());
        let b = FieldElement::new(ibig!(7), prime.clone());
        let field_element1 = FieldElement::new(ibig!(170), prime.clone());
        let field_element2 = FieldElement::new(ibig!(142), prime.clone());
        let field_element3 = FieldElement::new(ibig!(60), prime.clone());
        let field_element4 = FieldElement::new(ibig!(139), prime.clone());
        let field_element5 = FieldElement::new(ibig!(220), prime.clone());
        let field_element6 = FieldElement::new(ibig!(181), prime.clone());
        let point1 = Point::new(
            Some(field_element1),
            Some(field_element2),
            a.clone(),
            b.clone(),
        );
        let point2 = Point::new(
            Some(field_element3),
            Some(field_element4),
            a.clone(),
            b.clone(),
        );
        let point3 = Point::new(Some(field_element5), Some(field_element6), a, b);

        assert_eq!(point1 + point2, point3);
    }
}
