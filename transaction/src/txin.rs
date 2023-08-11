use std::{
    io::{self, Read},
    mem,
};

use bitcoin::consensus::{encode::MAX_VEC_SIZE, Decodable};
use script::Script;

pub struct TxIn {
    pub prev_tx: u32,
    pub prev_index: u32,
    pub script_sig: Script,
    pub sequence: u32,
}

impl TxIn {
    pub fn new(prev_tx: u32, prev_index: u32, script_sig: Option<Script>, sequence: u32) -> Self {
        let script_sig = if script_sig.is_none() {
            Script::new(vec![])
        } else {
            script_sig.unwrap()
        };

        Self {
            prev_tx,
            prev_index,
            script_sig,
            sequence,
        }
    }

    pub fn parse<R: Read>(stream: &mut R) -> Result<Vec<Self>, io::Error> {
        let len = u64::consensus_decode_from_finite_reader(stream).unwrap();
        let max_capacity = MAX_VEC_SIZE / 4 / mem::size_of::<TxIn>();
        let mut ret = Vec::with_capacity(core::cmp::min(len as usize, max_capacity));
        for _ in 0..len {
            ret.push(Decodable::consensus_decode_from_finite_reader(stream).unwrap());
        }
        Ok(ret)
        // let mut prev_tx = [0u8; 32];
        // stream.read_exact(&mut prev_tx)?;

        // let prev_index = {
        //     let mut buf = [0u8; 4];
        //     stream.read_exact(&mut buf)?;
        //     u32::from_le_bytes(buf)
        // };

        // let script_sig = Script::parse();

        // let sequence = {
        //     let mut buf = [0u8; 4];
        //     stream.read_exact(&mut buf)?;
        //     u32::from_le_bytes(buf)
        // };

        // Ok(Self {
        //     prev_tx,
        //     prev_index,
        //     script_sig: Some(script_sig),
        //     sequence,
        // })
    }
}

impl Decodable for TxIn {
    fn consensus_decode_from_finite_reader<R: io::Read + ?Sized>(
        reader: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        Ok(TxIn {
            prev_tx: Decodable::consensus_decode_from_finite_reader(reader).unwrap(),
            prev_index: Decodable::consensus_decode_from_finite_reader(reader).unwrap(),
            script_sig: Decodable::consensus_decode_from_finite_reader(reader).unwrap(),
            sequence: Decodable::consensus_decode_from_finite_reader(reader).unwrap(),
        })
    }
}
