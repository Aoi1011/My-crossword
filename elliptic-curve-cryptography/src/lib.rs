pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use finite_fields::FieldElement;

    #[test]
    fn test_exercise1() {
        let prime = 223;

        let vec: Vec<(FieldElement, FieldElement)> = vec![
            (FieldElement::new(192, prime), FieldElement::new(105, prime)),
            (FieldElement::new(17, prime), FieldElement::new(56, prime)),
            (FieldElement::new(200, prime), FieldElement::new(119, prime)),
            (FieldElement::new(1, prime), FieldElement::new(193, prime)),
            (FieldElement::new(42, prime), FieldElement::new(99, prime)),
        ];

        let mut result = vec![];
        for point in vec {
            let x = point.0.pow(3).add(Some(FieldElement { num: 7, prime }));
            let y = point.1.pow(2);

            result.push(x == y);
        }

        assert_eq!(vec![true, true, false, true, false], result);
    }
}
