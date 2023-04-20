pub struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> Self {
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
}
