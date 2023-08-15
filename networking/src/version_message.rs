use std::{time::{SystemTime, UNIX_EPOCH}, io::BufReader};

use elliptic_curves::helper::{encode_varint, int_to_little_endian};
use rand::prelude::*;

use crate::message::Message;

pub const VERSION_COMMAND: &str = "version";

pub struct VersionMessage {
    pub version: u32,
    pub services: u32,
    pub timestamp: u64,
    pub receiver_services: u32,
    pub receiver_ip: Vec<u8>,
    pub receiver_port: u16,
    pub sender_services: u32,
    pub sender_ip: Vec<u8>,
    pub sender_port: u16,
    pub nonce: Vec<u8>,
    pub user_agent: Vec<u8>,
    pub latest_block: u32,
    pub relay: bool,
}

impl VersionMessage {
    pub fn new(
        version: u32,
        services: u32,
        timestamp: Option<u64>,
        receiver_services: u32,
        receiver_ip: Vec<u8>,
        receiver_port: u16,
        sender_services: u32,
        sender_ip: Vec<u8>,
        sender_port: u16,
        nonce: Option<Vec<u8>>,
        user_agent: Vec<u8>,
        latest_block: u32,
        relay: bool,
    ) -> Self {
        let timestamp = if timestamp.is_none() {
            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("");
            current_time.as_secs()
        } else {
            timestamp.unwrap()
        };

        let nonce = if nonce.is_none() {
            let mut rng = thread_rng();
            let n = rng.gen_range(0..2_u128.pow(64));
            int_to_little_endian(n, 8)
        } else {
            nonce.unwrap()
        };

        Self {
            version,
            services,
            timestamp,
            receiver_services,
            receiver_ip,
            receiver_port,
            sender_services,
            sender_ip,
            sender_port,
            nonce,
            user_agent,
            latest_block,
            relay,
        }
    }

}

impl Message for VersionMessage {
    fn command() -> String {
        VERSION_COMMAND.to_string()
    }

    fn parse(stream: Vec<u8>) -> Self {
        Self::default()
    }

    fn serialize(&self) -> Vec<u8> {
        let mut result = int_to_little_endian(self.version as u128, 4);

        result.append(&mut int_to_little_endian(self.services as u128, 8));

        result.append(&mut int_to_little_endian(self.timestamp as u128, 8));

        result.append(&mut int_to_little_endian(self.receiver_services as u128, 8));

        let mut padding1 = vec![0u8; 10];
        result.append(&mut padding1);

        let padding2 = b"\xff\xff";
        result.append(&mut padding2.to_vec());

        let mut receiver_ip_clone = self.receiver_ip.clone();
        result.append(&mut receiver_ip_clone);

        let receiver_port_clone = self.receiver_port.to_be_bytes();
        result.append(&mut receiver_port_clone.to_vec());

        result.append(&mut int_to_little_endian(self.sender_services as u128, 8));

        let mut padding1 = vec![0u8; 10];
        result.append(&mut padding1);

        let padding2 = b"\xff\xff";
        result.append(&mut padding2.to_vec());

        let mut sender_ip_clone = self.sender_ip.clone();
        result.append(&mut sender_ip_clone);

        let sender_port_clone = self.sender_port.to_be_bytes();
        result.append(&mut sender_port_clone.to_vec());

        let mut nonce_clone = self.nonce.clone();
        result.append(&mut nonce_clone);

        result.append(&mut encode_varint(self.user_agent.len() as u128));

        let mut user_agent_clone = self.user_agent.clone();
        result.append(&mut user_agent_clone);

        result.append(&mut int_to_little_endian(self.latest_block as u128, 4));

        if self.relay {
            result.append(&mut b"\x01".to_vec());
        } else {
            result.append(&mut b"\x00".to_vec());
        }

        result
    }
}

impl Default for VersionMessage {
    fn default() -> Self {
        let version = 70015;
        let services = 0;
        let timestamp = Some(0);
        let receiver_services = 0;
        let receiver_ip = b"\x00\x00\x00\x00";
        let receiver_port = 8333;
        let sender_services = 0;
        let sender_ip = b"\x00\x00\x00\x00";
        let sender_port = 8333;
        let nonce = Some(vec![0u8; 8]);
        let user_agent = b"/programmingbitcoin:0.1/";
        let latest_block = 0;
        let relay = false;

        VersionMessage::new(
            version,
            services,
            timestamp,
            receiver_services,
            receiver_ip.to_vec(),
            receiver_port,
            sender_services,
            sender_ip.to_vec(),
            sender_port,
            nonce,
            user_agent.to_vec(),
            latest_block,
            relay,
        )
    }
}

#[cfg(test)]
mod tests {
    use hex::ToHex;

    use crate::message::Message;

    use super::VersionMessage;

    #[test]
    fn test_serialize() {
        let version = 70015;
        let services = 0;
        let timestamp = Some(0);
        let receiver_services = 0;
        let receiver_ip = b"\x00\x00\x00\x00";
        let receiver_port = 8333;
        let sender_services = 0;
        let sender_ip = b"\x00\x00\x00\x00";
        let sender_port = 8333;
        let nonce = Some(vec![0u8; 8]);
        let user_agent = b"/programmingbitcoin:0.1/";
        let latest_block = 0;
        let relay = false;

        let v = VersionMessage::new(
            version,
            services,
            timestamp,
            receiver_services,
            receiver_ip.to_vec(),
            receiver_port,
            sender_services,
            sender_ip.to_vec(),
            sender_port,
            nonce,
            user_agent.to_vec(),
            latest_block,
            relay,
        );

        assert_eq!(v.serialize().encode_hex::<String>(), "7f11010000000000000000000000000000000000000000000000000000000000000000000000ffff00000000208d000000000000000000000000000000000000ffff00000000208d0000000000000000182f70726f6772616d6d696e67626974636f696e3a302e312f0000000000");
    }
}
