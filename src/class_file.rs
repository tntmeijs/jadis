//! Represents the JVM .class format
//!
//! This module is used to add class format parsing functionality to Jadis
//! Do note that the actual file IO is not handled by this module

use crate::access_flags::ClassAccessFlags;
use crate::attribute::AttributeInfo;
use crate::byte_reader::ByteReader;
use crate::constant_pool::{ConstantClassInfo, ConstantPoolContainer, ConstantPoolInfo, Tag};
use crate::field::FieldInfo;
use crate::method::MethodInfo;
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
    pub constant_pool: ConstantPoolContainer,

    /// Class access and property modifiers
    pub access_flags: Vec<ClassAccessFlags>,

    /// Represents the class defined by this class file
    pub this_class: ConstantClassInfo,

    /// Represets the direct superclass of the class defined by this class file
    pub super_class: Option<ConstantClassInfo>,

    /// Represents all interfaces that are a direct superinterface of this class or interface type
    pub interfaces: Vec<ConstantClassInfo>,

    /// Represents all fields, both class variables and instance variables, declared by this class or interface type
    pub fields: Vec<FieldInfo>,

    /// Represents all methods
    pub methods: Vec<MethodInfo>,

    /// Represents all class attributes
    pub attributes: Vec<AttributeInfo>,
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
        let fields = Self::read_fields(reader, &constant_pool);
        let methods = Self::read_methods(reader, &constant_pool);
        let attributes = Self::read_attributes(reader, &constant_pool);

        Self {
            magic,
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
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
    fn read_constant_pool(reader: &mut ByteReader) -> ConstantPoolContainer {
        let constant_pool_count = to_u16(reader.read_n_bytes(2));
        let mut constant_pool = ConstantPoolContainer::new();

        // Index into the constant pool
        // The constant pool starts indexing at one, which is why this index starts at one as well
        let mut index = 1;

        // Read the entire constant pool
        while index < constant_pool_count {
            let info = ConstantPoolInfo::new(reader, index);

            // Long and double "occupy" two indices
            // See: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.4.5
            let offset = match info.tag {
                Tag::ConstantLong | Tag::ConstantDouble => 2,
                _ => 1,
            };

            // First store the new entry with the current index
            constant_pool.insert(index, info);

            // Once the entry has been stored, the index can safely be updated to the next index
            index += offset;
        }

        constant_pool
    }

    /// Read the class access and property modifiers
    fn read_access_flags(reader: &mut ByteReader) -> Vec<ClassAccessFlags> {
        let bitmask = to_u16(reader.read_n_bytes(2));
        ClassAccessFlags::from_u16(bitmask)
    }

    /// Read information from the constant pool about the class represented by this class file
    fn read_this_class(
        reader: &mut ByteReader,
        constant_pool: &ConstantPoolContainer,
    ) -> ConstantClassInfo {
        let constant_pool_index = to_u16(reader.read_n_bytes(2));

        let constant_pool_entry = constant_pool.get(&constant_pool_index).expect(&format!(
            "Unable to fetch entry from constant pool at index {}",
            constant_pool_index
        ));

        match constant_pool_entry.try_cast_into_class() {
            Some(class) => class.clone(),
            None => panic!(
                "Unable to fetch \"this class\" information from constant pool at index {}",
                constant_pool_index
            ),
        }
    }

    /// Read information from the constant pool about the direct super class of the class represented by this class file
    fn read_super_class(
        reader: &mut ByteReader,
        constant_pool: &ConstantPoolContainer,
    ) -> Option<ConstantClassInfo> {
        let constant_pool_index = to_u16(reader.read_n_bytes(2));

        if constant_pool_index == 0 {
            return None;
        }

        let constant_pool_entry = constant_pool.get(&constant_pool_index).expect(&format!(
            "Unable to fetch entry from constant pool at index {}",
            constant_pool_index
        ));

        match constant_pool_entry.try_cast_into_class() {
            Some(class) => Some(class.clone()),
            None => None,
        }
    }

    /// Read information about all direct superinterfaces of this class or interface type from the constant pool
    fn read_interfaces(
        reader: &mut ByteReader,
        constant_pool: &ConstantPoolContainer,
    ) -> Vec<ConstantClassInfo> {
        let interfaces_count = to_u16(reader.read_n_bytes(2));
        let mut interfaces = vec![];

        for _ in 0..interfaces_count {
            let constant_pool_index = to_u16(reader.read_n_bytes(2));

            let constant_pool_entry = constant_pool.get(&constant_pool_index).expect(&format!(
                "Unable to fetch entry from constant pool at index {}",
                constant_pool_index
            ));

            match constant_pool_entry.try_cast_into_class() {
                Some(class) => interfaces.push(class.clone()),
                None => panic!("Unable to fetch a class entry from the constant pool, error at constant pool index {}", constant_pool_index)
            };
        }

        interfaces
    }

    /// Read information about the fields in this class or interface represented by this class file
    fn read_fields(
        reader: &mut ByteReader,
        constant_pool: &ConstantPoolContainer,
    ) -> Vec<FieldInfo> {
        let fields_count = to_u16(reader.read_n_bytes(2));
        let mut fields = vec![];

        for _ in 0..fields_count {
            fields.push(FieldInfo::new(reader, constant_pool));
        }

        fields
    }

    /// Read information about the methods
    fn read_methods(
        reader: &mut ByteReader,
        constant_pool: &ConstantPoolContainer,
    ) -> Vec<MethodInfo> {
        let methods_count = to_u16(reader.read_n_bytes(2));
        let mut methods = vec![];

        for _ in 0..methods_count {
            methods.push(MethodInfo::new(reader, constant_pool));
        }

        methods
    }

    /// Read information about the class attributes
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
