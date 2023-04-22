#[derive(Debug, PartialEq)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Self { num, prime }
    }

    pub fn repr(&self) {
        println!("FieldElement_{}({})", self.prime, self.num);
    }

    pub fn eq(&self, other: Option<FieldElement>) -> bool {
        if other.is_none() {
            return false;
        }

        let other = other.unwrap();
        self.num == other.num && self.prime == other.prime
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

        let num = self.modulo(self.num + other.num);
        Self {
            num,
            prime: self.prime,
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

        let num = self.modulo(self.num - other.num);
        Self {
            num,
            prime: self.prime,
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

        let num = self.modulo(self.num * other.num);
        Self {
            num,
            prime: self.prime,
        }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let num = self.modulo(self.num.pow(exponent));
        Self {
            num,
            prime: self.prime,
        }
    }

    fn modulo(&self, b: i32) -> i32 {
        let result = b % self.prime;
        if result < 0 {
            result + self.prime
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fieldelement_eq() {
        let element = FieldElement::new(7, 13);
        let other = FieldElement::new(6, 13);
        assert!(!element.eq(Some(other)));
    }

    #[test]
    fn test_fieldelement_ne() {
        let element = FieldElement::new(7, 13);
        let other = FieldElement::new(6, 13);
        assert!(element.ne(Some(other)));
    }

    #[test]
    fn test_calculate_modulo() {
        let prime = 57;

        let field_element_1 = FieldElement::new(44, prime);
        assert_eq!(20, field_element_1.modulo(field_element_1.num + 33));

        let field_element_2 = FieldElement::new(9, prime);
        assert_eq!(37, field_element_2.modulo(field_element_2.num + -29));

        let field_element_3 = FieldElement::new(17, prime);
        assert_eq!(51, field_element_3.modulo(field_element_3.num + 42 + 49));

        let field_element_4 = FieldElement::new(52, prime);
        assert_eq!(
            41,
            field_element_4.modulo(field_element_4.num + -30 - 38) % prime
        );
    }

    #[test]
    fn test_add() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);

        assert_eq!(a.add(Some(b)), c);
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(10, 13);

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
}
