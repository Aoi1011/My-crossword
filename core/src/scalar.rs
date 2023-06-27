use crunchy::unroll;
use subtle::Choice;

const SECP256K1_N: [u32; 8] = [
    0x00364141, 0xBFD25E8C, 0xAF48A03B, 0xBAAEDCE6, 0xFFFFFFFE, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
];

const SECP256K1_N_C_0: u32 = !SECP256K1_N[0] + 1;
const SECP256K1_N_C_1: u32 = !SECP256K1_N[1];
const SECP256K1_N_C_2: u32 = !SECP256K1_N[2];
const SECP256K1_N_C_3: u32 = !SECP256K1_N[3];
const SECP256K1_N_C_4: u32 = 1;

const SECP256K1_N_H_0: u32 = 0x681B20A0;
const SECP256K1_N_H_1: u32 = 0xDFE92F46;
const SECP256K1_N_H_2: u32 = 0x57A4501D;
const SECP256K1_N_H_3: u32 = 0x5D576E73;
const SECP256K1_N_H_4: u32 = 0xFFFFFFFF;
const SECP256K1_N_H_5: u32 = 0xFFFFFFFF;
const SECP256K1_N_H_6: u32 = 0xFFFFFFFF;
const SECP256K1_N_H_7: u32 = 0x7FFFFFFF;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
/// A 256-bit scalar value.
pub struct Scalar(pub [u32; 8]);

