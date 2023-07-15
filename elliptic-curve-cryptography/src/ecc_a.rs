//!
//! Elliptic Curve Calculation (Affine)
//!

use crate::{number::Number, EcAxis, EcOp, Point};

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
            x: T::zero,
            y: T::zero,
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
        l ==  r // return bool
    }

    fn equals(&self, p1: &Point<T>, p2: &Point<T>) -> bool {
        
    }


}
