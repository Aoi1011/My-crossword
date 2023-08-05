use num_bigint::BigUint;
use script::Script;

use crate::Tx;

pub struct TxIn {
    prev_tx: Tx,
    prev_index: BigUint,
    script_sig: Option<Script>,
    sequence: BigUint,
}

impl TxIn {
    pub fn new(
        prev_tx: Tx,
        prev_index: BigUint,
        script_sig: Option<Script>,
        sequence: BigUint,
    ) -> Self {
        let script_sig = if script_sig.is_none() {
            Script::new()
        } else {
            script_sig.unwrap()
        };

        Self {
            prev_tx,
            prev_index,
            script_sig: Some(script_sig),
            sequence,
        }
    }
}
