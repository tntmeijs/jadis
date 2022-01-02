//! Represents the JVM .class format
//!
//! This module is used to add class format parsing functionality to Jadis
//! Do note that the actual file IO is not handled by this module

use crate::access_flags::AccessFlags;
use crate::byte_reader::ByteReader;
use crate::constant_pool::{ConstantClassInfo, ConstantPoolInfo, Tag};
use crate::utils::{to_u16, to_u32};

const MAGIC_NUMBER: u32 = 0xCAFEBABE;
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

    /// Constant pool
    pub constant_pool: Vec<ConstantPoolInfo>,

    /// Class access and property modifiers
    pub access_flags: Vec<AccessFlags>,

    /// Represents the class defined by this class file
    pub this_class: ConstantClassInfo,

    /// Represets the direct superclass of the class defined by this class file
    pub super_class: Option<ConstantClassInfo>,

    /// Represents all interfaces that are a direct superinterface of this class or interface type
    pub interfaces: Vec<ConstantClassInfo>,
}

impl ClassFile {
    /// Create a new class file structure from a class file binary blob
    pub fn new(reader: &mut ByteReader) -> Self {
        let magic = Self::read_magic_number(reader);
        let minor_version = Self::read_u16(reader);
        let major_version = Self::read_u16(reader);
        let constant_pool = Self::read_constant_pool(reader);
        let access_flags = Self::read_access_flags(reader);
        let this_class = Self::read_this_class(reader, &constant_pool);
        let super_class = Self::read_super_class(reader, &constant_pool);
        let interfaces = Self::read_interfaces(reader, &constant_pool);

        Self {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
        }
    }

    /// Read the magic number (always 0xCAFEBABE)
    fn read_magic_number(reader: &mut ByteReader) -> u32 {
        let magic_number = to_u32(reader.read_n_bytes(SIZE_BYTES_U32));

        assert_eq!(
            magic_number, MAGIC_NUMBER,
            "Invalid class file - magic number did not equal {}",
            MAGIC_NUMBER
        );

        magic_number
    }

    /// Read a number (u16) from a binary blob
    fn read_u16(reader: &mut ByteReader) -> u16 {
        to_u16(reader.read_n_bytes(SIZE_BYTES_U16))
    }

    /// Read the entire constant pool
    fn read_constant_pool(reader: &mut ByteReader) -> Vec<ConstantPoolInfo> {
        let constant_pool_count = to_u16(reader.read_n_bytes(2));
        let mut constant_pool = vec![];

        // Index into the constant pool
        // The constant pool starts indexing at one, which is why this index starts at one as well
        let mut index = 1;

        // Read the entire constant pool
        while index < constant_pool_count {
            let info = ConstantPoolInfo::new(reader, index);

            // Long and double "occupy" two indices
            // See: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.5
            index += match info.tag {
                Tag::ConstantLong | Tag::ConstantDouble => 2,
                _ => 1,
            };

            constant_pool.push(info);
        }

        constant_pool
    }

    /// Read the class access and property modifiers
    fn read_access_flags(reader: &mut ByteReader) -> Vec<AccessFlags> {
        let bitmask = to_u16(reader.read_n_bytes(2));
        AccessFlags::from_u16(bitmask)
    }

    /// Read information from the constant pool about the class represented by this class file
    fn read_this_class(
        reader: &mut ByteReader,
        constant_pool: &Vec<ConstantPoolInfo>,
    ) -> ConstantClassInfo {
        // Subtract 1 to account for the 1-based indexing of a constant pool
        let index = to_u16(reader.read_n_bytes(2)) - 1;

        match constant_pool[usize::from(index)].try_cast_into_class() {
            Some(class) => class.clone(),
            None => panic!("Unable to fetch \"this class\" information from constant pool"),
        }
    }

    /// Read information from the constant pool about the direct super class of the class represented by this class file
    fn read_super_class(
        reader: &mut ByteReader,
        constant_pool: &Vec<ConstantPoolInfo>,
    ) -> Option<ConstantClassInfo> {
        let index = to_u16(reader.read_n_bytes(2));

        if index == 0 {
            return None;
        }

        // Subtract 1 to account for the 1-based indexing of a constant pool
        match constant_pool[usize::from(index - 1)].try_cast_into_class() {
            Some(class) => Some(class.clone()),
            None => None,
        }
    }

    /// Read information about all direct superinterfaces of this class or interface type from the constant pool
    fn read_interfaces(
        reader: &mut ByteReader,
        constant_pool: &Vec<ConstantPoolInfo>,
    ) -> Vec<ConstantClassInfo> {
        let interfaces_count = to_u16(reader.read_n_bytes(2));
        let mut interfaces = vec![];

        for _ in 0..interfaces_count {
            let constant_pool_index = to_u16(reader.read_n_bytes(2));

            // Subtract 1 to account for the 1-based indexing of a constant pool
            match constant_pool[usize::from(constant_pool_index - 1)].try_cast_into_class() {
                Some(class) => interfaces.push(class.clone()),
                None => panic!("Unable to fetch a class entry from the constant pool, error at constant pool index {}", constant_pool_index)
            };
        }

        interfaces
    }
}
