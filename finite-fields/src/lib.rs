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

        let num = self.modulo(other.num);
        Self {
            num,
            prime: self.prime,
        }
    }

    fn modulo(&self, b: i32) -> i32 {
        let result = (self.num + b) % self.prime;
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
        assert_eq!(20, field_element_1.modulo(33));

        let field_element_2 = FieldElement::new(9, prime);
        assert_eq!(37, field_element_2.modulo(-29));

        let field_element_3 = FieldElement::new(17, prime);
        assert_eq!(51, field_element_3.modulo(42 + 49));

        let field_element_4 = FieldElement::new(52, prime);
        assert_eq!(41, field_element_4.modulo(- 30 - 38) % prime);
    }
}
