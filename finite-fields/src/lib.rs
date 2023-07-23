use std::str::{self, FromStr};

use ethers_core::types::I256;
use ibig::IBig;

const P: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";

#[derive(Debug, PartialEq, Clone)]
pub struct FieldElement {
    pub num: I256,
    pub prime: I256,
}

impl FieldElement {
    pub fn new(num: I256, prime: Option<I256>) -> Self {
        let prime = if prime.is_none() {
            I256::from_str(P).unwrap()
        } else {
            prime.unwrap()
        };

        if num >= prime {
            panic!(
                "Num {} not in field range 0 to {}",
                num,
                prime - I256::one()
            );
        }
        Self { num, prime }
    }

    pub fn repr(&self) {
        println!("FieldElement_{}({})", self.prime, self.num);
    }

    pub fn equal(&self, other: &FieldElement) -> bool {
        self.eq(other)
    }

    pub fn ne(&self, other: &FieldElement) -> bool {
        self.num != other.num || self.prime != other.prime
    }

    pub fn add(&self, other: &FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("cannot add two numbers in different Fields");
        }

        let num = self.modulo(&(self.num + other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn sub(&self, other: &FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("cannot subtract two numbers in different Fields");
        }

        let num = self.modulo(&(self.num - other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn mul(&self, other: &FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("cannot multiply two numbers in different Fields");
        }

        let num = self.modulo(&(self.num * other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let num = self.modulo(&self.num.pow(exp));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn true_div(&self, other: &FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("cannot divide two numbers in different Fields");
        }

        // use Fermat's little theorem
        // self.num.pow(p-1) % p == 1
        // this means:
        // 1/n == pow(n, p-2, p) in Python
        let exp = other.prime - (I256::one() + I256::one());
        let num_pow = other.pow(exp.as_u32());
        let result = self.num.clone() * num_pow.num;
        Self {
            num: result % self.prime.clone(),
            prime: self.prime.clone(),
        }
    }

    fn modulo(&self, b: &I256) -> I256 {
        let result = *b % self.prime.clone();
        if result < I256::zero() {
            result + self.prime.clone()
        } else {
            result
        }
    }

    pub fn from_bytes_radix(buf: &[u8], radix: u32) -> IBig {
        let s = str::from_utf8(buf).ok().unwrap();
        IBig::from_str_radix(s, radix).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! i256 {
        ($val: expr) => {
            I256::from($val)
        };
    }

    #[test]
    fn test_fieldelement_eq() {
        let element = FieldElement::new(i256!(10), Some(i256!(13)));
        let other = FieldElement::new(i256!(6), Some(i256!(13)));
        assert!(!element.equal(&other));
    }

    #[test]
    fn test_fieldelement_ne() {
        let element = FieldElement::new(i256!(6), Some(i256!(13)));
        let other = FieldElement::new(i256!(7), Some(i256!(13)));
        assert!(element.ne(&other));
    }

    #[test]
    fn test_calculate_modulo() {
        let prime = Some(i256!(57));

        let field_element_1 = FieldElement::new(i256!(44), prime.clone());
        assert_eq!(
            i256!(20),
            field_element_1.modulo(&(field_element_1.num + i256!(33)))
        );

        let field_element_2 = FieldElement::new(i256!(9), prime.clone());
        assert_eq!(
            i256!(37),
            field_element_2.modulo(&(field_element_2.num + i256!(-29)))
        );

        let field_element_3 = FieldElement::new(i256!(17), prime.clone());
        assert_eq!(
            i256!(51),
            field_element_3.modulo(&(field_element_3.num + i256!(42) + i256!(49)))
        );

        let field_element_4 = FieldElement::new(i256!(52), prime.clone());
        assert_eq!(
            i256!(41),
            field_element_4.modulo(&(field_element_4.num + i256!(-30) - i256!(38)))
        );
    }

    #[test]
    fn test_add() {
        let prime = Some(i256!(13));
        let a = FieldElement::new(i256!(7), prime.clone());
        let b = FieldElement::new(i256!(12), prime.clone());
        let c = FieldElement::new(i256!(6), prime);

        assert_eq!(a.add(&b), c);
    }

    #[test]
    fn test_mul() {
        let prime = Some(i256!(13));
        let a = FieldElement::new(i256!(3), prime.clone());
        let b = FieldElement::new(i256!(12), prime.clone());
        let c = FieldElement::new(i256!(10), prime);

        assert_eq!(a.mul(&b), c);
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
        let a = FieldElement::new(i256!(7), Some(i256!(13)));
        let b = FieldElement::new(i256!(8), Some(i256!(13)));

        assert_eq!(a.pow(9), b);
    }

    #[test]
    fn test_true_div() {
        let prime = Some(i256!(19));
        let mut a = FieldElement::new(i256!(2), prime.clone());
        let mut b = FieldElement::new(i256!(7), prime.clone());
        let mut c = FieldElement::new(i256!(3), prime.clone());

        assert_eq!(a.true_div(&b), c);

        a = FieldElement::new(i256!(7), prime.clone());
        b = FieldElement::new(i256!(5), prime.clone());
        c = FieldElement::new(i256!(9), prime);

        assert_eq!(a.true_div(&b), c);
    }
}
