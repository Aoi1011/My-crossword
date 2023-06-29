use std::alloc::{alloc, Layout};

use crate::{
    field::Field,
    group::{set_table_gej_var, Affine, AffineStorage, Jacobian, AFFINE_G},
    scalar::Scalar,
};

pub const WINDOW_A: usize = 5;
pub const WINDOW_G: usize = 16;
pub const ECMULT_TABLE_SIZE_A: usize = 1 << (WINDOW_A - 2);
pub const ECMULT_TABLE_SIZE_G: usize = 1 << (WINDOW_G - 2);
pub const WNAF_BITS: usize = 256;

fn odd_multiples_table_storage_var(pre: &mut [AffineStorage], a: &Jacobian) {
    let mut prej: Vec<Jacobian> = Vec::with_capacity(pre.len());
    for _ in 0..pre.len() {
        prej.push(Jacobian::default());
    }
    let mut prea: Vec<Affine> = Vec::with_capacity(pre.len());
    for _ in 0..pre.len() {
        prea.push(Affine::default());
    }
    let mut zr: Vec<Field> = Vec::with_capacity(pre.len());
    for _ in 0..pre.len() {
        zr.push(Field::default());
    }

    odd_multiple_table(&mut prej, &mut zr, a);
    set_table_gej_var(&mut prea, &prej, &zr);

    for i in 0..pre.len() {
        pre[i] = prea[i].into();
    }
}

/// Context for accelerating the computation of a*P + b*G
pub struct ECMultContext {
    pre_g: [AffineStorage; ECMULT_TABLE_SIZE_G],
}

impl ECMultContext {
    /// Create a new 'ECMultContext' from raw values.
    ///
    /// # Safety
    /// The function is unsafe because incorrect value of 'pre_g' can lead to
    /// crypto logic failure. You most likely do not want to use this function,
    /// but 'ECMulContext::new_boxed'.
    pub const unsafe fn new_from_raw(pre_g: [AffineStorage; ECMULT_TABLE_SIZE_G]) -> Self {
        Self { pre_g }
    }

    /// Inspect raw values of 'ECMulContext'.
    pub fn inspect_raw(&self) -> &[AffineStorage; ECMULT_TABLE_SIZE_G] {
        &self.pre_g
    }

    /// Generate a new 'ECMulContext' on the heap. Note that this function is expensive.
    pub fn new_boxed() -> Box<Self> {
        let mut this = unsafe {
            let ptr = alloc(Layout::new::<ECMultContext>()) as *mut ECMultContext;
            let mut this = Box::from_raw(ptr);

            for i in 0..ECMULT_TABLE_SIZE_G {
                this.pre_g[i] = AffineStorage::default();
            }

            this
        };

        let mut gj = Jacobian::default();
        gj.set_ge(&AFFINE_G);
        odd_multiples_table_storage_var(&mut this.pre_g, &gj);

        this
    }
}

/// Set a batch of group elements equal to the inputs given in jacobian
/// coordinates. Not constant time
pub fn set_all_gej_var(a: &[Jacobian]) -> Vec<Affine> {
    let mut az: Vec<Field> = Vec::with_capacity(a.len());
    for point in a {
        if !point.is_infinity() {
            az.push(point.z);
        }
    }
    let azi: Vec<Field> = inv_all_var(&az);

    let mut ret = vec![Affine::default(); a.len()];

    let mut count = 0;
    for i in 0..a.len() {
        ret[i].infinity = a[i].infinity;
        if !a[i].is_infinity() {
            ret[i].set_gej_zinv(&a[i], &azi[count]);
            count += 1;
        }
    }

    ret
}

/// Calculate the (modular) invarses of a batch of field
/// elements. Requires the inputs' magnitudes to be at most 8. The
/// output magnitudes are 1 (but not guaranteed to be
/// normalized).
pub fn inv_all_var(fields: &[Field]) -> Vec<Field> {
    if fields.is_empty() {
        return Vec::new();
    }

    let mut ret = Vec::with_capacity(fields.len());
    ret.push(fields[0]);

    for i in 1..fields.len() {
        ret.push(Field::default());
        ret[i] = ret[i - 1] * fields[i];
    }

    let mut u = ret[fields.len() - 1].inv_var();

    for i in (1..fields.len()).rev() {
        let j = i;
        let i = i - 1;
        ret[j] = ret[i] * u;
        u *= fields[j];
    }

    ret[0] = u;
    ret
}

