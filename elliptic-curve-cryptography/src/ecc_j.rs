//!
//! Elliptic Curve Calculation (Projective - Jacobian)

use crate::{
    ec_param::{EcParam, ParamOp},
    ecc::{EcAxis, EcOp, Point},
    number::Number,
};

/// Elliptic curve (Projective - Jacobian)
#[derive(Clone)]
pub struct EcpJ<T: Number> {
    pub a: T,
    pub b: T,
    pub p: T,
    pub n: T,
    pub h: T,
    pub p_zero: Point<T>,
    pub p_g: Point<T>,
}

impl<T: Number> EcOp<T> for EcpJ<T> {
    /// Get zero point
    fn get_zero(&self) -> Point<T> {
        Point {
            axis: EcAxis::Proj,
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Get zero point
    #[inline]
    fn is_zero(&self, p: &Point<T>) -> bool {
        p.is_zero()
    }

    /// Copy point
    fn set(&self, p1: &mut Point<T>, p2: &Point<T>) {
        *p1 = p2.clone();
    }

    /// Point P is on curve ?
    #[allow(clippy::many_single_char_names)]
    fn on_curve(&self, p: &Point<T>) -> bool {
        let (l, r);
        let (z2, z4, z6);

        if p.z.is_one() {
            return true;
        }

        // Y^2 = x^3 + aXZ^2 + bZ^6
        l = p.y.mul_ref(&p.y) % &self.p;
        z2 = p.z.mul_ref(&p.z) % &self.p;
        z4 = z2.mul_ref(&z2) % &self.p;
        z6 = z2.mul_ref(&z4) % &self.p;
        r = ((p.x.mul_ref(&p.x) * &p.x) + self.a.mul_ref(&p.x) * &z4 + self.b.mul_ref(&z6))
            % &self.p;

        l == r
    }

    /// Convert 3D to 2D
    fn to_affine(&self, p: &Point<T>) -> Point<T> {
        if p.z.is_one() {
            return p.clone();
        }

        let x = (Number::mod_inv(&(p.z.mul_ref(&p.z)), &self.p) * &p.x) % &self.p;
        let y = (Number::mod_inv(&Number::mod_pow(&p.z, &T::from(3u32), &self.p), &self.p) * &p.y)
            % &self.p;

        Point {
            axis: EcAxis::Proj,
            x,
            y,
            z: T::one(),
        }
    }

    fn normalize(&self, p: &mut Point<T>) {
        if p.z.is_one() || p.z.is_zero() {
            return;
        }

        p.x = (Number::mod_inv(&(p.z.mul_ref(&p.z)), &self.p) * &p.x) % &self.p;
        p.y = (Number::mod_inv(&Number::mod_pow(&p.z, &T::from(3u32), &self.p), &self.p) * &p.y)
            % &self.p;
        p.z = T::one();
    }

    /// Check equality
    fn equals(&self, p1: &Point<T>, p2: &Point<T>) -> bool {
        let mut result = true;
        let (mut z1, mut z2);

        if p1.z.is_one() && p2.z.is_one() {
            return p1.x == p2.x && p1.y == p2.y;
        }
        if p1.z.is_one() {
            // z2^2
            z2 = p2.z.mul_ref(&p2.z) % &self.p;
            // x1 * z2^2 = x2 ?
            result = result && (p2.x == p1.x.mul_ref(&z2) % &self.p);
            z2 = z2.mul_ref(&p1.z) % &self.p;
            // y1 * z2^3 = y1 ?
            result = result && (p1.y == p1.y.mul_ref(&z2) % &self.p);
            return result;
        }
        z1 = p1.z.mul_ref(&p1.z) % &self.p; // z1^2

        if p2.z.is_one() {
            result = result && (p1.x == p2.x.mul_ref(&z1) % &self.p);
            z1 = z1.mul_ref(&p1.z);
            result = result && (p1.y == p2.y.mul_ref(&z1) % &self.p);
        } else {
            z2 = p2.z.mul_ref(&p2.z) % &self.p;
            result = result && (p1.x.mul_ref(&z2) % &self.p == p2.x.mul_ref(&z1) % &self.p);
            z1 = p1.z.mul_ref(&z1) % &self.p;
            z2 = p2.z.mul_ref(&z2) % &self.p;
            result = result && (p1.y.mul_ref(&z2) % &self.p == p2.y.mul_ref(&z1) % &self.p);
        }

        result
    }

    /// Negate P
    fn negate(&self, p: &Point<T>) -> Point<T> {
        Point {
            axis: EcAxis::Proj,
            x: p.x.clone(),
            y: self.p.clone() - p.y.clone(),
            z: p.z.clone(),
        }
    }

    /// EC point double : q = 2 * P
    fn double(&self, p: &Point<T>) -> Point<T> {
        let (mut t1, mut t2, mut t3) = (p.x.clone(), p.y.clone(), p.z.clone());
        let (mut t4, mut t5);

        if t2.is_zero() || t3.is_zero() {
            return Point {
                axis: EcAxis::Proj,
                x: T::one(),
                y: T::one(),
                z: T::zero(),
            };
        }
        if self.a == self.p.sub_ref(&T::from(3u32)) % &self.p {
            t4 = t3.mul_ref(&t3) % &self.p;
            t5 = t1.sub_ref(&t4) % &self.p;
            t4 = t1.add_ref(&t4) % &self.p;
            t5 = t4.mul_ref(&t5) % &self.p;
            t4 = T::from(3u32) * &t5 % &self.p;
        } else {
            t4 = self.a.clone();
            t5 = t3.mul_ref(&t3) % &self.p;
            t5 = t5.mul_ref(&t5) % &self.p;
            t5 = t4.mul_ref(&t5) % &self.p;
            t4 = t1.mul_ref(&t1) % &self.p;
            t4 = T::from(3u32) * &t4 % &self.p;
            t4 = t4.add_ref(&t5) % &self.p;
        }
        t3 = t2.mul_ref(&t3) % &self.p;
        t3 = Number::mod_cal(&(T::from(2u32) * &t3), &self.p);
        t2 = t2.mul_ref(&t2) % &self.p;
        t5 = t1.mul_ref(&t2) % &self.p;
        t5 = T::from(4u32) * &t5 % &self.p;
        t1 = t4.mul_ref(&t4) % &self.p;
        t1 = Number::mod_cal(&(t1.sub_ref(&(T::from(2u32) * &t5))), &self.p);
        t2 = t2.mul_ref(&t2) % &self.p;
        t2 = T::from(8u32) * &t2 % &self.p;
        t5 = t5.sub_ref(&t1) % &self.p;
        t5 = t4.mul_ref(&t5) % &self.p;
        t2 = Number::mod_cal(&(t5.sub_ref(&t2)), &self.p);

        Point {
            axis: EcAxis::Proj,
            x: t1,
            y: t2,
            z: t3,
        }
    }

    /// EC Point<T> add : Q = P1 + P2
    fn add(&self, p1: &Point<T>, p2: &Point<T>) -> Point<T> {
        if p1.z.is_zero() {
            return p2.clone();
        }
        if p2.z.is_zero() {
            return p1.clone();
        }
        if self.equals(p1, p2) {
            return self.double(p1);
        }

        let (mut t1, mut t2, mut t3) = (p1.x.clone(), p1.y.clone(), p1.z.clone());
        let (mut t4, mut t5) = (p2.x.clone(), p2.y.clone());
        let (mut t6, mut t7): (T, T);

        t6 = T::one();
        if !p2.z.is_one() {
            t6 = p2.z.clone() % &self.p;
            t7 = t6.mul_ref(&t6) % &self.p;
            t1 = t1.mul_ref(&t7) % &self.p; // U0 = P1.x * P2.z^2
            t7 = t6.mul_ref(&t7) % &self.p;
            t2 = t2.mul_ref(&t7) % &self.p; // S0 = P1.y * P2.z^3
        }

        t7 = t3.mul_ref(&t3) % &self.p;
        t4 = t4.mul_ref(&t7) % &self.p; // U1 = P2.x * P1.z^2
        t7 = t3.mul_ref(&t7) % &self.p;
        t5 = t5.mul_ref(&t7) % &self.p; // S1 = P2.y * P1.z^3
        t4 = t1.sub_ref(&t4) % &self.p; // W = U0 - U1
        t5 = t2.sub_ref(&t5) % &self.p; // R = S0 - S1

        if t4.is_zero() {
            if t5.is_zero() {
                return Point {
                    axis: EcAxis::Proj,
                    x: T::zero(),
                    y: T::zero(),
                    z: T::zero(),
                };
            } else {
                return Point {
                    axis: EcAxis::Proj,
                    x: T::one(),
                    y: T::one(),
                    z: T::zero(),
                };
            }
        }

        t1 = (T::from(2u32) * &t1 - &t4) % &self.p; // T = U0 + U1
        t2 = (T::from(2u32) * &t2 - &t5) % &self.p; // H = S0 + S1
        if !p2.z.is_one() {
            t3 = t3.mul_ref(&t6) % &self.p;
        }

        t3 = Number::mod_cal(&(t3.mul_ref(&t4)), &self.p); // Z3 = z1 * z2*W
        t7 = t4.mul_ref(&t4) % &self.p;
        t4 = t4.mul_ref(&t7) % &self.p;
        t7 = t1.mul_ref(&t7) % &self.p;
        t1 = t5.mul_ref(&t5) % &self.p;
        t1 = Number::mod_cal(&(t1.sub_ref(&t7)), &self.p); // X3 - R^2 - T*W^2

        t7 = (t7.sub_ref(&(T::from(2u32) * &t1))) % &self.p; // V = T*W^2 - 2*x3
        t5 = t5.mul_ref(&t7) % &self.p;
        t4 = t2.mul_ref(&t4) % &self.p;
        t2 = t5.sub_ref(&t4) % &self.p;
        t2 = Number::mod_inv(&T::from(2u32), &self.p) * &t2 % &self.p; //  Y3 = (V*R - M*W^3)/2
        t2 = Number::mod_cal(&t2, &self.p);

        Point {
            axis: EcAxis::Proj,
            x: t1,
            y: t2,
            z: t3,
        }
    }

    /// Generate a random point
    fn gen_point(&self) -> Point<T> {
        let (mut x, mut y, mut y2): (T, T, T);
        let p = self.p.clone();

        loop {
            x = Number::gen_rand(&T::from(1u32), &(p.sub_ref(&T::from(1u32))));

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

    /// Find points on curve on x
    /// - returns: ((x, y, 1), (x, y, 1)) or not found on exception
    fn point_from_x(&self, x: &T, yt: u32) -> Point<T> {
        assert!(x < &self.p);

        // y^2 - x^3 + ax + b = (x^2 + a) * x + b
        let y2 = Number::mod_cal(&((x.mul_ref(x) + &self.a) * x + &self.b), &self.p);
        let mut y = Number::mod_sqrt(&y2, &self.p);

        let bite = y.clone() % T::from(2u32);
        if bite != T::from(yt) {
            y = self.p.sub_ref(&y);
        }

        Point {
            axis: EcAxis::Proj,
            x: x.clone(),
            y,
            z: T::one(),
        }
    }

    /// Find points on curve at (x, y)
    fn point_from_xy(&self, x: &T, y: &T) -> Point<T> {
        Point {
            axis: EcAxis::Proj,
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

    fn calc_order(&self, p: &Point<T>) -> T {
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

impl<T: Number> EcpJ<T> {
    pub fn new(ec_name: &str) -> EcpJ<T> {
        let ec = EcParam::new(ec_name);

        EcpJ {
            a: ec.get_a(),
            b: ec.get_b(),
            p: ec.get_prime(),
            n: ec.get_order(),
            h: ec.get_cofactor(),
            p_zero: Point {
                axis: EcAxis::Proj,
                x: T::zero(),
                y: T::zero(),
                z: T::zero(),
            },
            p_g: Point {
                axis: EcAxis::Proj,
                x: ec.get_gx(),
                y: ec.get_gy(),
                z: T::one(),
            },
        }
    }
}