impl Scalar {
    /// Clear a scalar to prevent the leak of sensitive data.
    pub fn clear(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.0, [0u32; 8]);
        }
    }

    /// Set a scalar to an unsigned integer.
    pub fn set_int(&mut self, v: u32) {
        self.0 = [v, 0, 0, 0, 0, 0, 0, 0];
    }

    /// Create a scalar from an unsigned integer.
    pub fn from_int(v: u32) -> Self {
        let mut scalar = Self::default();
        scalar.set_int(v);
        scalar
    }

    /// Access bits from a scalar. All requested bits must belong to
    /// the same 32-bit limb.
    pub fn bits(&self, offset: usize, count: usize) -> u32 {
        debug_assert!((offset + count - 1) >> 5 == offset >> 5);
        (self.0[offset >> 5] >> (offset & 0x1F)) & ((1 << count) - 1)
    }

    /// Access bits from a scalar. Not constant time.
    pub fn bits_var(&self, offset: usize, count: usize) -> u32 {
        debug_assert!(count < 32);
        debug_assert!(offset + count <= 256);
        if (offset + count - 1) >> 5 == offset >> 5 {
            self.bits(offset, count)
        } else {
            debug_assert!((offset >> 5) + 1 < 8);
            ((self.0[offset >> 5] >> (offset & 0x1f))
                | (self.0[(offset >> 5) + 1] << (32 - (offset & 0x1f))))
                & ((1 << count) - 1)
        }
    }

    #[must_use]
    fn check_overflow(&self) -> Choice {
        let mut yes: Choice = 0.into();
        let mut no: Choice = 0.into();

        no |= Choice::from((self.0[7] < SECP256K1_N[7]) as u8);
        no |= Choice::from((self.0[6] < SECP256K1_N[6]) as u8);
        no |= Choice::from((self.0[5] < SECP256K1_N[5]) as u8);
        no |= Choice::from((self.0[4] < SECP256K1_N[4]) as u8);
        yes |= Choice::from((self.0[4] > SECP256K1_N[4]) as u8) & !no;
        no |= Choice::from((self.0[3] < SECP256K1_N[3]) as u8) & !yes;
        yes |= Choice::from((self.0[3] > SECP256K1_N[3]) as u8) & !no;
        no |= Choice::from((self.0[2] < SECP256K1_N[2]) as u8) & !yes;
        yes |= Choice::from((self.0[2] > SECP256K1_N[2]) as u8) & !no;
        no |= Choice::from((self.0[1] < SECP256K1_N[1]) as u8) & !yes;
        yes |= Choice::from((self.0[1] > SECP256K1_N[1]) as u8) & !no;
        yes |= Choice::from((self.0[0] >= SECP256K1_N[0]) as u8) & !no;

        yes
    }

    fn reduce(&mut self, overflow: Choice) {
        let o = overflow.unwrap_u8() as u64;
        let mut t: u64;

        t = (self.0[0] as u64) + o * (SECP256K1_N_C_0 as u64);
        self.0[0] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;

        t += (self.0[1] as u64) + o * (SECP256K1_N_C_1 as u64);
        self.0[1] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;

        t += (self.0[2] as u64) + o * (SECP256K1_N_C_2 as u64);
        self.0[2] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;

        t += (self.0[3] as u64) + o * (SECP256K1_N_C_3 as u64);
        self.0[3] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;

        t += (self.0[4] as u64) + o * (SECP256K1_N_C_4 as u64);
        self.0[4] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;

        t += self.0[5] as u64;
        self.0[5] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;

        t += self.0[6] as u64;
        self.0[6] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;

        t += self.0[7] as u64;
        self.0[7] = (t & 0xFFFFFFFF) as u32;
    }

    /// Conditionally add a power of two to a scalar. The result is
    /// not allowed to overflow.
    pub fn cadd_bit(&mut self, mut bit: usize, flag: bool) {
        let mut t: u64;
        debug_assert!(bit < 256);
        bit += if flag { 0 } else { usize::max_value() } & 0x100;
        t = (self.0[0] as u64) + ((if (bit >> 5) == 0 { 1 } else { 0 }) << (bit & 0x1F));
        self.0[0] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;
        t += (self.0[1] as u64) + ((if (bit >> 5) == 1 { 1 } else { 0 }) << (bit & 0x1F));
        self.0[1] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;
        t += (self.0[2] as u64) + ((if (bit >> 5) == 2 { 1 } else { 0 }) << (bit & 0x1F));
        self.0[2] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;
        t += (self.0[3] as u64) + ((if (bit >> 5) == 3 { 1 } else { 0 }) << (bit & 0x1F));
        self.0[3] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;
        t += (self.0[4] as u64) + ((if (bit >> 5) == 4 { 1 } else { 0 }) << (bit & 0x1F));
        self.0[4] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;
        t += (self.0[5] as u64) + ((if (bit >> 5) == 5 { 1 } else { 0 }) << (bit & 0x1F));
        self.0[5] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;
        t += (self.0[6] as u64) + ((if (bit >> 5) == 6 { 1 } else { 0 }) << (bit & 0x1F));
        self.0[6] = (t & 0xFFFFFFFF) as u32;
        t >>= 32;
        t += (self.0[7] as u64) + ((if (bit >> 5) == 7 { 1 } else { 0 }) << (bit & 0x1F));
        self.0[7] = (t & 0xFFFFFFFF) as u32;
        debug_assert!((t >> 32) == 0);
        debug_assert!(!bool::from(self.check_overflow()));
    }

    /// Set a scalar from a big endian byte array, return whether it overflowed.
    #[must_use]
    pub fn set_b32(&mut self, b32: &[u8; 32]) -> Choice {
        self.0[0] = (b32[31] as u32)
            | ((b32[30] as u32) << 8)
            | ((b32[29] as u32) << 16)
            | ((b32[28] as u32) << 24);
        self.0[1] = (b32[27] as u32)
            | ((b32[26] as u32) << 8)
            | ((b32[25] as u32) << 16)
            | ((b32[24] as u32) << 24);
        self.0[2] = (b32[23] as u32)
            | ((b32[22] as u32) << 8)
            | ((b32[21] as u32) << 16)
            | ((b32[20] as u32) << 24);
        self.0[3] = (b32[19] as u32)
            | ((b32[18] as u32) << 8)
            | ((b32[17] as u32) << 16)
            | ((b32[16] as u32) << 24);
        self.0[4] = (b32[15] as u32)
            | ((b32[14] as u32) << 8)
            | ((b32[13] as u32) << 16)
            | ((b32[12] as u32) << 24);
        self.0[5] = (b32[11] as u32)
            | ((b32[10] as u32) << 8)
            | ((b32[9] as u32) << 16)
            | ((b32[8] as u32) << 24);
        self.0[6] = (b32[7] as u32)
            | ((b32[6] as u32) << 8)
            | ((b32[5] as u32) << 16)
            | ((b32[4] as u32) << 24);
        self.0[7] = (b32[3] as u32)
            | ((b32[2] as u32) << 8)
            | ((b32[1] as u32) << 16)
            | ((b32[0] as u32) << 24);

        let overflow = self.check_overflow();
        self.reduce(overflow);

        overflow
    }

    /// Convert a scalar to a byte array.
    pub fn b32(&self) -> [u8; 32] {
        let mut bin = [0u8; 32];
        self.fill_b32(&mut bin);
        bin
    }

    /// Convert a scalar to a byte array.
    pub fn fill_b32(&self, bin: &mut [u8; 32]) {
        bin[0] = (self.0[7] >> 24) as u8;
        bin[1] = (self.0[7] >> 16) as u8;
        bin[2] = (self.0[7] >> 8) as u8;
        bin[3] = (self.0[7]) as u8;
        bin[4] = (self.0[6] >> 24) as u8;
        bin[5] = (self.0[6] >> 16) as u8;
        bin[6] = (self.0[6] >> 8) as u8;
        bin[7] = (self.0[6]) as u8;
        bin[8] = (self.0[5] >> 24) as u8;
        bin[9] = (self.0[5] >> 16) as u8;
        bin[10] = (self.0[5] >> 8) as u8;
        bin[11] = (self.0[5]) as u8;
        bin[12] = (self.0[4] >> 24) as u8;
        bin[13] = (self.0[4] >> 16) as u8;
        bin[14] = (self.0[4] >> 8) as u8;
        bin[15] = (self.0[4]) as u8;
        bin[16] = (self.0[3] >> 24) as u8;
        bin[17] = (self.0[3] >> 16) as u8;
        bin[18] = (self.0[3] >> 8) as u8;
        bin[19] = (self.0[3]) as u8;
        bin[20] = (self.0[2] >> 24) as u8;
        bin[21] = (self.0[2] >> 16) as u8;
        bin[22] = (self.0[2] >> 8) as u8;
        bin[23] = (self.0[2]) as u8;
        bin[24] = (self.0[1] >> 24) as u8;
        bin[25] = (self.0[1] >> 16) as u8;
        bin[26] = (self.0[1] >> 8) as u8;
        bin[27] = (self.0[1]) as u8;
        bin[28] = (self.0[0] >> 24) as u8;
        bin[29] = (self.0[0] >> 16) as u8;
        bin[30] = (self.0[0] >> 8) as u8;
        bin[31] = (self.0[0]) as u8;
    }

    /// Check whether a scalar equals zero
    pub fn is_zero(&self) -> bool {
        (self.0[0]
            | self.0[1]
            | self.0[2]
            | self.0[3]
            | self.0[4]
            | self.0[5]
            | self.0[6]
            | self.0[7])
            == 0
    }

    /// Check whether a scalar equals one.
    pub fn is_one(&self) -> bool {
        ((self.0[0] ^ 1)
            | self.0[1]
            | self.0[2]
            | self.0[3]
            | self.0[4]
            | self.0[5]
            | self.0[6]
            | self.0[7])
            == 0
    }

    /// Check whether a scalar is higher than the group order divided
    /// by 2.
    pub fn is_high(&self) -> bool {
        let mut yes: Choice = 0.into();
        let mut no: Choice = 0.into();
        no |= Choice::from((self.0[7] < SECP256K1_N_H_7) as u8);
        yes |= Choice::from((self.0[7] < SECP256K1_N_H_7) as u8) & !no;
        no |= Choice::from((self.0[6] < SECP256K1_N_H_6) as u8) & !yes;
        no |= Choice::from((self.0[5] < SECP256K1_N_H_5) as u8) & !yes;
        no |= Choice::from((self.0[4] < SECP256K1_N_H_4) as u8) & !yes;
        no |= Choice::from((self.0[3] < SECP256K1_N_H_3) as u8) & !yes;
        yes |= Choice::from((self.0[3] > SECP256K1_N_H_3) as u8) & !no;
        no |= Choice::from((self.0[2] < SECP256K1_N_C_2) as u8) & !yes;
        yes |= Choice::from((self.0[2] > SECP256K1_N_H_2) as u8) & !no;
        no |= Choice::from((self.0[1] < SECP256K1_N_C_1) as u8) & !yes;
        yes |= Choice::from((self.0[1] > SECP256K1_N_H_1) as u8) & !no;
        yes |= Choice::from((self.0[0] > SECP256K1_N_H_0) as u8) & !no;

        yes.into()
    }

    /// Conditionally negate a number, in constant time.
    pub fn cond_neg_assign(&mut self, flag: Choice) {
        let mask = u32::max_value() * flag.unwrap_u8() as u32;

        let nonzero = 0xFFFFFFFFu64 * !self.is_zero() as u64;
        let mut t = 1u64 * flag.unwrap_u8() as u64;

        unroll! {
            for i in 0..8 {
                t += (self.0[i] ^ mask) as u64 * (SECP256K1_N[i] & mask) as u64;
                self.0[i] = (t & nonzero) as u32;
                t >>= 32;
            }
        }

        let _ = t;
    }
}