const GEN_BLIND: Scalar = Scalar([
    2217680822, 850875797, 1046150361, 1330484644, 4015777837, 2466086288, 2052467175, 2084507480,
]);

const GEN_INITIAL: Jacobian = Jacobian {
    x: Field::new_raw(
        86608, 43357028, 207667908, 262670128, 142222828, 38529388, 267186148, 45417712, 115291924,
        13447464,
    ),
    y: Field::new_raw(
        12696548, 208302564, 112025180, 191752716, 143238548, 145482948, 228906000, 69755164,
        243572800, 210897016,
    ),
    z: Field::new_raw(
        3685368, 75404844, 20246216, 5748944, 73206666, 107661790, 110806176, 73488774, 5707384,
        104448710,
    ),
    infinity: false,
};

/// Context for accelerating the computation of a * G
pub struct ECMultGenContext {
    prec: [[AffineStorage; 16]; 64],
    blind: Scalar,
    initial: Jacobian,
}

impl ECMultGenContext {
    /// Create a new 'ECMultGenContext' from raw value.
    ///
    /// # Safety
    /// The function is unsafe because incorrect value of 'pre_g' can lead to
    /// crypto logic failure. You most likely do not want to use this function,
    /// but 'ECMulContext::new_boxed'.
    pub const unsafe fn new_from_raw(prec: [[AffineStorage; 16]; 64]) -> Self {
        Self {
            prec,
            blind: GEN_BLIND,
            initial: GEN_INITIAL,
        }
    }

    /// Inspect 'ECMultGenContext' values.
    pub fn inspect_raw(&self) -> &[[AffineStorage; 16]; 64] {
        &self.prec
    }

    pub fn new_boxed() -> Box<Self> {
        let mut this = unsafe {
            let ptr = alloc(Layout::new::<ECMultGenContext>()) as *mut ECMultGenContext;
            let mut this = Box::from_raw(ptr);

            for j in 0..64 {
                for i in 0..16 {
                    this.prec[j][i] = AffineStorage::default();
                }
            }

            this.blind = GEN_BLIND;
            this.initial = GEN_INITIAL;

            this
        };

        let mut gj = Jacobian::default();
        gj.set_ge(&AFFINE_G);

        // Construct a group element with no known corresponding scalar (nothing up my sleave).
        let mut nums_32 = [0u8; 32];
        debug_assert!(b"The scalar for this x is unknown".len() == 32);
        for (i, v) in b"The scalar for this x is unknown".iter().enumerate() {
            nums_32[i] = *v;
        }
        let mut nums_x = Field::default();
        assert!(nums_x.set_b32(&nums_32));
        let mut nums_ge = Affine::default();
        assert!(nums_ge.set_xo_var(&nums_x, false));
        let mut nums_gej = Jacobian::default();
        nums_gej.set_ge(&nums_ge);
        nums_gej = nums_gej.add_ge_var(&AFFINE_G, None);

        // Compute prec.
        let mut precj: Vec<Jacobian> = Vec::with_capacity(1024);
        for _ in 0..1024 {
            precj.push(Jacobian::default());
        }
        let mut gbase = gj;
        let mut numsbase = nums_gej;
        for j in 0..64 {
            precj[j * 16] = numsbase;
            for i in 1..16 {
                precj[j * 16 + i] = precj[j * 16 + i - 1].add_var(&gbase, None);
            }
            for _ in 0..4 {
                gbase = gbase.double_var(None);
            }
            numsbase = numsbase.double_var(None);
            if j == 62 {
                numsbase = numsbase.neg();
                numsbase = numsbase.add_var(&nums_gej, None);
            }
        }
        let prec = set_all_gej_var(&precj);

        for j in 0..64 {
            for i in 0..16 {
                let pg: AffineStorage = prec[j * 16 + i].into();
                this.prec[j][i] = pg;
            }
        }
        this
    }
}

pub fn odd_multiple_table(prej: &mut [Jacobian], zr: &mut [Field], a: &Jacobian) {}
