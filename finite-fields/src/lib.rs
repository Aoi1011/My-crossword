use ibig::{UBig, IBig, ibig};

#[derive(Debug, PartialEq, Clone)]
pub struct FieldElement {
    pub num: IBig,
    pub prime: IBig,
}

impl FieldElement {
    pub fn new(num: IBig, prime: IBig) -> Self {
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

    pub fn add(&self, other: Option<FieldElement>) -> Self {
        if other.is_none() {
            panic!("other is none");
        }

        let other = other.unwrap();
        if self.prime != other.prime {
            panic!("cannot add two numbers in different Fields");
        }

        let num = self.modulo(&(&self.num + &other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn sub(&self, other: Option<FieldElement>) -> Self {
        if other.is_none() {
            panic!("other is none");
        }

        let other = other.unwrap();
        if self.prime != other.prime {
            panic!("cannot subtract two numbers in different Fields");
        }

        let num = self.modulo(&(&self.num - &other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    pub fn mul(&self, other: Option<FieldElement>) -> Self {
        if other.is_none() {
            panic!("other is none");
        }

        let other = other.unwrap();
        if self.prime != other.prime {
            panic!("cannot multiply two numbers in different Fields");
        }

        let num = self.modulo(&(&self.num * other.num));
        Self {
            num,
            prime: self.prime.clone(),
        }
    }

    // pub fn pow(&self, exponent: i32) -> Self {
    //     let mut n = exponent;
    //     while n < 0 {
    //         n += self.prime - 1;
    //     }
    //     let num = self.modulo(self.num.pow(n as u32));
    //     Self {
    //         num,
    //         prime: self.prime,
    //     }
    // }

    //pub fn true_dev(&self, other: Option<FieldElement>) -> Self {
    //    let other = other.unwrap();
    //    if self.prime != other.prime {
    //        panic!("cannot divide two numbers in different Fields");
    //    }

    //    // use Fermat's little theorem
    //    // self.num.pow(p-1) % p == 1
    //    // this means:
    //    // 1/n == pow(n, p-2, p) in Python
    //    let result = self.num.clone() * self.pow_mod(other.num);
    //    Self {
    //        num: result % self.prime.clone(),
    //        prime: self.prime.clone(),
    //    }
    //}

    fn modulo(&self, b: &IBig) -> IBig {
        let result = b % self.prime.clone();
        if result < ibig!(0) {
            result + self.prime.clone()
        } else {
            result
        }
    }

    //fn pow_mod(&self, base: IBig) -> IBig {
    //    let mut result = ibig!(1);
    //    let mut base = base;
    //    let mut exponent = self.prime - 2;
    //    let modules = self.prime;

    //    while exponent > ibig!(0) {
    //        if exponent % 2 == 1 {
    //            result = (result * base) % modules;
    //        }
    //        base = (base * base) % modules;
    //        exponent /= 2;
    //    }
    //    result
    //}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fieldelement_eq() {
        let element = FieldElement::new(ibig!(7), ibig!(13));
        let other = FieldElement::new(ibig!(6), ibig!(13));
        assert!(!element.equal(Some(other)));
    }

    //#[test]
    //fn test_fieldelement_ne() {
    //    let element = FieldElement::new(7, 13);
    //    let other = FieldElement::new(6, 13);
    //    assert!(element.ne(Some(other)));
    //}

    //#[test]
    //fn test_calculate_modulo() {
    //    let prime = 57;

    //    let field_element_1 = FieldElement::new(44, prime);
    //    assert_eq!(20, field_element_1.modulo(field_element_1.num + 33));

    //    let field_element_2 = FieldElement::new(9, prime);
    //    assert_eq!(37, field_element_2.modulo(field_element_2.num + -29));

    //    let field_element_3 = FieldElement::new(17, prime);
    //    assert_eq!(51, field_element_3.modulo(field_element_3.num + 42 + 49));

    //    let field_element_4 = FieldElement::new(52, prime);
    //    assert_eq!(
    //        41,
    //        field_element_4.modulo(field_element_4.num + -30 - 38) % prime
    //    );
    //}

    //#[test]
    //fn test_add() {
    //    let a = FieldElement::new(7, 13);
    //    let b = FieldElement::new(12, 13);
    //    let c = FieldElement::new(6, 13);

    //    assert_eq!(a.add(Some(b)), c);
    //}

    //#[test]
    //fn test_mul() {
    //    let a = FieldElement::new(3, 13);
    //    let b = FieldElement::new(12, 13);
    //    let c = FieldElement::new(10, 13);

    //    assert_eq!(a.mul(Some(b)), c);
    //}

    //#[test]
    //fn test_example_pow() {
    //    let samples = Vec::from([7, 11, 13, 17]);
    //    let mut sets: Vec<Vec<u128>> = Vec::new();

    //    for p in samples {
    //        let pow_p: Vec<u128> = (1..=p - 1).map(|n: u128| n.pow(p as u32 - 1) % p).collect();
    //        sets.push(pow_p);
    //    }

    //    println!("{sets:?}");
    //}

    //#[test]
    //fn test_pow() {
    //    let a = FieldElement::new(7, 13);
    //    let b = FieldElement::new(8, 13);

    //    assert_eq!(a.pow(-3), b);
    //}

    //#[test]
    //fn test_true_dev() {
    //    let mut a = FieldElement::new(2, 19);
    //    let mut b = FieldElement::new(7, 19);
    //    let mut c = FieldElement::new(3, 19);

    //    assert_eq!(a.true_dev(Some(b)), c);

    //    a = FieldElement::new(7, 19);
    //    b = FieldElement::new(5, 19);
    //    c = FieldElement::new(9, 19);

    //    assert_eq!(a.true_dev(Some(b)), c);
    //}
}