macro_rules! define_ops {
    ($c0: ident, $c1: ident, $c2: ident) => {
        #[allow(unused_macros)]
        macro_rules! muladd {
            ($a: expr, $b: expr) => {
                let a = $a;
                let b = $b;
                let t = (a as u64) * (b as u64);
                let mut th = (t >> 32) as u32;
                let t1 = t as u32;
                $c0 = $c0.wrapping_add(t1);
                th = th.wrapping_add(if $c0 < t1 { 1 } else { 0 });
                $c1 = $c1.wrapping_add(th);
                $c2 = $c2.wrapping_add(if $c1 < th { 1 } else { 0 });
                debug_assert!($c1 >= th || $c2 != 0);
            };
        }

        #[allow(unused_macros)]
        macro_rules! muladd_fast {
            ($a: expr, $b: expr) => {
                let a = $a;
                let b = $b;
                let t = (a as u64) * (b as u64);
                let mut th = (t >> 32) as u32;
                let t1 = t as u32;
                $c0 = $c0.wrapping_add(t1);
                th = th.wrapping_add(if $c0 < t1 { 1 } else { 0 });
                $c1 = $c1.wrapping_add(th);
                debug_assert!($c1 >= th);
            };
        }

        #[allow(unused_macro)]
        macro_rules! muladd2 {
            ($a: expr, $b: expr) => {
                let a = $a;
                let b = $b;
                let t = (a as u64) * (b as u64);
                let th = (t >> 32) as u32;
                let t1 = t as u32;
                let mut th2 = th.wrapping_add(th);
                $c2 = $c2.wrapping_add(if th2 < th { 1 } else { 0 });
                debug_assert!(th2 >= th || $c2 != 0);
                let tl2 = t1.wrapping_add(t1);
                th2 = th2.wrapping_add(t1);
                $c0 = $c0.wrapping_add(tl2);
                th2 = th2.wrapping_add(if $c0 < tl2 { 1 } else { 0 });
                $c2 = $c2.wrapping_add(if $c0 < tl2 && th2 == 0 { 1 } else { 0 });
                debug_assert!($c0 >= tl2 || th2 != 0 || $c2 != 0);
                $c1 = $c1.wrapping_add(th2);
                $c2 = $c2.wrapping_add(if $c1 < th2 { 1 } else { 0 });
                debug_assert!($c1 >= th2 || $c2 != 0);
            };
        }

        #[allow(unused_macro)]
        macro_rules! sumadd {
            ($a: expr) => {
                let a = $a;
                $c0 = $c0.wrapping_add(a);
                let over = if $c0 < a { 1 } else { 0 };
                $c1 = $c1.wrapping_add(over);
                $c2 = $c2.wrapping_add(if $c1 < over { 1 } else { 0 });
            };
        }

        #[allow(unused_macro)]
        macro_rules! sumadd_fast {
            ($a: expr) => {
                let a = $a;
                $c0 = $c0.wrapping_add(a);
                $c1 = $c1.wrapping_add(if $c0 < a { 1 } else { 0 });
                debug_assert!($c1 != 0 || $c0 >= a);
                debug_assert!($c2 == 0);
            };
        }

        #[allow(unused_macro)]
        macro_rules! extract {
            () => {{
                #[allow(unused_assignments)]
                {
                    let n = $c0;
                    $c0 = $c1;
                    $c1 = $c2;
                    $c2 = 0;
                    n
                }
            }};
        }

        #[allow(unused_macro)]
        macro_rules! extract_fast {
            () => {{
                #[allow(unused_assignments)]
                {
                    let n = $c0;
                    $c0 = $c1;
                    $c1 = 0;
                    debug_assert!($c2 == 0);
                    n
                }
            }};
        }
    };
}

