use finite_fields::FieldElement;
use num_bigint::BigUint;
use num_traits::{Num, One};
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
}
