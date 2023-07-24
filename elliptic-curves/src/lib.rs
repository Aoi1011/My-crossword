use std::{ops::Add, str::FromStr};

use ethereum_types::U256;
use finite_fields::FieldElement;

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
            FieldElement::new(U256::from_str(A).unwrap(), None)
        } else {
            a.unwrap()
        };

        let b = if b.is_none() {
            FieldElement::new(U256::from_str(B).unwrap(), None)
        } else {
            b.unwrap()
        };

        if let (Some(x_field), Some(y_field)) = (x.clone(), y.clone()) {
            if y_field.pow(U256::from(2))
                != x_field.pow(U256::from(3)).add(&a.mul(&x_field)).add(&b)
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

    pub fn rmul(&self, coefficient: u32) -> Self {
        let mut coef = coefficient;
        let mut current = self.clone();
        let mut result = Self::new(None, None, Some(self.a.clone()), Some(self.b.clone()));
        while coef != 0 {
            if coef & 1 == 1 {
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
                    .true_div(&other_x.sub(&self.x.clone().unwrap()));
                // x3 = s ^ 2 - x1 - x2
                let x3 = s
                    .pow(U256::from(2))
                    .sub(&self.x.unwrap())
                    .sub(&rhs.x.clone().unwrap());
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
                let s = FieldElement::new(U256::from_str("3").unwrap(), Some(x_prime.clone()))
                    .mul(&self_x.pow(U256::from(2)))
                    .add(&self.a)
                    .true_div(
                        &FieldElement::new(U256::from_str("2").unwrap(), Some(x_prime.clone()))
                            .mul(&self.y.clone().unwrap()),
                    );
                // x3 = s ^ 2 - 2x1
                let x3 = s.pow(U256::from(2)).sub(
                    &FieldElement::new(U256::from_str("2").unwrap(), Some(x_prime.clone()))
                        .mul(&self_x),
                );
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
    use std::str::FromStr;

    use ethereum_types::{U256, U512};
    use finite_fields::FieldElement;
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, Num};

    use crate::Point;

    macro_rules! u256 {
        ($val: expr) => {
            U256::from($val)
        };
    }

    #[test]
    fn test_equal() {
        let prime = Some(u256!(27));

        let field_element1 = FieldElement::new(u256!(-1), prime.clone());
        let field_element2 = FieldElement::new(u256!(-1), prime.clone());
        let a = FieldElement::new(u256!(5), prime.clone());
        let b = FieldElement::new(u256!(7), prime.clone());
        let point1 = Point::new(
            Some(field_element1.clone()),
            Some(field_element1),
            Some(a.clone()),
            Some(b.clone()),
        );
        let point2 = Point::new(
            Some(field_element2.clone()),
            Some(field_element2),
            Some(a),
            Some(b),
        );

        assert!(point1.equal(Some(point2)));
    }

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
    fn test_infinity() {
        const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
        let x = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
        let y = "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";

        let gx = BigUint::from_str_radix(x, 16);
        let gy = BigUint::from_str_radix(y, 16);
        // let field_y = FieldElement::new(U256::from_str(y).unwrap(), None);
        // let point_g = Point::new(Some(field_x.clone()), Some(field_y.clone()), None, None);

        let n = BigUint::from_str_radix(N, 16).unwrap();

        let c = n.clone() + n.clone();

        // let temp_x = FieldElement::new(
        //     field_x.num.checked_mul(n.clone()).unwrap() % field_x.prime,
        //     None,
        // );
        // let temp_y = FieldElement::new(field_y.num * n % field_y.prime, None);
        // let temp = Point::new(Some(temp_x), Some(temp_y), None, None);

        // assert!(point_g.x == temp.x && point_g.y != temp.y);
    }

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
}