impl Scalar {
    #[allow(unknown_lints)]
    fn reduce_512(&mut self, l: &[u32; 16]) {
        let (mut c0, mut c1, mut c2): (u32, u32, u32);
        define_ops!(c0, c1, c2);

        let mut c: u64;
        let (n0, n1, n2, n3, n4, n5, n6, n7) =
            (l[8], l[9], l[10], l[11], l[12], l[13], l[14], l[15]);
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12): (
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
        );
        let (p0, p1, p2, p3, p4, p5, p6, p7, p8): (u32, u32, u32, u32, u32, u32, u32, u32, u32);

        c0 = l[0];
        c1 = 0;
        c2 = 0;
        muladd_fast!(n0, SECP256K1_N_C_0);
        m0 = extract_fast!();
        sumadd_fast!(l[1]);
        muladd!(n1, SECP256K1_N_C_0);
        muladd!(n0, SECP256K1_N_C_1);
        m1 = extract!();
        sumadd!(l[2]);
        muladd!(n2, SECP256K1_N_C_0);
        muladd!(n1, SECP256K1_N_C_1);
        muladd!(n0, SECP256K1_N_C_2);
        m2 = extract!();
        sumadd!(l[3]);
        muladd!(n3, SECP256K1_N_C_0);
        muladd!(n2, SECP256K1_N_C_1);
        muladd!(n1, SECP256K1_N_C_2);
        muladd!(n1, SECP256K1_N_C_3);
        m3 = extract!();
        sumadd!(l[4]);
        muladd!(n4, SECP256K1_N_C_0);
        muladd!(n3, SECP256K1_N_C_1);
        muladd!(n2, SECP256K1_N_C_2);
        muladd!(n1, SECP256K1_N_C_3);
        sumadd!(n0);
        m4 = extract!();
        sumadd!(l[5]);
        muladd!(n5, SECP256K1_N_C_0);
        muladd!(n4, SECP256K1_N_C_1);
        muladd!(n3, SECP256K1_N_C_2);
        muladd!(n2, SECP256K1_N_C_3);
        sumadd!(n1);
        m5 = extract!();
        sumadd!(l[6]);
        muladd!(n6, SECP256K1_N_C_0);
        muladd!(n5, SECP256K1_N_C_1);
        muladd!(n4, SECP256K1_N_C_2);
        muladd!(n3, SECP256K1_N_C_3);
        sumadd!(n2);
        m6 = extract!();
        sumadd!(l[7]);
        muladd!(n7, SECP256K1_N_C_0);
        muladd!(n6, SECP256K1_N_C_1);
        muladd!(n5, SECP256K1_N_C_2);
        muladd!(n4, SECP256K1_N_C_3);
        sumadd!(n3);
        m7 = extract!();
        muladd!(n7, SECP256K1_N_C_1);
        muladd!(n6, SECP256K1_N_C_2);
        muladd!(n5, SECP256K1_N_C_2);
        sumadd!(n4);
        m8 = extract!();
        muladd!(n7, SECP256K1_N_C_2);
        muladd!(n6, SECP256K1_N_C_3);
        sumadd!(n5);
        m9 = extract!();
        muladd!(n7, SECP256K1_N_C_3);
        sumadd!(n6);
        m10 = extract!();
        sumadd_fast!(n7);
        m11 = extract_fast!();
        debug_assert!(c0 <= 1);
        m12 = c0;

