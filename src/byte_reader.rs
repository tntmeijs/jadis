//! Simplifies reading bytes from binary files
//!
//! This module contains all functionality necessary to read binary data from disk.
//! It is essentially a wrapper around the low-level IO functions provided by Rust.

/// Binary file reader
pub struct ByteReader {
    /// Binary data as bytes
    data: Vec<u8>,

    /// Current read index into the byte buffer
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
}
