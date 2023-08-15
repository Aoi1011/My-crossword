use std::{
    collections::HashMap,
    io::{self, BufReader, ErrorKind, Write},
    net::TcpStream,
};

use crate::{
    message::Message,
    ping_message::PING_COMMAND,
    pong_message::PongMessage,
    ver_ack_message::VerAckMessage,
    version_message::{VersionMessage, VERSION_COMMAND},
    NetworkEnvelope,
};

pub struct SimpleNode {
    testnet: bool,
    logging: bool,
    socket: TcpStream,
    stream: BufReader<TcpStream>,
}

impl SimpleNode {
    pub fn new(host: String, port: Option<u16>, testnet: bool, logging: bool) -> io::Result<Self> {
        let resolved_port = port.unwrap_or(if testnet { 18333 } else { 8333 });
        // let mut addr = host;
        // addr.extend([resolved_port.to_string()]);
        let socket = TcpStream::connect("127.0.0.1:18333")?;
        let stream = BufReader::new(socket.try_clone()?);

        Ok(SimpleNode {
            testnet,
            logging,
            socket,
            stream,
        })
    }

    pub fn send<T>(&mut self, message: &T) -> io::Result<()>
    where
        T: Message,
    {
        let command = b"verack";
        let envelope = NetworkEnvelope::new(command.to_vec(), message.serialize(), self.testnet);

        if self.logging {
            println!("sending: {:?}", envelope);
        }

        self.socket.write_all(&envelope.serialize())?;
        self.socket.flush()?;

        Ok(())
    }

    pub fn read(&mut self) -> io::Result<NetworkEnvelope> {
        let envelope = NetworkEnvelope::parse(&mut self.stream.buffer().to_vec(), self.testnet)?;
        if self.logging {
            println!("receiving: {:?}", envelope);
        }

        Ok(envelope)
    }

    pub fn wait_for<T>(&mut self, message_classes: &[T]) -> io::Result<Box<T>>
    where
        T: Message,
    {
        let mut command_to_class = HashMap::new();
        for message_class in message_classes.into_iter() {
            command_to_class.insert(<T as Message>::command(), message_class);
        }

        let mut command: String = String::new();
        while !command_to_class.contains_key(&command) {
            let envelope = self.read()?;
            command = String::from_utf8(envelope.command)
                .map_err(|_utf8_error| io::Error::new(ErrorKind::InvalidData, "Invalid Data"))?;
            match command.as_str() {
                VERSION_COMMAND => {
                    self.send(&VerAckMessage::new())?;
                }
                PING_COMMAND => {
                    self.send(&PongMessage::new(envelope.payload))?;
                }
                _ => {}
            }
        }

        // let message_class = command_to_class[&command];
        let envelope = self.read()?;
        Ok(Box::new(<T as Message>::parse(envelope.stream())))
    }

    pub fn handshake(&mut self) -> Result<(), io::Error> {
        let version = VersionMessage::default();
        self.send(&version)?;
        self.wait_for(&vec![VerAckMessage::new()])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleNode;

    #[test]
    fn test_handshake() {
        let host = "127.0.0.1";
        let port = 18333;
        let mut node = SimpleNode::new(host.to_string(), Some(port), true, true).unwrap();
        node.handshake().unwrap();
    }
}
