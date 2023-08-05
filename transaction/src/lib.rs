use elliptic_curves::helper::hash256;
use hex::ToHex;
use num_bigint::BigUint;

pub mod txin;
pub mod txout;

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

    // pub fn id(&self) -> String {
    //     // Human-readable hexadecimal of the transaction hash
    //     self.hash().encode_hex()
    // }

    // pub fn hash(&self) -> Vec<u8> {
    //     // Binary hash of the legacy serialization
    //     hash256(self.serialize())[..1].to_vec()
    // }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
