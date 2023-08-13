use std::io::{self, Cursor};

use bitcoin::consensus::encode::Decodable;
use elliptic_curves::helper::read_varint;
use txin::TxIn;
use txout::TxOut;

pub mod txin;
pub mod txout;

pub struct Tx {
    pub version: i32,
    pub tx_ins: Vec<TxIn>,
    pub tx_outs: Vec<TxOut>,
    pub locktime: u32,
    pub testnet: bool,
}

impl Tx {
    pub fn new(
        version: i32,
        tx_ins: Vec<TxIn>,
        tx_outs: Vec<TxOut>,
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

    pub fn parse(stream: &mut Cursor<Vec<u8>>, testnet: bool) -> Result<Self, io::Error> {
        // let mut stream = &stream[..];

        // let mut version = [0u8; 4];
        // stream.read_exact(&mut version)?;
        let version = i32::consensus_decode_from_finite_reader(stream).unwrap();

        let inputs = TxIn::parse(stream).unwrap();

        // let mut inputs = Vec::new();
        // for _ in 0..num_inputs {
        //     inputs.push(TxIn::parse(stream)?)
        // }

        // let num_outputs = read_varint(stream)?;
        let outputs = TxOut::parse(stream).unwrap();
        // for _ in 0..num_outputs {
        //     outputs.push(TxOut::parse(stream)?)
        // }

        // let mut locktime = [0u8; 4];
        // stream.read_exact(&mut locktime)?;

        Ok(Self {
            version,
            tx_ins: inputs,
            tx_outs: outputs,
            locktime: 0,
            testnet,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use hex::FromHex;

    use crate::Tx;

    #[test]
    fn test_parse_tx() {
        let hex_transaction = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e\
148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd\
205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4e\
a13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253\
dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd\
3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1c\
dc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a716012\
1035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567\
bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a4730440\
2204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08\
bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0\
172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375\
925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea\
0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23\
852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff272\
2eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1a\
b6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80\
df2b3eda8db57397088ac46430600";

        let stream = Vec::from_hex(hex_transaction).unwrap();
        let mut decoder = Cursor::new(stream);

        // let (rv, consumed) = deserialize_partial(&stream).unwrap();
        let tx_obj = Tx::parse(&mut decoder, false).unwrap();
        assert_eq!(tx_obj.tx_outs[1].value, 40000000);

        // let tx: Transaction = deserialize(&stream).unwrap();

        //         assert_eq!(
        //             tx_obj.tx_ins.unwrap()[1].script_sig,
        //             "304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008\
        // b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601 035d5c93d9ac9\
        // 6881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937"
        //         )
    }
}
