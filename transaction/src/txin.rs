use num_bigint::BigUint;

use crate::Tx;

pub struct TxIn {
    prev_tx: Tx,
    prev_index: BigUint,
}
