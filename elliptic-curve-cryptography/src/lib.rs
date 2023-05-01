pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use elliptic_curves::Point;
    use finite_fields::FieldElement;

    #[test]
    fn test_elliptic_curve_cryptographic_exercise1() {
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

    #[test]
    fn test_on_curve() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let mut field_1 = FieldElement::new(170, prime);
        let mut field_2 = FieldElement::new(142, prime);
        let mut field_3 = FieldElement::new(60, prime);
        let mut field_4 = FieldElement::new(139, prime);
        let mut field_5 = FieldElement::new(220, prime);
        let mut field_6 = FieldElement::new(181, prime);

        let mut point_1 = Point::new(Some(field_1), Some(field_2), a, b);
        let mut point_2 = Point::new(Some(field_3), Some(field_4), a, b);
        let mut point_3 = Point::new(Some(field_5), Some(field_6), a, b);

        assert_eq!(point_1 + point_2, point_3);

        field_1 = FieldElement::new(47, prime);
        field_2 = FieldElement::new(71, prime);
        field_3 = FieldElement::new(17, prime);
        field_4 = FieldElement::new(56, prime);
        field_5 = FieldElement::new(215, prime);
        field_6 = FieldElement::new(68, prime);

        point_1 = Point::new(Some(field_1), Some(field_2), a, b);
        point_2 = Point::new(Some(field_3), Some(field_4), a, b);
        point_3 = Point::new(Some(field_5), Some(field_6), a, b);

        assert_eq!(point_1 + point_2, point_3);

        field_1 = FieldElement::new(143, prime);
        field_2 = FieldElement::new(98, prime);
        field_3 = FieldElement::new(76, prime);
        field_4 = FieldElement::new(66, prime);
        field_5 = FieldElement::new(47, prime);
        field_6 = FieldElement::new(71, prime);

        point_1 = Point::new(Some(field_1), Some(field_2), a, b);
        point_2 = Point::new(Some(field_3), Some(field_4), a, b);
        point_3 = Point::new(Some(field_5), Some(field_6), a, b);

        assert_eq!(point_1 + point_2, point_3);
    }

    #[test]
    fn test_on_curve_scalar() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let mut field_1 = FieldElement::new(192, prime);
        let mut field_2 = FieldElement::new(105, prime);
        let mut field_3 = FieldElement::new(49, prime);
        let mut field_4 = FieldElement::new(71, prime);

        let mut point_1 = Point::new(Some(field_1), Some(field_2), a, b);
        let mut point_2 = Point::new(Some(field_3), Some(field_4), a, b);

        assert_eq!(point_1 + point_1, point_2);

        field_1 = FieldElement::new(143, prime);
        field_2 = FieldElement::new(98, prime);
        field_3 = FieldElement::new(64, prime);
        field_4 = FieldElement::new(168, prime);

        point_1 = Point::new(Some(field_1), Some(field_2), a, b);
        point_2 = Point::new(Some(field_3), Some(field_4), a, b);

        assert_eq!(point_1 + point_1, point_2);

        field_1 = FieldElement::new(47, prime);
        field_2 = FieldElement::new(71, prime);
        field_3 = FieldElement::new(36, prime);
        field_4 = FieldElement::new(111, prime);

        point_1 = Point::new(Some(field_1), Some(field_2), a, b);
        point_2 = Point::new(Some(field_3), Some(field_4), a, b);

        assert_eq!(point_1 + point_1, point_2);

        field_3 = FieldElement::new(194, prime);
        field_4 = FieldElement::new(51, prime);
        point_2 = Point::new(Some(field_3), Some(field_4), a, b);
        assert_eq!(point_1 + point_1 + point_1 + point_1, point_2);

        field_3 = FieldElement::new(116, prime);
        field_4 = FieldElement::new(55, prime);
        point_2 = Point::new(Some(field_3), Some(field_4), a, b);
        assert_eq!(
            point_1 + point_1 + point_1 + point_1 + point_1 + point_1 + point_1 + point_1,
            point_2
        );

        point_2 = Point::new(None, None, a, b);
        assert_eq!(
            point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1
                + point_1,
            point_2
        );
    }

    #[test]
    fn test_find_the_order() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let field_1 = FieldElement::new(15, prime);
        let field_2 = FieldElement::new(86, prime);

        let point = Point::new(Some(field_1), Some(field_2), a, b);
        let point_at_inf = Point::new(None, None, a, b);

        let mut product = point;
        let mut count = 1;

        while product != point_at_inf {
            product = product + point;
            count += 1;
        }

        assert_eq!(count, 7);

        assert_eq!(point.rmul(7), point_at_inf);
    }
}