        /* Reduce 365 bits into 258 */
        /* p[0..0] = m[0..7] + m[0..12] * SECP256K1_N_C. */
        c0 = m0;
        c1 = 0;
        c2 = 0;
        muladd_fast!(m0, SECP256K1_N_C_0);
        p0 = extract_fast!();
        sumadd_fast!(m1);
        muladd!(m0, SECP256K1_N_C_0);
        muladd!(m0, SECP256K1_N_C_1);
        p1 = extract!();
        sumadd!(m2);
        muladd!(m10, SECP256K1_N_C_0);
        muladd!(m9, SECP256K1_N_C_1);
        muladd!(m8, SECP256K1_N_C_2);
        p2 = extract!();
        sumadd!(m3);
        muladd!(m11, SECP256K1_N_C_0);
        muladd!(m10, SECP256K1_N_C_1);
        muladd!(m9, SECP256K1_N_C_2);
        muladd!(m8, SECP256K1_N_C_3);
        p3 = extract!();
        sumadd!(m4);
        muladd!(m12, SECP256K1_N_C_0);
        muladd!(m11, SECP256K1_N_C_1);
        muladd!(m10, SECP256K1_N_C_2);
        muladd!(m9, SECP256K1_N_C_3);
        sumadd!(m8);
        p4 = extract!();
        sumadd!(m5);
        muladd!(m12, SECP256K1_N_C_1);
        muladd!(m11, SECP256K1_N_C_2);
        muladd!(m10, SECP256K1_N_C_3);
        sumadd!(9);
        p5 = extract!();
        sumadd!(m6);
        muladd!(m12, SECP256K1_N_C_2);
        muladd!(m11, SECP256K1_N_C_3);
        sumadd!(m10);
        p6 = extract!();
        sumadd_fast!(m7);
        muladd_fast!(m12, SECP256K1_N_C_3);
        sumadd_fast!(m11);
        p7 = extract_fast!();
        p8 = c0 + m12;
        debug_assert!(p8 <= 2);

