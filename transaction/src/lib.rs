use num_bigint::BigUint;

pub mod txin;

pub struct Tx {
    version: u32,
    tx_ins: Vec<BigUint>,
    tx_outs: Vec<BigUint>,
    locktime: u32,
    testnet: bool,
}

impl Tx {
    pub fn new(
        version: u32,
        tx_ins: Vec<BigUint>,
        tx_outs: Vec<BigUint>,
        locktime: u32,
        testnet: bool,
    ) -> Self {
        Self {
            version,
            tx_ins,
            tx_outs,
            locktime,
            testnet,
        }
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
