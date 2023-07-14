//!
//! Elliptic Curve Point Parameter Interface

use crate::number::Number;

/// EC curve parameters
/// id, a, b, p, g (04|X|Y), order, h
pub struct EcParam<'a> {
    pub id: &'a str,
    pub a: &'a [u8],
    pub b: &'a [u8],
    pub p: &'a [u8],
    pub g: &'a [u8],
    pub n: &'a [u8],
    pub h: &'a [u8],
}

static CURVE_PARAMS: [EcParam; 1] = [
        EcParam {
        id: "secp256k1",
        a: b"0000000000000000000000000000000000000000000000000000000000000000",
        b: b"0000000000000000000000000000000000000000000000000000000000000007",
        p: b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        g: b"0479BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        n: b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        h: b"01"
    },
];

impl EcParam<'_> {
    pub fn new(ecid: &str) -> Self {
        let mut id: &str = CURVE_PARAMS[0].id;
        let mut ai: &[u8] = CURVE_PARAMS[0].a;
        let mut bi: &[u8] = CURVE_PARAMS[0].b;
        let mut pi: &[u8] = CURVE_PARAMS[0].p;
        let mut gi: &[u8] = CURVE_PARAMS[0].g;
        let mut ni: &[u8] = CURVE_PARAMS[0].n;
        let mut hi: &[u8] = CURVE_PARAMS[0].h;

        for prm in CURVE_PARAMS.iter() {
            if prm.id == ecid {
                id = prm.id;
                ai = prm.a;
                bi = prm.b;
                pi = prm.p;
                gi = prm.g;
                ni = prm.n;
                hi = prm.h;
            }
        }

        EcParam {
            id,
            a: ai,
            b: bi,
            p: pi,
            g: gi,
            n: ni,
            h: hi,
        }
    }

    pub fn get_ecid(&self) -> &str {
        self.id
    }

    pub fn set_parameters(&mut self, id: &str) {
        for prm in CURVE_PARAMS.iter() {
            if prm.id == id {
                self.id = prm.id;
                self.a = prm.a;
                self.b = prm.b;
                self.p = prm.p;
                self.g = prm.g;
                self.n = prm.n;
                self.h = prm.h;
            }
        }
    }

    pub fn check_parameters(&self, id: &str) -> bool {
        for prm in CURVE_PARAMS.iter() {
            if prm.id == id {
                return true;
            }
        }
        false
    }
}

pub trait ParamOp<T: Number> {
    fn get_prime(&self) -> T;
    fn get_a(&self) -> T;
    fn get_b(&self) -> T;
    fn get_order(&self) -> T;
    fn get_cofactor(&self) -> T;
    fn get_gx(&self) -> T;
    fn get_gy(&self) -> T;
}

impl<T: Number> ParamOp<T> for EcParam<'_> {
    fn get_prime(&self) -> T {
        T::from_bytes_radix(self.p, 16)
    }

    fn get_a(&self) -> T {
        T::from_bytes_radix(self.a, 16)
    }

    fn get_b(&self) -> T {
        T::from_bytes_radix(self.b, 16)
    }

    fn get_order(&self) -> T {
        T::from_bytes_radix(self.n, 16)
    }

    fn get_cofactor(&self) -> T {
        T::from_bytes_radix(self.h, 16)
    }

    fn get_gx(&self) -> T {
        let ln = (self.g.len() - 2) / 2;
        let gb = &self.g[2..(ln + 2)];
        T::from_bytes_radix(gb, 16)
    }

    fn get_gy(&self) -> T {
        let ln = (self.g.len() - 2) / 2;
        let gb = &self.g[(ln + 2)..self.g.len()];
        T::from_bytes_radix(gb, 16)
    }
}
