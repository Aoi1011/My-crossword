//!
//! Elliptic Curve Calculation (Affine)
//!

use crate::{
    ec_param::{EcParam, ParamOp},
    ecc::{EcAxis, EcOp, Point},
    number::Number,
};

/// Elliptic curve (Affine)
#[derive(Clone)]
pub struct EcpA<T: Number> {
    pub a: T,
    pub b: T,
    pub p: T,
    pub n: T,
    pub h: T,
    pub p_zero: Point<T>,
    pub p_g: Point<T>,
}

impl<T: Number> EcOp<T> for EcpA<T> {
    // Get zero Point<T>
    fn get_zero(&self) -> Point<T> {
        Point {
            axis: EcAxis::Affine,
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }

    /// Get zero Point<T>
    #[inline]
    fn is_zero(&self, p: &Point<T>) -> bool {
        p.is_zero()
    }

    /// Copy Point<T>
    fn set(&self, p1: &mut Point<T>, p2: &Point<T>) {
        *p1 = p2.clone();
    }

    /// Point<T> P is on curve ?
    fn on_curve(&self, p: &Point<T>) -> bool {
        if p.is_zero() {
            return true;
        }

        // y^2 = x^3 + ax + b = (x^2 + a) * x + b
        let l = p.y.mul_ref(&p.x) % &self.p;
        let r = ((p.x.mul_ref(&p.x) + &self.a) * &p.x + &self.b) % &self.p;
        l == r // return bool
    }

    fn equals(&self, p1: &Point<T>, p2: &Point<T>) -> bool {
        p1.x == p2.x && p1.y == p2.y
    }

    /// Convert 3D to 2D (No operation)
    fn to_affine(&self, p: &Point<T>) -> Point<T> {
        p.clone()
    }

    fn normalize(&self, _p: &mut Point<T>) {}

    /// Find Point<T> on curve at x
    fn point_from_x(&self, x: &T, yt: u32) -> Point<T> {
        assert!(x < &self.p);

        // y^2 = x^3 + ax + b = (x^2 + a) * x + b
        let y2 = Number::mod_cal(&((x.mul_ref(x) + &self.a) * x + &self.b), &self.p);
        let mut y = Number::mod_sqrt(&y2, &self.p);

        let bit0 = y.clone() % T::from(2u32);
        if bit0 != T::from(yt) {
            y = self.p.sub_ref(&y);
        }

        Point {
            axis: EcAxis::Affine,
            x: x.clone(),
            y,
            z: T::one(),
        }
    }

    /// Find point on curve at (x, y)
    fn point_from_xy(&self, x: &T, y: &T) -> Point<T> {
        Point {
            axis: EcAxis::Affine,
            x: x.clone(),
            y: y.clone(),
            z: T::one(),
        }
    }

    /// OS2ECPP: Decode a point on this curve which has been encoded using point
    /// compression(x9.62) returning the point.
    #[allow(unused_assignments)]
    fn decode_point(&self, enc: &[u8]) -> Point<T> {
        if enc.len() == 1 {
            if enc[0] == 0 {
                return self.get_zero();
            } else {
                panic!("Invalid point compression");
            }
        }

        let mut x = T::zero();
        let mut y = T::zero();
        match enc[0] {
            // compressed
            0x02 | 0x03 => {
                let ln = enc.len() - 1;
                let xb = &enc[1..ln + 1];
                x = T::from_bytes_be(xb);
                let yt = enc[0] & 1;
                return self.point_from_x(&x, yt.into());
            }
            // uncompressed
            0x04 => {
                let ln = (enc.len() - 1) / 2;
                let xb = &enc[1..(ln + 1)];
                let yb = &enc[(ln + 1)..enc.len()];
                x = T::from_bytes_be(xb);
                y = T::from_bytes_be(yb);
            }
            _ => panic!("Invalid point compression"),
        }

        Point {
            axis: EcAxis::Affine,
            x,
            y,
            z: T::one(),
        }
    }

    /// Returns the field element encoded with point compression
    fn get_encoded(&self, p: &Point<T>) -> Vec<u8> {
        let q = self.to_affine(p);

        let y = q.y;
        let x = q.x;
        let yt: T = y % T::from(2u32);
        let hd = if yt.is_one() { 0x03 } else { 0x02 };

        let xb = x.to_bytes_be();
        let mut po = vec![0u8; xb.len() + 1];
        po[0] = hd;
        po[1..].copy_from_slice(&xb[..xb.len()]);

        po
    }

    /// Negate P
    fn negate(&self, p: &Point<T>) -> Point<T> {
        Point {
            axis: EcAxis::Affine,
            x: p.x.clone(),
            y: self.p.clone() - p.y.clone(),
            z: T::one(),
        }
    }

    /// EC Point<T> double : q = 2 * P
    fn double(&self, p: &Point<T>) -> Point<T> {
        let (x, y, mut r): (T, T, T);

        if p.is_zero() || p.y == T::zero() {
            return self.p_zero.clone();
        }

        r = (T::from(3u32) * &p.x * &p.x + &self.a) % &self.p;
        r = Number::mod_inv(&(T::from(2u32) * &p.y), &self.p) * &r % &self.p;
        x = Number::mod_cal(&(r.mul_ref(&r) % &self.p - T::from(2u32) * &p.x), &self.p);
        y = Number::mod_cal(&((p.x.sub_ref(&x)) * &r % &self.p - &p.y), &self.p);

        Point {
            axis: EcAxis::Affine,
            x,
            y,
            z: T::one(),
        }
    }

    /// EC point add : Q = P1 + P2
    fn add(&self, p1: &Point<T>, p2: &Point<T>) -> Point<T> {
        let (x, y, mut r): (T, T, T);

        if p1.is_zero() {
            return p2.clone();
        }
        if p2.is_zero() {
            return p1.clone();
        }

        if p1.x == p2.x && (p1.y != p2.y || p1.y == T::zero()) {
            // P1 + (-P1) == 0
            return self.p_zero.clone();
        }

        if p1.x == p2.x {
            r = (T::from(3u32) * &p1.x * &p1.x + &self.a) % &self.p;
            r = Number::mod_inv(&(T::from(2u32) * &p1.y), &self.p) * &r % &self.p;
        } else {
            r = ((p1.y.sub_ref(&p2.y)) * Number::mod_inv(&(p1.x.sub_ref(&p2.x)), &self.p))
                % &self.p;
        }
        x = Number::mod_cal(&(r.mul_ref(&r) - &p1.x - &p2.x), &self.p);
        y = Number::mod_cal(&((p1.x.sub_ref(&x)) * &r - &p1.y), &self.p);

        Point {
            axis: EcAxis::Affine,
            x,
            y,
            z: T::one(),
        }
    }

    /// Generate a random point
    fn gen_point(&self) -> Point<T> {
        let (mut x, mut y, mut y2): (T, T, T);
        let pu = self.p.clone();

        loop {
            x = Number::gen_rand(&T::one(), &(pu.sub_ref(&T::one())));

            // y^2 = x^3 + ax + b = (x^2 + a) * x  b
            y2 = Number::mod_cal(&((x.mul_ref(&x) + &self.a) * &x + &self.b), &self.p);
            y = Number::mod_sqrt(&y2, &self.p);

            let pt = Point {
                axis: EcAxis::Affine,
                x,
                y,
                z: T::one(),
            };
            if self.on_curve(&pt) {
                return pt;
            }
        }
    }

    fn calc_order(&self, p: &Point<T>) -> T {
        //assert self.on_curve(p) and P != self.zero
        let mut m = T::one();

        while m < self.p.add_ref(&T::one()) {
            if self.mul(p, &m).is_zero() {
                return m;
            }
            m = m + T::one();
        }
        panic!("Invalid order");
    }

    /// Get generator point
    #[inline]
    fn get_gp(&self) -> Point<T> {
        self.p_g.clone()
    }

    /// Get order
    #[inline]
    fn get_order(&self) -> T {
        self.n.clone()
    }

    /// Get cofactor
    #[inline]
    fn get_cofac(&self) -> T {
        self.h.clone()
    }

    /// Display point data
    fn print(&self, s: &str, p: &Point<T>) {
        println!("{}: [{}, {}]", s, p.x, p.y);
    }
}

impl<T: Number> EcpA<T> {
    pub fn new(ec_name: &str) -> EcpA<T> {
        let ec = EcParam::new(ec_name);

        EcpA {
            a: ec.get_a(),
            b: ec.get_b(),
            p: ec.get_prime(),
            n: ec.get_order(),
            h: ec.get_cofactor(),
            p_zero: Point {
                axis: EcAxis::Affine,
                x: T::zero(),
                y: T::zero(),
                z: T::one(),
            },
            p_g: Point {
                axis: EcAxis::Affine,
                x: ec.get_gx(),
                y: ec.get_gy(),
                z: T::one(),
            },
        }
    }
}
