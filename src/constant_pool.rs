//! Represents an entity in the constant pool section of a class file
//!
//! This module contains all information necessary to parse constant pool entities from class files

use crate::byte_reader::ByteReader;

/// JVM constant pool info tag
pub enum Tag {
    /// UTF-8 string
    Utf8,

    /// Integer value
    Integer,

    /// Floating-point value
    Float,

    /// Long value
    Long,

    /// Double value
    Double,

    /// Class structure
    Class,

    /// Reference to a UTF-8 string
    String,

    /// Reference to a field
    FieldRef,

    /// Reference to a method
    MethodRef,

    /// Reference to an interface method
    InterfaceMethodRef,

    /// Name and type information
    NameAndType,

    /// Method handle
    MethodHandle,

    /// Method type
    MethodType,

    /// Dynamically-computed entity
    Dynamic,

    /// Dynamically-computed call site
    InvokeDynamic,

    /// Module
    Module,

    /// Package
    Package,
}

/// Represents an entity in the constant pool
pub struct ConstantPoolInfo {
    tag: Tag,
    data: Vec<u8>,
}

impl ConstantPoolInfo {
    /// Create a new constant pool entity from a class file binary blob
    pub fn new(reader: &mut ByteReader) -> Self {
        Self {
            tag: Tag::Utf8,
            data: vec![],
        }
    }
}