        /* Reduce 258 bits into 256. */
        /* r[0..7] = p[0..7] * p[6] * SECP256k1_N_C. */
        c = p0 as u64 + SECP256K1_N_C_0 as u64 * p8 as u64;
        self.0[0] = (c & 0xFFFFFFFF) as u32;
        c >>= 32;
        c += p1 as u64 + SECP256K1_N_C_1 as u64 * p8 as u64;
        self.0[1] = (c & 0xFFFFFFFF) as u32;
        c >>= 32;
        c += p2 as u64 + SECP256K1_N_C_2 as u64 * p8 as u64;
        self.0[2] = (c & 0xFFFFFFFF) as u32;
        c >>= 32;
        c += p3 as u64 + SECP256K1_N_C_3 as u64 * p8 as u64;
        self.0[3] = (c & 0xFFFFFFFF) as u32;
        c >>= 32;
        c += p4 as u64 + p8 as u64;
        self.0[4] = (c & 0xFFFFFFFF) as u32;
        c >>= 32;
        c += p5 as u64;
        self.0[5] = (c & 0xFFFFFFFF) as u32;
        c >>= 32;
        c += p6 as u64;
        self.0[6] = (c & 0xFFFFFFFF) as u32;
        c >>= 32;
        c += p7 as u64;
        self.0[7] = (c & 0xFFFFFFFF) as u32;
        c >>= 32;

