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

    }
}

impl Default for Scalar {
    fn default() -> Self {
        Scalar([0u32; 8])
    }
}
