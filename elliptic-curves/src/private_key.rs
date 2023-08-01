use finite_fields::FieldElement;
use num_bigint::BigUint;
use num_traits::{Num, One};
use rand::Rng;

use crate::{helper::encode_base58_checksum, signature::Signature, Point, N};

#[derive(Debug)]
pub struct PrivateKey {
    pub secret: BigUint,
    pub point: Point,
}

impl PrivateKey {
    pub fn new(secret: BigUint) -> Self {
        let g = Point::get_point_g();
        Self {
            secret: secret.clone(),
            point: g.rmul(secret),
        }
    }

    pub fn sign(&self, z: BigUint) -> Signature {
        let n = BigUint::from_str_radix(N, 16).unwrap();
        let mut rng = rand::thread_rng();
        let mut k;

        loop {
            k = BigUint::from(rng.gen::<u64>());

            if &k < &n {
                break;
            }
        }

        let g = Point::get_point_g();

        let r = g.rmul(k.clone()).x.unwrap().num;

        let k_inv = FieldElement::mod_pow(&k, n.clone() - (BigUint::one() + BigUint::one()), &n);

        let mut s = (z.clone() + r.clone() * self.secret.clone()) * k_inv % n.clone();
        if s > n.clone() / (BigUint::one() + BigUint::one()) {
            s = n - s;
        }

        Signature { r, s }
    }

    pub fn wif(&self, compressed: bool, testnet: bool) -> String {
        let mut data = Vec::new();

        if testnet {
            data.push(0xef);
        } else {
            data.push(0x80);
        }

        let secret_bytes = self.secret.to_bytes_be();
        let mut res = [0u8; 32];
        if secret_bytes.len() <= 32 {
            let offset = 32 - secret_bytes.len();
            res[offset..].copy_from_slice(&secret_bytes);
        }
        data.extend(res);


        if compressed {
            data.push(0x01);
        }

        encode_base58_checksum(&mut data)
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use num_traits::{FromPrimitive, Num};

    use super::PrivateKey;

    #[test]
    fn test_wif() {
        let mut priv_key = PrivateKey::new(BigUint::from_u16(5003).unwrap());
        let mut sec_priv_key = priv_key.wif(true, true);

        assert_eq!(
            sec_priv_key,
            "cMahea7zqjxrtgAbB7LSGbcQUr1uX1ojuat9jZodMN8rFTv2sfUK"
        );

        priv_key = PrivateKey::new(BigUint::from_u64(2021_u64.pow(5)).unwrap());
        sec_priv_key = priv_key.wif(false, true);

        assert_eq!(
            sec_priv_key,
            "91avARGdfge8E4tZfYLoxeJ5sGBdNJQH4kvjpWAxgzczjbCwxic"
        );

        priv_key = PrivateKey::new(BigUint::from_str_radix("54321deadbeef", 16).unwrap());
        sec_priv_key = priv_key.wif(true, false);

        assert_eq!(sec_priv_key, "KwDiBf89QgGbjEhKnhXJuH7LrciVrZi3qYjgiuQJv1h8Ytr2S53a");
    }
}
