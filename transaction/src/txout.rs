use std::io::{self, Read};

use script::Script;

pub struct TxOut {
    pub amount: u64,
    pub script_pubkey: Script,
}

impl TxOut {
    pub fn new(amount: u64, script_pubkey: Script) -> Self {
        Self {
            amount,
            script_pubkey,
        }
    }

    /// Takes a byte stream and parses the tx_output at the start.
    /// Returns a TxOut object
    pub fn parse<R: Read>(stream: &mut R) -> Result<Self, io::Error> {
        let mut amount = [0u8; 8];
        stream.read_exact(&mut amount)?;

        let script_pubkey = Script::parse();

        Ok(Self {
            amount: u64::from_le_bytes(amount),
            script_pubkey,
        })
    }
}
