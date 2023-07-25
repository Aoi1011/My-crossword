use std::fmt;

use num_traits::{One, Zero};

use crate::number::Number;

#[derive(Clone, PartialEq, Copy)]
pub enum EcAxis {
    Affine,
    Proj,
}

#[derive(Clone)]
pub struct Point<T> {
    pub axis: EcAxis, // Affine, Proj
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T>
where
    T: Zero + One,
{
    pub fn get_axis(&self) -> EcAxis {
        self.axis
    }

    pub fn is_zero(&self) -> bool {
        if self.axis == EcAxis::Affine {
            return self.x.is_zero() && self.y.is_zero();
        }
        self.z.is_zero() // Proj
    }

    pub fn set_zero(&mut self) {
        self.x.set_zero();
        self.y.set_zero();
        self.z.set_zero()
    }
}

pub trait PointOp<T> {
    fn set(&mut self, other: &Point<T>);
    fn equals(&self, other: &Point<T>) -> bool;
}

impl<T: Number> PointOp<T> for Point<T> {
    fn set(&mut self, p: &Point<T>) {
        self.axis = p.axis;
        self.x = p.x.clone();
        self.y = p.y.clone();
        self.z = p.z.clone();
    }

    fn equals(&self, other: &Point<T>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T: Number> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.axis == EcAxis::Affine {
            return write!(f, "[{}, {}]", self.x, self.y);
        }
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

pub trait EcOp<T: Number> {
    /// Point on curve ?
    fn on_curve(&self, p: &Point<T>) -> bool;
    /// Check equality
    fn equals(&self, p1: &Point<T>, p2: &Point<T>) -> bool;
    /// Find points on curve at x
    fn point_from_x(&self, x: &T, yt: u32) -> Point<T>;
    /// Find points on curve at (x, y)
    fn point_from_xy(&self, x: &T, y: &T) -> Point<T>;
    /// OS2ECPP: Decode a point
    fn decode_point(&self, enc: &[u8]) -> Point<T>;
    /// Convert 3D to 2D
    fn to_affine(&self, p: &Point<T>) -> Point<T>;
    fn normalize(&self, p: &mut Point<T>);
    /// Get zero point
    fn get_zero(&self) -> Point<T>;
    /// Is zero point ?
    fn is_zero(&self, p: &Point<T>) -> bool;
    /// Set point P1 <- P2
    fn set(&self, p1: &mut Point<T>, p2: &Point<T>);

    /// Negate P
    fn negate(&self, p: &Point<T>) -> Point<T>;
    /// EC point add : Q = P1 + P2
    fn add(&self, p1: &Point<T>, p2: &Point<T>) -> Point<T>;
    /// EC point sub : Q = P1 - P2
    fn sub(&self, p1: &Point<T>, p2: &Point<T>) -> Point<T> {
        let pt = self.negate(p2);
        self.add(p1, &pt)
    }

    /// EC point double : Q = 2 * P
    fn double(&self, p: &Point<T>) -> Point<T>;

    /// Multiply (k * P)
    /// Sliding window multiplication
    fn mul(&self, p_p: &Point<T>, k: &T) -> Point<T> {
        let w = get_win_size(k);
        let table = self.get_win_table(p_p, w);
        let mut rv = self.get_zero();

        if k.is_zero() {
            return rv;
        }

        let mut word = 0;
        let mut wbits = 0;
        let size = k.bit_len() - 1;
        let mut bit;

        let mut inw = 0;
        for s in (0..size + 1).rev() {
            rv = self.double(&rv);
            bit = k.test_bit(s);
            if inw == 0 && !bit {
                continue;
            }
            if inw == 0 {
                inw = 1;
                word = 1;
                wbits = 1;
            } else {
                if bit {
                    word = (word << 1) + 1;
                } else {
                    word <<= 1;
                }
                wbits += 1;
            }
            if wbits == w || s == 0 {
                rv = self.add(&rv, &table[word]);
                inw = 0;
            }
        }
        rv
    }

    /// Builds w-bit lookup window
    fn get_win_table(&self, p: &Point<T>, w: usize) -> Vec<Point<T>> {
        if w < 1 {
            return vec![]; // no window
        }

        // build 2 ^ w look up table. lookup[i]
        let lookup_size = 1 << w;
        let mut lookup: Vec<Point<T>> = Vec::with_capacity(lookup_size);

        lookup.push(self.get_zero()); // lookup[0] = 0
        for i in 1..lookup_size {
            lookup.push(self.add(p, &lookup[i - 1]));
        }
        lookup
    }

    /// Multiply (k * P) (Binary method)
    fn mul_bin(&self, p: &Point<T>, k: &T) -> Point<T> {
        let mut p_r = self.get_zero();
        let mut m = k.clone();
        let mut p_s = p.clone();

        while m > T::zero() {
            // if &m & T::one() == T::one() {
            if m.test_bit(0) {
                p_r = self.add(&p_r, &p_s);
            }
            m = m >> 1;
            p_s = self.double(&p_s);
        }
        p_r
    }

    /// Generate a random point
    fn gen_point(&self) -> Point<T>;
    /// Order calculation
    fn calc_order(&self, p: &Point<T>) -> T;
    /// Get order
    fn get_order(&self) -> T;
    /// Get cofactor
    fn get_cofac(&self) -> T;
    /// Get generator point
    fn get_gp(&self) -> Point<T>;
    fn get_encoded(&self, p: &Point<T>) -> Vec<u8>;
    /// Display point data
    fn print(&self, s: &str, p: &Point<T>);
}

/// Gets lookup window size
pub fn get_win_size<T: Number>(n: &T) -> usize {
    let table = vec![47, 157, 474, 1324, 3529, 9005];

    let pbits = n.bit_len();
    for ret in (0..6).rev() {
        // 5, 4, ... 0
        if pbits > table[ret] {
            return ret + 3;
        }
    }
    2
}
