use std::io::{self, Read};

use elliptic_curves::helper::{hash256, int_to_little_endian, little_endian_bytes_to_u64};

mod get_headers_message;
mod headers_message;
mod message;
mod ping_message;
mod pong_message;
mod simple_node;
mod ver_ack_message;
mod version_message;

const NETWORK_MAGIC: &[u8; 4] = b"\xf9\xbe\xb4\xd9";
const TESTNET_NETWORK_MAGIC: &[u8; 4] = b"\x0b\x11\x09\x07";

#[derive(Debug)]
pub struct NetworkEnvelope {
    pub command: Vec<u8>,
    pub payload: Vec<u8>,
    pub magic: Vec<u8>,
}

impl NetworkEnvelope {
    pub fn new(command: Vec<u8>, payload: Vec<u8>, testnet: bool) -> Self {
        let magic = if testnet {
            TESTNET_NETWORK_MAGIC.to_vec()
        } else {
            NETWORK_MAGIC.to_vec()
        };

        Self {
            command,
            payload,
            magic,
        }
    }

    pub fn parse(s: &mut Vec<u8>, testnet: bool) -> Result<Self, io::Error> {
        let mut stream = &s[..];
        let mut magic = [0u8; 4];
        stream.read_exact(&mut magic)?;

        if magic.is_empty() {
            return Err(io::ErrorKind::NotFound.into());
        }

        let expected_magic = if testnet {
            TESTNET_NETWORK_MAGIC.to_vec()
        } else {
            NETWORK_MAGIC.to_vec()
        };

        if magic.to_vec() != expected_magic {
            return Err(io::ErrorKind::InvalidData.into());
        }

        let mut command = [0u8; 12];
        stream.read_exact(&mut command)?;
        let command = command
            .iter()
            .cloned()
            .take_while(|&byte| byte != 0)
            .collect::<Vec<u8>>();

        let mut paylaod_length = [0u8; 4];
        stream.read_exact(&mut paylaod_length)?;
        let payload_length = little_endian_bytes_to_u64(&paylaod_length);

        let mut checksum = [0u8; 4];
        stream.read_exact(&mut checksum)?;

        let mut payload = vec![0u8; payload_length as usize];
        stream.read_exact(&mut payload)?;

        let calculated_checksum = &hash256(payload.as_slice())[..4];
        if calculated_checksum != checksum {
            return Err(io::ErrorKind::InvalidData.into());
        }

        Ok(Self {
            command,
            payload,
            magic: magic.to_vec(),
        })
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut result = self.magic.clone();

        let mut command_clone = self.command.clone();
        result.append(&mut command_clone);

        let mut padding = vec![0u8; 12 - self.command.len()];
        result.append(&mut padding);

        result.append(&mut int_to_little_endian(self.payload.len() as u128, 4));

        let hashed_payload = &hash256(&self.payload)[..4];
        result.append(&mut hashed_payload.to_vec());

        let mut payload_clone = self.payload.clone();
        result.append(&mut payload_clone);

        result
    }

    pub fn stream(&self) -> Vec<u8> {
        self.payload.clone()
    }
}

#[cfg(test)]
mod tests {
    use hex::FromHex;

    use crate::NetworkEnvelope;

    #[test]
    fn test_parse() {
        let mut msg = Vec::from_hex("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
        let mut envelope = NetworkEnvelope::parse(&mut msg, false).unwrap();

        assert_eq!(envelope.command, b"verack");
        assert_eq!(envelope.payload, b"");

        msg = Vec::from_hex("f9beb4d976657273696f6e0000000000650000005f1a69d2721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001").unwrap();
        envelope = NetworkEnvelope::parse(&mut msg, false).unwrap();

        assert_eq!(envelope.command, b"version");
        assert_eq!(envelope.payload, msg[24..]);
    }

    #[test]
    fn test_serialize() {
        let mut msg = Vec::from_hex("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
        let mut envelope = NetworkEnvelope::parse(&mut msg, false).unwrap();

        assert_eq!(envelope.serialize(), msg);

        msg = Vec::from_hex("f9beb4d976657273696f6e0000000000650000005f1a69d2721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001").unwrap();
        envelope = NetworkEnvelope::parse(&mut msg, false).unwrap();

        assert_eq!(envelope.serialize(), msg);
    }

    #[test]
    fn test_network() {}
}
