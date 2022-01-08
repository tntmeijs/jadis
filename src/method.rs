//! Provides functionality to simplify working with method structures
//!
//! Reference: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.6

use crate::{
    access_flags::MethodAccessFlags, attribute::AttributeInfo, byte_reader::ByteReader,
    constant_pool::ConstantPoolContainer, utils::to_u16,
};

/// Represents a method on a class or interface
pub struct MethodInfo {
    pub access_flags: Vec<MethodAccessFlags>,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    /// Create a new method from a class file binary blob
    pub fn new(reader: &mut ByteReader, constant_pool: &ConstantPoolContainer) -> Self {
        let access_flags = Self::read_access_flags(reader);
        let name_index = to_u16(reader.read_n_bytes(2));
        let descriptor_index = to_u16(reader.read_n_bytes(2));
        let attributes = Self::read_attributes(reader, constant_pool);

        Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }

    /// Read field access flags
    fn read_access_flags(reader: &mut ByteReader) -> Vec<MethodAccessFlags> {
        let bitmask = to_u16(reader.read_n_bytes(2));
        MethodAccessFlags::from_u16(bitmask)
    }

    /// Read field attributes
    fn read_attributes(
        reader: &mut ByteReader,
        constant_pool: &ConstantPoolContainer,
    ) -> Vec<AttributeInfo> {
        let attributes_count = to_u16(reader.read_n_bytes(2));
        let mut attributes = vec![];

        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(reader, constant_pool));
        }

        attributes
    }
}
