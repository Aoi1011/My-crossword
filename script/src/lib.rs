use bitcoin::consensus::Decodable;

pub struct Script(Vec<u8>);

pub mod op;

impl Script {
    pub fn new(cmds: Vec<u8>) -> Self {
        Self(cmds)
    }

    pub fn parse() -> Self {
        Self(Vec::new())
    }
}

impl Decodable for Script {
    fn consensus_decode_from_finite_reader<R: std::io::Read + ?Sized>(
        _reader: &mut R,
    ) -> Result<Self, bitcoin::consensus::encode::Error> {
        Ok(Self(Vec::new()))
    }
}

#[cfg(test)]
mod tests {}
