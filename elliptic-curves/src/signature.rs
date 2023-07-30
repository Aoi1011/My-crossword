use num_bigint::BigUint;

pub struct Signature {
    pub r: BigUint,
    pub s: BigUint,
}

impl Signature {
    pub fn new(r: BigUint, s: BigUint) -> Self {
        Self { r, s }
    }

    pub fn der(&self) -> Vec<u8> {
        let mut rbin = self.r.to_bytes_be();

        // remove all null bytes at the beginning
        while !rbin.is_empty() && rbin[0] == 0 {
            rbin.remove(0);
        }
        // if rbin has a high bit, add a \x00
        if !rbin.is_empty() && (rbin[0] & 0x80) != 0 {
            rbin.insert(0, 0x00);
        }

        let mut result = vec![0x02, rbin.len() as u8];
        result.extend(&rbin);

        let mut sbin = self.s.to_bytes_be();
        // remove all null bytes at the beginning
        while !sbin.is_empty() && sbin[0] == 0 {
            sbin.remove(0);
        }
        // if sbin has a high bit, add a \x00
        if !sbin.is_empty() && (sbin[0] & 0x80) != 0 {
            sbin.insert(0, 0x00);
        }

        result.push(0x02);
        result.push(sbin.len() as u8);
        result.extend(&sbin);

        let mut full_result = vec![0x30, result.len() as u8];
        full_result.append(&mut result);

        full_result
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::Num;

    use super::Signature;

    #[test]
    fn test_der_format() {
        let r = BigUint::from_str_radix(
            "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        )
        .unwrap();
        let s = BigUint::from_str_radix(
            "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        )
        .unwrap();

        let sig = Signature::new(r, s);
        let res = hex::encode(sig.der());
        assert_eq!(
            res,
            String::from(
                "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6022100\
8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec"
            )
        );
    }
}
