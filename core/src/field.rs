#[derive(Debug, Clone, Copy)]
// Field element for secp256k1
pub struct Field {
    n: [u32; 10],
    magnitude: u32,
    normalized: bool,
}

impl Field {
    pub const fn new_raw(
        d9: u32,
        d8: u32,
        d7: u32,
        d6: u32,
        d5: u32,
        d4: u32,
        d3: u32,
        d2: u32,
        d1: u32,
        d0: u32,
    ) -> Self {
        Self {
            n: [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9],
            magnitude: 1,
            normalized: false,
        }
    }

    pub const fn new(
        d7: u32,
        d6: u32,
        d5: u32,
        d4: u32,
        d3: u32,
        d2: u32,
        d1: u32,
        d0: u32,
    ) -> Self {
        Self {
            n: [
                d0 & 0x3ffffff,
                (d0 >> 26) | ((d1 & 0xfffff) << 6),
                (d1 >> 20) | ((d2 & 0x3fff) << 12),
                (d2 >> 14) | ((d3 & 0xff) << 18),
                (d3 >> 8) | ((d4 & 0x3) << 24),
                (d4 >> 2) & 0x3ffffff,
                (d4 >> 28) | ((d5 & 0x3fffff) << 4),
                (d5 >> 22) | ((d6 & 0xffff) << 10),
                (d6 >> 16) | ((d7 & 0x3ff) << 16),
                (d7 >> 10),
            ],
            magnitude: 1,
            normalized: true,
        }
    }

    fn verify(&self) -> bool {
        let m = if self.normalized { 1 } else { 2 } * self.magnitude;
        let mut r = true;
        // TODO
        r
    }

    pub fn from_int(a: u32) -> Field {
        let mut f = Field::default();
        f.set_int(a);
        f
    }

    // Set a field element equal to small integer, Resulting field
    // element is normalized.
    pub fn set_int(&mut self, a: u32) {
        self.n = [a, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        self.magnitude = 1;
        self.normalized = true;
        debug_assert!(self.verify());
    }
}

impl Default for Field {
    fn default() -> Self {
        Self {
            n: [0u32; 10],
            magnitude: 0,
            normalized: true,
        }
    }
}
