use crate::field::{Field, FieldStorage};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
// A group element of the secp256k1 curve, in affine coordinates.
pub struct Affine {
    pub x: Field,
    pub y: Field,
    pub infinity: bool,
}

#[derive(Debug, Clone, Copy)]
// A group of element of the secp256k1 curve, in jacobian coordinates.
pub struct Jacobian {
    pub x: Field,
    pub y: Field,
    pub z: Field,
    pub infinity: bool,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
// Affine coordinate group element compact storage.
pub struct AffineStorage {
    pub x: FieldStorage,
    pub y: FieldStorage,
}

impl Default for Affine {
    fn default() -> Self {
        Self {
            x: Field::default(),
            y: Field::default(),
            infinity: false,
        }
    }
}

impl Default for Jacobian {
    fn default() -> Self {
        Self {
            x: Field::default(),
            y: Field::default(),
            z: Field::default(),
            infinity: false,
        }
    }
}

impl Default for AffineStorage {
    fn default() -> Self {
        Self {
            x: FieldStorage::default(),
            y: FieldStorage::default(),
        }
    }
}

pub static AFFINE_INFINITY: Affine = Affine {
    x: Field::new(0, 0, 0, 0, 0, 0, 0, 0),
    y: Field::new(0, 0, 0, 0, 0, 0, 0, 0),
    infinity: true,
};

pub static JACOBIAN_INFINITY: Jacobian = Jacobian {
    x: Field::new(0, 0, 0, 0, 0, 0, 0, 0),
    y: Field::new(0, 0, 0, 0, 0, 0, 0, 0),
    z: Field::new(0, 0, 0, 0, 0, 0, 0, 0),
    infinity: true,
};

pub static AFFINE_G: Affine = Affine::new(
    Field::new(
        0x79BE667E, 0xF9DCBBAC, 0x55A06295, 0xCE870B07, 0x029BFCDB, 0x2DCE28D9, 0x59F2815B,
        0x16F81798,
    ),
    Field::new(
        0x483ADA77, 0x26A3C465, 0x5DA4FBFC, 0x0E1100AB, 0xFD17B448, 0xA6855419, 0x9C47D08F,
        0xFB10D488,
    ),
);

pub const CURVE_B: u32 = 7;

impl Affine {
    /// Create a new affine
    pub const fn new(x: Field, y: Field) -> Self {
        Self {
            x,
            y,
            infinity: false,
        }
    }

    /// Set a group element equal to the point with given x and y
    /// coordinates
    pub fn set_xy(&mut self, x: &Field, y: &Field) {
        self.infinity = false;
        self.x = *x;
        self.y = *y;
    }

    /// Set a group element (affine) equal to the point with the given
    /// X coordinate and a Y coordinate that is a quadratic residue
    /// modulo p. The return value is true if a coordinate with the
    /// given X coordinate exists.
    pub fn set_xquad(&mut self, x: &Field) -> bool {
        self.x = *x;
        let x2 = x.sqr();
        let x3 = *x * x2;
        self.infinity = false;
        let mut c = Field::default();
        c.set_int(CURVE_B);
        c += x3;
        let (v, ret) = c.sqrt();
        self.y = v;
        ret
    }

    /// Set a group element (affine) equal to the point with the given
    /// X coordinate, and given oddness for Y. Return value indicates
    /// whether the result is valid
    pub fn set_xo_var(&mut self, x: &Field, odd: bool) -> bool {
        if !self.set_xquad(x) {
            return false;
        }
        self.y.normalize_var();
        if self.y.is_odd() != odd {
            self.y = self.y.neg(1);
        }
        true
    }

    /// Check whether a group element is the point at infinity
    pub fn is_infinity(&self) -> bool {
        self.infinity
    }

    /// Check whether a group element is valid (i.e., on the curve).
    pub fn is_valid_var(&self) -> bool {
        if self.is_infinity() {
            return false;
        }
        let y2 = self.y.sqr();
        let mut x3 = self.x.sqr();
        x3 *= &self.x;
        let mut c = Field::default();
        c.set_int(CURVE_B);
        x3 += &c;
        x3.normalize_weak();
        y2.eq_var(&x3)
    }

    pub fn neg_in_place(&mut self, other: &Affine) {
        *self = *other;
        self.y.normalize_weak();
        self.y = self.y.neg(1);
    }

    pub fn neg(&self) -> Affine {
        let mut ret = Affine::default();
        ret.neg_in_place(self);
        ret
    }

    /// Set a group element equal to another which is given in
    /// jacobian coordinate.
    pub fn set_gej(&mut self, a: &Jacobian) {
        self.infinity = a.infinity;
        let mut a = *a;
        a.z = a.z.inv();
        let z2 = a.z.sqr();
        let z3 = a.z * z2;
        a.x *= z2;
        a.y *= z3;
        a.z.set_int(1);
        self.x = a.x;
        self.y = a.y;
    }

    pub fn from_gej(a: &Jacobian) -> Self {
        let mut ge = Self::default();
        ge.set_gej(a);
        ge
    }

    pub fn set_gej_var(&mut self, a: &Jacobian) {
        let mut a = *a;
        self.infinity = a.infinity;
        if a.is_infinity() {
            return;
        }
        a.z = a.z.inv_var();
        let z2 = a.z.sqr();
        let z3 = a.z * z2;
        a.x *= &z2;
        a.y *= &z3;
        a.z.set_int(1);
        self.x = a.x;
        self.y = a.y;
    }

    pub fn set_gej_zinv(&mut self, a: &Jacobian, zi: &Field) {
        let zi2 = zi.sqr();
        let zi3 = zi2 * *zi;
        self.x = a.x * zi2;
        self.y = a.y * zi3;
        self.infinity = a.infinity;
    }

    /// Clear a secp256k1_ge to prevent leaking sensitive information
    pub fn clear(&mut self) {
        self.infinity = false;
        self.x.clear();
        self.y.clear();
    }
}

impl Jacobian {
    /// Check whether a group element is the point at infinity
    pub fn is_infinity(&self) -> bool {
        self.infinity
    }
}
