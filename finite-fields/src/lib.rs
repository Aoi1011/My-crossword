use std::str;

use ibig::{ibig, IBig};

const P: &[u8; 64] = b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";

#[derive(Debug, PartialEq, Clone)]
pub struct FieldElement {
    pub num: IBig,
    pub prime: IBig,
}

impl FieldElement {
    pub fn new(num: IBig, prime: Option<IBig>) -> Self {
        let prime = if prime.is_none() {
            Self::from_bytes_radix(P, 16)
        } else {
            prime.unwrap()
        };

        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
    }

    pub fn repr(&self) {
        println!("FieldElement_{}({})", self.prime, self.num);
    }

    pub fn equal(&self, other: Option<FieldElement>) -> bool {
        if other.is_none() {
            return false;
        }

        let other = other.unwrap();
        self.eq(&other)
    }

    pub fn ne(&self, other: Option<FieldElement>) -> bool {
        if other.is_none() {
            return false;
        }

        let other = other.unwrap();
        self.num != other.num || self.prime != other.prime
    }

    pub fn add(&self, other: &FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("cannot add two numbers in different Fields");
        }

        let num = self.modulo(&(&self.num + &other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn sub(&self, other: &FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("cannot subtract two numbers in different Fields");
        }

        let num = self.modulo(&(&self.num - &other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn mul(&self, other: &FieldElement) -> Self {
        if self.prime != other.prime {
            panic!("cannot multiply two numbers in different Fields");
        }

        let num = self.modulo(&(&self.num * &other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn pow(&self, exp: u32) -> Self {
        let num = self.modulo(&self.num.pow(exp as usize));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn true_div(&self, other: Option<FieldElement>) -> Self {
        let other = other.unwrap();
        if self.prime != other.prime {
            panic!("cannot divide two numbers in different Fields");
        }

        // use Fermat's little theorem
        // self.num.pow(p-1) % p == 1
        // this means:
        // 1/n == pow(n, p-2, p) in Python
        let exp = other.prime.to_f32() - 2f32;
        let num_pow = other.pow(exp as u32);
        let result = self.num.clone() * num_pow.num;
        Self {
            num: result % self.prime.clone(),
            prime: self.prime.clone(),
        }
    }

    fn modulo(&self, b: &IBig) -> IBig {
        let result = b % self.prime.clone();
        if result < ibig!(0) {
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

    #[test]
    fn test_fieldelement_eq() {
        let element = FieldElement::new(ibig!(7), Some(ibig!(13)));
        let other = FieldElement::new(ibig!(6), Some(ibig!(13)));
        assert!(!element.equal(Some(other)));
    }

    #[test]
    fn test_fieldelement_ne() {
        let element = FieldElement::new(ibig!(7), Some(ibig!(13)));
        let other = FieldElement::new(ibig!(6), Some(ibig!(13)));
        assert!(element.ne(Some(other)));
    }

    #[test]
    fn test_calculate_modulo() {
        let prime = Some(ibig!(57));

        let field_element_1 = FieldElement::new(ibig!(44), prime.clone());
        assert_eq!(
            ibig!(20),
            field_element_1.modulo(&(&field_element_1.num + ibig!(33)))
        );

        let field_element_2 = FieldElement::new(ibig!(9), prime.clone());
        assert_eq!(
            ibig!(37),
            field_element_2.modulo(&(&field_element_2.num + ibig!(-29)))
        );

        let field_element_3 = FieldElement::new(ibig!(17), prime.clone());
        assert_eq!(
            ibig!(51),
            field_element_3.modulo(&(&field_element_3.num + ibig!(42) + ibig!(49)))
        );

        let field_element_4 = FieldElement::new(ibig!(52), prime.clone());
        assert_eq!(
            ibig!(41),
            field_element_4.modulo(&(&field_element_4.num + ibig!(-30) - ibig!(38))) % prime
        );
    }

    #[test]
    fn test_add() {
        let prime = Some(ibig!(13));
        let a = FieldElement::new(ibig!(7), prime.clone());
        let b = FieldElement::new(ibig!(12), prime.clone());
        let c = FieldElement::new(ibig!(6), prime);

        assert_eq!(a.add(Some(b)), c);
    }

    #[test]
    fn test_mul() {
        let prime = Some(ibig!(13));
        let a = FieldElement::new(ibig!(3), prime.clone());
        let b = FieldElement::new(ibig!(12), prime.clone());
        let c = FieldElement::new(ibig!(10), prime);

        assert_eq!(a.mul(Some(b)), c);
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
        let a = FieldElement::new(ibig!(7), Some(ibig!(13)));
        let b = FieldElement::new(ibig!(8), Some(ibig!(13)));

        assert_eq!(a.pow(9), b);
    }

    #[test]
    fn test_true_div() {
        let prime = Some(ibig!(19));
        let mut a = FieldElement::new(ibig!(2), prime.clone());
        let mut b = FieldElement::new(ibig!(7), prime.clone());
        let mut c = FieldElement::new(ibig!(3), prime.clone());

        assert_eq!(a.true_div(Some(b)), c);

        a = FieldElement::new(ibig!(7), prime.clone());
        b = FieldElement::new(ibig!(5), prime.clone());
        c = FieldElement::new(ibig!(9), prime);

        assert_eq!(a.true_div(Some(b)), c);
    }
}
