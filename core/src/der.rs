use crate::error::Error;

pub struct SignatureArray([u8; 6 + 33 + 33], usize);

impl SignatureArray {
    pub fn new(size: usize) -> Self {
        SignatureArray([0u8; 6 + 33 + 33], size)
    }

    pub fn len(&self) -> usize {
        self.1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl AsRef<[u8]> for SignatureArray {
    fn as_ref(&self) -> &[u8] {
        &self.0[..self.1]
    }
}

impl AsMut<[u8]> for SignatureArray {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0[..self.1]
    }
}

pub struct Decoder<'a>(&'a [u8], usize);

impl<'a> Decoder<'a> {
    pub fn new(arr: &'a [u8]) -> Self {
        Decoder(arr, 0)
    }

    pub fn remaining_len(&self) -> usize {
        self.0.len() - self.1
    }

    pub fn read(&mut self) -> Result<u8, Error> {
        if self.1 >= self.0.len() {
            Err(Error::InvalidSignature)
        } else {
            let v = self.0[self.1];
            self.1 += 1;
            Ok(v)
        }
    }

    pub fn peek(&self, forward: usize) -> Result<u8, Error> {
        if self.1 + forward >= self.0.len() {
            Err(Error::InvalidSignature)
        } else {
            let v = self.0[self.1 + forward];
            Ok(v)
        }
    }

    pub fn peek_slice(&self, len: usize) -> Result<&[u8], Error> {
        if (len == 0 && self.1 >= self.0.len()) || self.1 + len > self.0.len() {
            Err(Error::InvalidSignature)
        } else {
            let v = &self.0[self.1..(self.1 + len)];
            Ok(v)
        }
    }

    pub fn skip(&mut self, len: usize) -> Result<(), Error> {
        if (len == 0 && self.1 >= self.0.len()) || self.1 + len >  self.0.len() {
            Err(Error::InvalidSignature)
        } else {
            self.1 += len;
            Ok(())
        }
    }

    pub fn read_constructed_sequence(&mut self) -> Result<(), Error> {
        let v = self.read()?;
        if v == 0x30 {
            Ok(())
        } else {
            Err(Error::InvalidSignature)
        }
    }


}
