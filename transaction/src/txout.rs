use std::{
    io::{self, Read},
    mem,
};

use bitcoin::consensus::{encode::MAX_VEC_SIZE, Decodable};
use script::Script;

pub struct TxOut {
    pub value: u64,
    pub script_pubkey: Script,
}

impl TxOut {
    pub fn new(value: u64, script_pubkey: Script) -> Self {
        Self {
            value,
            script_pubkey,
        }
    }

    /// Takes a byte stream and parses the tx_output at the start.
    /// Returns a TxOut object
    pub fn parse<R: Read>(stream: &mut R) -> Result<Vec<Self>, io::Error> {
        let len = u64::consensus_decode_from_finite_reader(stream).unwrap();
        let max_capacity = MAX_VEC_SIZE / 4 / mem::size_of::<TxOut>();
        let mut ret = Vec::with_capacity(core::cmp::min(len as usize, max_capacity));
        for _ in 0..len {
            ret.push(Decodable::consensus_decode_from_finite_reader(stream).unwrap());
        }
        Ok(ret)
        // let mut amount = [0u8; 8];
        // stream.read_exact(&mut amount)?;

        // let script_pubkey = Script::parse();

        // Ok(Self {
        //     amount: u64::from_le_bytes(amount),
        //     script_pubkey,
        // })
    }
}

impl Decodable for TxOut {
    fn consensus_decode_from_finite_reader<R: io::Read + ?Sized>(
        reader: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        Ok(Self {
            value: Decodable::consensus_decode_from_finite_reader(reader).unwrap(),
            script_pubkey: Decodable::consensus_decode_from_finite_reader(reader).unwrap(),
        })
    }
}
