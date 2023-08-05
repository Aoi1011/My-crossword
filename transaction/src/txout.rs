use num_bigint::BigUint;

pub struct TxOut {
    amount: u64,
    script_pubkey: BigUint,
}

impl TxOut {
    pub fn new(amount: u64, script_pubkey: BigUint) -> Self {
        Self {
            amount,
            script_pubkey,
        }
    }
}
