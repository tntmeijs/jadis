//! Represents the JVM .class format
//!
//! This module is used to add class format parsing functionality to Jadis
//! Do note that the actual file IO is not handled by this module

use crate::byte_reader::ByteReader;

/// JVM class file representation
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
}

impl ClassFile {
    /// Create a new class file structure from a class file binary blob
    pub fn new(reader: &mut ByteReader) -> Self {
        // Empty instance with default values so it can be constructed
        let mut instance = Self {
            magic: 0,
            minor_version: 0,
            major_version: 0,
        };

        // Start reading data into the instance
        instance.read_magic_number(reader);
        instance.read_version(reader);

        // Instance now contains all relevant information
        instance
    }

    /// Read the magic number from a class file
    fn read_magic_number(&mut self, reader: &mut ByteReader) {
        self.magic = reader.read_u32()
    }

    /// Read the major and minor version from a class file
    fn read_version(&mut self, reader: &mut ByteReader) {
        self.minor_version = reader.read_u16();
        self.major_version = reader.read_u16();
    }
}
