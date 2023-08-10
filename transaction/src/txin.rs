use std::io::{self, Read};

use num_bigint::BigUint;
use script::Script;

use crate::Tx;

pub struct TxIn {
    pub prev_tx: [u8; 32],
    pub prev_index: u32,
    pub script_sig: Option<Script>,
    pub sequence: u32,
}

impl TxIn {
    pub fn new(
        prev_tx: [u8; 32],
        prev_index: u32,
        script_sig: Option<Script>,
        sequence: u32,
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

    pub fn parse<R: Read>(stream: &mut R) -> Result<Self, io::Error> {
        let mut prev_tx = [0u8; 32];
        stream.read_exact(&mut prev_tx)?;

        let prev_index = {
            let mut buf = [0u8; 4];
            stream.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        let script_sig = Script::parse();

        let sequence = {
            let mut buf = [0u8; 4];
            stream.read_exact(&mut buf)?;
            u32::from_le_bytes(buf)
        };

        Ok(Self {
            prev_tx,
            prev_index,
            script_sig: Some(script_sig),
            sequence,
        })
    }
}
