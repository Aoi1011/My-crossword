use finite_fields::FieldElement;
use num_bigint::BigUint;
use num_traits::{One, FromPrimitive, Num};
use rand::Rng;

use crate::{signature::Signature, Point, N};

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
        
        let k = rng.gen_range(0..n);
        let e = BigUint::from_bytes_be(&Point::hash256(b"my secret"));
        // 0x231c6f3d980a6b0fb7152f85cee7eb52bf92433d9919b9c5218cb08e79cce78
        let z = BigUint::from_bytes_be(&Point::hash256(b"my message"));

        let k = BigUint::from_u32(1234567890).unwrap();

        let g = Point::get_point_g();

        // 0x2b698a0f0a4041b77e63488ad48c23e8e8838dd1fb7520408b121697b782ef22
        let r = g.rmul(k.clone()).x.unwrap().num;

        let k_inv = FieldElement::mod_pow(&k, n.clone() - (BigUint::one() + BigUint::one()), &n);

        // 0xbb14e602ef9e3f872e25fad328466b34e6734b7a0fcd58b1eb635447ffae8cb9
        let s = (z.clone() + r.clone() * e.clone()) * k_inv % n;

        Signature { r, s }
    }
}
