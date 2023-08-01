use std::ops::{Add, Div, Mul, Sub};

use num_bigint::{BigInt, BigUint};
use num_traits::{FromPrimitive, Num, One, Zero};

pub const P: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub num: BigUint,
    pub prime: BigUint,
}

impl FieldElement {
    pub fn new(num: BigUint, prime: Option<BigUint>) -> Self {
        let prime = if prime.is_none() {
            BigUint::from_str_radix(P, 16).unwrap()
        } else {
            prime.unwrap()
        };

        if num >= prime {
            panic!(
                "Num {} not in field range 0 to {}",
                num,
                prime - BigUint::one()
            );
        }
        Self { num, prime }
    }

    pub fn zero(prime: BigUint) -> Self {
        Self {
            num: BigUint::zero(),
            prime,
        }
    }

    pub fn get_prime(&self) -> BigUint {
        self.prime.clone()
    }

    pub fn get_number(&self) -> BigUint {
        self.num.clone()
    }

    pub fn to_the_power_of(&self, exponent: BigUint) -> Self {
        let exp = exponent % (self.prime.clone() - BigUint::one());
        let new_num = Self::mod_pow(&self.num, exp, &self.prime);
        Self {
            num: new_num,
            prime: self.prime.clone(),
        }
    }

    pub fn mod_pow(base: &BigUint, mut exp: BigUint, modulus: &BigUint) -> BigUint {
        if modulus == &BigUint::one() {
            return BigUint::zero();
        }

        let mut result = BigUint::one();
        let mut base = base % modulus;
        while exp > BigUint::zero() {
            if exp.clone() % (BigUint::one() + BigUint::one()) == BigUint::one() {
                result = result * base.clone() % modulus;
            }
            exp = exp / (BigUint::one() + BigUint::one());
            base = base.clone() * base.clone() % modulus;
        }

        result
    }

    pub fn ne(&self, other: &FieldElement) -> bool {
        self.num != other.num || self.prime != other.prime
    }

    pub fn pow(&self, exp: u32) -> Self {
        let num = self.modulo(&self.num.pow(exp));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    fn modulo(&self, b: &BigUint) -> BigUint {
        let result = b % self.prime.clone();
        if result < BigUint::zero() {
            result + self.prime.clone()
        } else {
            result
        }
    }

    pub fn sqrt(&self) -> Self {
        let p = BigUint::from_str_radix(P, 16).unwrap();
        self.to_the_power_of((p + BigUint::one()) / (BigUint::from_u8(4).unwrap()))
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Eq for FieldElement {}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("cannot add two numbers in different Fields");
        }

        let num = self.modulo(&(self.num.clone() + rhs.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("cannot subtract two numbers in different Fields");
        }

        let difference = BigInt::from(self.num.clone()) - BigInt::from(rhs.num.clone());
        let big_prime = BigInt::from(self.prime.clone());
        let remainder = difference % big_prime.clone();
        if remainder < BigInt::zero() {
            let new_num = remainder + big_prime;
            Self {
                num: new_num.try_into().unwrap(),
                prime: self.prime.clone(),
            }
        } else {
            Self {
                num: remainder.try_into().unwrap(),
                prime: self.prime.clone(),
            }
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("cannot multiply two numbers in different Fields");
        }

        let num = self.modulo(&(self.num.clone() * rhs.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("cannot divide two numbers in different Fields");
        }

        // use Fermat's little theorem
        // self.num.pow(p-1) % p == 1
        // this means:
        // 1/n == pow(n, p-2, p) in Python
        let exp = rhs.prime.clone() - (BigUint::one() + BigUint::one());
        let num_pow = rhs.to_the_power_of(exp);
        let result = self.num.clone() * num_pow.num;
        Self {
            num: result % self.prime.clone(),
            prime: self.prime.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    macro_rules! biguint {
        ($val: expr) => {
            BigUint::from_u32($val).unwrap()
        };
    }

    #[test]
    fn test_fieldelement_eq() {
        let element = FieldElement::new(biguint!(10), Some(biguint!(13)));
        let other = FieldElement::new(biguint!(6), Some(biguint!(13)));
        assert!(element != other);
    }

    #[test]
    fn test_fieldelement_ne() {
        let element = FieldElement::new(biguint!(6), Some(biguint!(13)));
        let other = FieldElement::new(biguint!(7), Some(biguint!(13)));
        assert!(element.ne(&other));
    }

    #[test]
    fn test_calculate_modulo() {
        let prime = Some(biguint!(57));

        let field_element_1 = FieldElement::new(biguint!(44), prime.clone());
        assert_eq!(
            biguint!(20),
            field_element_1.modulo(&(field_element_1.num.clone() + biguint!(33)))
        );

        let field_element_3 = FieldElement::new(biguint!(17), prime.clone());
        assert_eq!(
            biguint!(51),
            field_element_3.modulo(&(field_element_3.num.clone() + biguint!(42) + biguint!(49)))
        );
    }

    #[test]
    fn test_add() {
        let prime = Some(biguint!(13));
        let a = FieldElement::new(biguint!(7), prime.clone());
        let b = FieldElement::new(biguint!(12), prime.clone());
        let c = FieldElement::new(biguint!(6), prime);

        assert_eq!(a + b, c);
    }

    #[test]
    fn test_mul() {
        let prime = Some(biguint!(13));
        let a = FieldElement::new(biguint!(3), prime.clone());
        let b = FieldElement::new(biguint!(12), prime.clone());
        let c = FieldElement::new(biguint!(10), prime);

        assert_eq!(a * b, c);
    }

    #[test]
    fn test_example_pow() {
        let samples = Vec::from([7, 11, 13, 17]);
        let mut sets: Vec<Vec<u128>> = Vec::new();

        for p in samples {
            let pow_p: Vec<u128> = (1..=p - 1).map(|n: u128| n.pow(p as u32 - 1) % p).collect();
            sets.push(pow_p);
        }

        println!("{sets:?}");
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(biguint!(7), Some(biguint!(13)));
        let b = FieldElement::new(biguint!(8), Some(biguint!(13)));

        assert_eq!(a.to_the_power_of(biguint!(9)), b);
    }

    #[test]
    fn test_true_div() {
        let prime = Some(biguint!(19));
        let mut a = FieldElement::new(biguint!(2), prime.clone());
        let mut b = FieldElement::new(biguint!(7), prime.clone());
        let mut c = FieldElement::new(biguint!(3), prime.clone());

        assert_eq!(a / b, c);

        a = FieldElement::new(biguint!(7), prime.clone());
        b = FieldElement::new(biguint!(5), prime.clone());
        c = FieldElement::new(biguint!(9), prime);

        assert_eq!(a / b, c);
    }
}