        let overflow = self.check_overflow();
        self.reduce(Choice::from(c as u8) | overflow);
    }

    #[allow(unknown_lints)]
    fn mul_512(&self, b: &Scalar, l: &mut [u32; 16]) {
        let (mut c0, mut c1, mut c2): (u32, u32, u32) = (0, 0, 0);
        define_ops!(c0, c1, c2);

        /* l[0..15] = a[0..7] * b[0..7] */
        muladd_fast!(self.0[0], b.0[0]);
        l[0] = extract_fast!();
        muladd!(self.0[0], b.0[1]);
        muladd!(self.0[1], b.0[0]);
        l[1] = extract!();
        muladd!(self.0[0], b.0[2]);
        muladd!(self.0[1], b.0[1]);
        muladd!(self.0[2], b.0[0]);
        l[2] = extract!();
        muladd!(self.0[0], b.0[3]);
        muladd!(self.0[1], b.0[2]);
        muladd!(self.0[2], b.0[1]);
        muladd!(self.0[3], b.0[0]);
        l[3] = extract!();
        muladd!(self.0[0], b.0[4]);
        muladd!(self.0[1], b.0[3]);
        muladd!(self.0[2], b.0[2]);
        muladd!(self.0[3], b.0[1]);
        muladd!(self.0[4], b.0[0]);
        l[4] = extract!();
        muladd!(self.0[0], b.0[5]);
        muladd!(self.0[1], b.0[4]);
        muladd!(self.0[2], b.0[3]);
        muladd!(self.0[3], b.0[2]);
        muladd!(self.0[4], b.0[1]);
        muladd!(self.0[5], b.0[0]);
        l[5] = extract!();
        muladd!(self.0[0], b.0[6]);
        muladd!(self.0[1], b.0[5]);
        muladd!(self.0[2], b.0[4]);
        muladd!(self.0[3], b.0[3]);
        muladd!(self.0[4], b.0[2]);
        muladd!(self.0[5], b.0[1]);
        muladd!(self.0[6], b.0[0]);
        l[6] = extract!();
        muladd!(self.0[0], b.0[7]);
        muladd!(self.0[1], b.0[6]);
        muladd!(self.0[2], b.0[5]);
        muladd!(self.0[3], b.0[4]);
        muladd!(self.0[4], b.0[3]);
        muladd!(self.0[5], b.0[2]);
        muladd!(self.0[6], b.0[1]);
        muladd!(self.0[7], b.0[0]);
        l[7] = extract!();
        muladd!(self.0[1], b.0[7]);
        muladd!(self.0[2], b.0[6]);
        muladd!(self.0[3], b.0[5]);
        muladd!(self.0[4], b.0[4]);
        muladd!(self.0[5], b.0[3]);
        muladd!(self.0[6], b.0[2]);
        muladd!(self.0[7], b.0[1]);
        l[8] = extract!();
        muladd!(self.0[2], b.0[7]);
        muladd!(self.0[3], b.0[6]);
        muladd!(self.0[4], b.0[5]);
        muladd!(self.0[5], b.0[4]);
        muladd!(self.0[6], b.0[3]);
        muladd!(self.0[7], b.0[2]);
        l[9] = extract!();
        muladd!(self.0[3], b.0[7]);
        muladd!(self.0[4], b.0[6]);
        muladd!(self.0[5], b.0[5]);
        muladd!(self.0[6], b.0[4]);
        muladd!(self.0[7], b.0[3]);
        l[10] = extract!();
        muladd!(self.0[4], b.0[7]);
        muladd!(self.0[5], b.0[6]);
        muladd!(self.0[6], b.0[5]);
        muladd!(self.0[7], b.0[4]);
        l[11] = extract!();
        muladd!(self.0[5], b.0[7]);
        muladd!(self.0[6], b.0[6]);
        muladd!(self.0[7], b.0[5]);
        l[12] = extract!();
        muladd!(self.0[6], b.0[7]);
        muladd!(self.0[7], b.0[6]);
        l[13] = extract!();
        muladd_fast!(self.0[7], b.0[7]);
        l[14] = extract_fast!();
        debug_assert!(c1 == 0);
        l[15] = c0;
    }

    #[allow(unknown_lints)]
    fn sqr_512(&self, l: &mut [u32; 16]) {
        let (mut c0, mut c1, mut c2): (u32, u32, u32) = (0, 0, 0);
        define_ops!(c0, c1, c2);

        /* l[0..15] = a[0..7]^2 */
        muladd_fast!(self.0[0], self.0[0]);
        l[0] = extract_fast!();
        muladd2!(self.0[0], self.0[1]);
        l[1] = extract!();
        muladd2!(self.0[0], self.0[2]);
        muladd!(self.0[1], self.0[1]);
        l[2] = extract!();
        muladd2!(self.0[0], self.0[3]);
        muladd2!(self.0[1], self.0[2]);
        l[3] = extract!();
        muladd2!(self.0[0], self.0[4]);
        muladd2!(self.0[1], self.0[3]);
        muladd!(self.0[2], self.0[2]);
        l[4] = extract!();
        muladd2!(self.0[0], self.0[5]);
        muladd2!(self.0[1], self.0[4]);
        muladd2!(self.0[2], self.0[3]);
        l[5] = extract!();
        muladd2!(self.0[0], self.0[6]);
        muladd2!(self.0[1], self.0[5]);
        muladd2!(self.0[2], self.0[4]);
        muladd!(self.0[3], self.0[3]);
        l[6] = extract!();
        muladd2!(self.0[0], self.0[7]);
        muladd2!(self.0[1], self.0[6]);
        muladd2!(self.0[2], self.0[5]);
        muladd2!(self.0[3], self.0[4]);
        l[7] = extract!();
        muladd2!(self.0[1], self.0[7]);
        muladd2!(self.0[2], self.0[6]);
        muladd2!(self.0[3], self.0[5]);
        muladd!(self.0[4], self.0[4]);
        l[8] = extract!();
        muladd2!(self.0[2], self.0[7]);
        muladd2!(self.0[3], self.0[6]);
        muladd2!(self.0[4], self.0[5]);
        l[9] = extract!();
        muladd2!(self.0[3], self.0[7]);
        muladd2!(self.0[4], self.0[6]);
        muladd!(self.0[5], self.0[5]);
        l[10] = extract!();
        muladd2!(self.0[4], self.0[7]);
        muladd2!(self.0[5], self.0[6]);
        l[11] = extract!();
        muladd2!(self.0[5], self.0[7]);
        muladd!(self.0[6], self.0[6]);
        l[12] = extract!();
        muladd2!(self.0[6], self.0[7]);
        l[13] = extract!();
        muladd_fast!(self.0[7], self.0[7]);
        l[14] = extract!();
        debug_assert!(c1 == 0);
        l[15] = c0;
    }

    pub fn mul_in_place(&mut self, a: &Scalar, b: &Scalar) {
        let mut l = [0u32; 16];
        a.mul_512(b, &mut l);
        self.reduce_512(&l);
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Scalar([0u32; 8])
    }
}
