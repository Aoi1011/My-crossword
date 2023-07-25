use std::ops::Add;

use finite_fields::FieldElement;
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Num, One, Zero};

const A: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const B: &str = "0000000000000000000000000000000000000000000000000000000000000007";
const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

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
        a: Option<FieldElement>,
        b: Option<FieldElement>,
    ) -> Self {
        let a = if a.is_none() {
            FieldElement::new(BigUint::from_str_radix(A, 16).unwrap(), None)
        } else {
            a.unwrap()
        };

        let b = if b.is_none() {
            FieldElement::new(BigUint::from_str_radix(B, 16).unwrap(), None)
        } else {
            b.unwrap()
        };

        if let (Some(x_field), Some(y_field)) = (x.clone(), y.clone()) {
            if y_field.to_the_power_of(BigUint::from_u8(2).unwrap())
                != x_field.to_the_power_of(BigUint::from_u8(3).unwrap())
                    + (a.clone() * x_field.clone())
                    + b.clone()
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

    pub fn rmul(&self, coefficient: BigUint) -> Self {
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = Self::new(None, None, Some(self.a.clone()), Some(self.b.clone()));
        while coef.clone() != BigUint::zero() {
            if coef.clone() % (BigUint::one() + BigUint::one()) == BigUint::one() {
                result = result + current.clone();
            }
            current = current.clone() + current.clone();
            coef >>= 1;
        }
        result
    }

    pub fn is_infinity(&self) -> bool {
        if self.x.is_none() && self.y.is_none() {
            return true;
        }
        false
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, rhs);
        }

        if self.x.is_none() {
            return rhs;
        }

        if rhs.x.is_none() {
            return self;
        }

        if self.x == rhs.x && self.y != rhs.y {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }

        if self.x != rhs.x {
            // s = (y2 - y1) / (x2 - x1)
            let s = (rhs.y.unwrap() - self.y.clone().unwrap())
                / (rhs.x.clone().unwrap() - self.x.clone().unwrap());
            // x3 = s ^ 2 - x1 - x2
            let x3 = s.to_the_power_of(BigUint::from_u8(2).unwrap())
                - self.x.clone().unwrap()
                - rhs.x.clone().unwrap();
            // y3 = s(x1 - x3) - y1
            let y3 = s * (self.x.unwrap() - x3.clone()) - self.y.unwrap();
            return Self {
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b,
            };
        }

        if self == rhs {
            let x_prime = self.x.clone().unwrap().prime;
            // s = (3x1 ^ 2 + a) / (2y1)
            let s = ((FieldElement::new(BigUint::from_u8(3).unwrap(), Some(x_prime.clone()))
                * self
                    .x
                    .clone()
                    .unwrap()
                    .to_the_power_of(BigUint::from_u8(2).unwrap()))
                + self.a.clone())
                / (FieldElement::new(BigUint::from_u8(2).unwrap(), Some(x_prime.clone()))
                    * (self.y.clone().unwrap()));
            // x3 = s ^ 2 - 2x1
            let x3 = s.to_the_power_of(BigUint::from_u8(2).unwrap())
                - (FieldElement::new(BigUint::from_u8(2).unwrap(), Some(x_prime.clone()))
                    * self.x.clone().unwrap());
            // y3 = s(x1 - x3) - y1
            let y3 = s * (self.x.unwrap() - x3.clone()) - self.y.unwrap();

            return Self {
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b,
            };
        }

        if self == rhs
            && self.y.unwrap()
                == FieldElement::zero(self.x.clone().unwrap().prime) * self.x.unwrap()
        {
            return Self {
                x: None,
                y: None,
                a: self.a,
                b: self.b,
            };
        }

        Self {
            x: rhs.x,
            y: rhs.y,
            a: rhs.a,
            b: rhs.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use finite_fields::FieldElement;
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, Num, Zero};

    use crate::{Point, N};

    macro_rules! biguint {
        ($val: expr) => {
            BigUint::from_u8($val).unwrap()
        };
    }

    // #[test]
    // fn test_equal() {
    //     let prime = Some(biguint!(27));

    //     let field_element1 = FieldElement::new(biguint!(-1), prime.clone());
    //     let field_element2 = FieldElement::new(biguint!(-1), prime.clone());
    //     let a = FieldElement::new(biguint!(5), prime.clone());
    //     let b = FieldElement::new(biguint!(7), prime.clone());
    //     let point1 = Point::new(
    //         Some(field_element1.clone()),
    //         Some(field_element1),
    //         Some(a.clone()),
    //         Some(b.clone()),
    //     );
    //     let point2 = Point::new(
    //         Some(field_element2.clone()),
    //         Some(field_element2),
    //         Some(a),
    //         Some(b),
    //     );

    //     assert!(point1.equal(Some(point2)));
    // }

    // #[test]
    // fn test_add() {
    //     let mut prime = Some(ibig!(27));

    //     let field_element1 = FieldElement::new(ibig!(-1), prime.clone());
    //     let field_element2 = FieldElement::new(ibig!(1), prime.clone());
    //     let a = FieldElement::new(ibig!(5), prime.clone());
    //     let b = FieldElement::new(ibig!(7), prime.clone());
    //     let point1 = Point::new(
    //         Some(field_element1.clone()),
    //         Some(field_element1.clone()),
    //         Some(a.clone()),
    //         Some(b.clone()),
    //     );
    //     let point2 = Point::new(
    //         Some(field_element1),
    //         Some(field_element2),
    //         Some(a.clone()),
    //         Some(b.clone()),
    //     );
    //     let inf = Point::new(None, None, Some(a), Some(b));

    //     assert_eq!(point1 + point2, inf);

    //     // x1 != x2
    //     prime = Some(ibig!(223));

    //     let a = FieldElement::new(ibig!(0), prime.clone());
    //     let b = FieldElement::new(ibig!(7), prime.clone());
    //     let field_element1 = FieldElement::new(ibig!(170), prime.clone());
    //     let field_element2 = FieldElement::new(ibig!(142), prime.clone());
    //     let field_element3 = FieldElement::new(ibig!(60), prime.clone());
    //     let field_element4 = FieldElement::new(ibig!(139), prime.clone());
    //     let field_element5 = FieldElement::new(ibig!(220), prime.clone());
    //     let field_element6 = FieldElement::new(ibig!(181), prime.clone());
    //     let point1 = Point::new(
    //         Some(field_element1),
    //         Some(field_element2),
    //         Some(a.clone()),
    //         Some(b.clone()),
    //     );
    //     let point2 = Point::new(
    //         Some(field_element3),
    //         Some(field_element4),
    //         Some(a.clone()),
    //         Some(b.clone()),
    //     );
    //     let point3 = Point::new(Some(field_element5), Some(field_element6), Some(a), Some(b));

    //     assert_eq!(point1 + point2, point3);
    // }

    #[test]
    fn test_secp256k1() {
        let x = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let y = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

        let gx = BigUint::from_str_radix(x, 16).unwrap();
        let gy = BigUint::from_str_radix(y, 16).unwrap();

        let p = BigUint::from_u8(2).unwrap().pow(256_u32)
            - BigUint::from_u8(2).unwrap().pow(32_u32)
            - BigUint::from_u32(977).unwrap();

        assert_eq!(
            gy.pow(2) % &p,
            (gx.pow(3) + BigUint::from_u32(7).unwrap()) % &p
        );
    }

    #[test]
    fn test_infinity() {
        let str_x = "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let str_y = "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

        let gx = BigUint::from_str_radix(str_x, 16).unwrap();
        let gy = BigUint::from_str_radix(str_y, 16).unwrap();

        let p = BigUint::from_u8(2).unwrap().pow(256_u32)
            - BigUint::from_u8(2).unwrap().pow(32_u32)
            - BigUint::from_u32(977).unwrap();

        let n = BigUint::from_str_radix(N, 16).unwrap();

        let x = FieldElement::new(gx, Some(p.clone()));
        let y = FieldElement::new(gy, Some(p.clone()));

        let seven = FieldElement::new(BigUint::from_u8(7).unwrap(), None);
        let zero = FieldElement::new(BigUint::zero(), None);

        let g = Point::new(Some(x), Some(y), Some(zero), Some(seven));

        let ng = g.rmul(n);

        assert!(ng.is_infinity());
    }
}
