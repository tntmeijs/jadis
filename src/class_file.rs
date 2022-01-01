//! Represents the JVM .class format
//!
//! This module is used to add class format parsing functionality to Jadis
//! Do note that the actual file IO is not handled by this module

use crate::byte_reader::ByteReader;
use crate::constant_pool::{ConstantPoolInfo, Tag};
use crate::utils::{to_u16, to_u32};

const SIZE_BYTES_U16: usize = 2;
const SIZE_BYTES_U32: usize = 4;

/// JVM class file representation
pub struct ClassFile {
    /// Magic number - should always equal 0xCAFEBABE
    pub magic: u32,

    /// Bytecode minor version
    pub minor_version: u16,

    /// Bytecode major version
    pub major_version: u16,

    /// Number of items in the constant pool plus one
    pub constant_pool_count: u16,

    /// Constant pool
    pub constant_pool: Vec<ConstantPoolInfo>,
}

impl ClassFile {
    /// Create a new class file structure from a class file binary blob
    pub fn new(reader: &mut ByteReader) -> Self {
        // Empty instance with default values so it can be constructed
        let mut instance = Self {
            magic: 0,
            minor_version: 0,
            major_version: 0,
            constant_pool_count: 0,
            constant_pool: vec![],
        };

        // Start reading data into the instance
        instance.read_magic_number(reader);
        instance.read_version(reader);
        instance.read_constant_pool(reader);

        // Instance now contains all relevant information
        instance
    }

    /// Read the magic number from a class file
    fn read_magic_number(&mut self, reader: &mut ByteReader) {
        self.magic = to_u32(reader.read_n_bytes(SIZE_BYTES_U32))
    }

    /// Read the major and minor version from a class file
    fn read_version(&mut self, reader: &mut ByteReader) {
        self.minor_version = to_u16(reader.read_n_bytes(SIZE_BYTES_U16));
        self.major_version = to_u16(reader.read_n_bytes(SIZE_BYTES_U16));
    }

    /// Read the entire constant pool
    fn read_constant_pool(&mut self, reader: &mut ByteReader) {
        self.constant_pool_count = to_u16(reader.read_n_bytes(SIZE_BYTES_U16));

        // Index into the constant pool
        // The constant pool starts indexing at one, which is why this index starts at one as well
        let mut index = 1;

        // Read the entire constant pool
        while index < (self.constant_pool_count) {
            let info = ConstantPoolInfo::new(reader, index);

            // Long and double "occupy" two indices
            // See: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.5
            index += match info.tag {
                Tag::ConstantLong | Tag::ConstantDouble => 2,
                _ => 1,
            };

            self.constant_pool.push(info);
        }
    }
}
