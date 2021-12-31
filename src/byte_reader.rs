//! Simplifies reading bytes from binary files
//!
//! This module contains all functionality necessary to read binary data from disk.
//! It is essentially a wrapper around the low-level IO functions provided by Rust.

/// Binary file reader
pub struct ByteReader {
    data: Vec<u8>,
    position: usize,
}

impl ByteReader {
    /// Create a new byte reader instance
    pub fn new(path: &str) -> Self {
        let data = match std::fs::read(path) {
            Ok(file) => file,
            Err(error) => panic!("Error opening file: {}: {}", path, error),
        };

        Self { data, position: 0 }
    }

    /// Read N bytes from the current position in the binary blob
    pub fn read_n_bytes(&mut self, n: usize) -> &[u8] {
        let from = self.position;
        let to = self.position + n;
        self.position += n;

        let data = match self.data.get(from..to) {
            Some(data) => data,
            None => panic!("Unable to read {} bytes from the binary blob", n),
        };

        data
    }

    /// Read a u16 from the current pointer into the binary blob
    pub fn read_u16(&mut self) -> u16 {
        let bytes = self.read_n_bytes(std::mem::size_of::<u16>());
        (bytes[0] as u16) << 8 | bytes[1] as u16
    }

    /// Read a u32 from the current pointer into the binary blob
    pub fn read_u32(&mut self) -> u32 {
        let bytes = self.read_n_bytes(std::mem::size_of::<u32>());
        (bytes[0] as u32) << 24 | (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | bytes[3] as u32
    }
}
