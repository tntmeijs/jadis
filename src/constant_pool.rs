//! Represents an entity in the constant pool section of a class file
//!
//! This module contains all information necessary to parse constant pool entities from class files

use std::{any::Any, collections::BTreeMap, panic};

use crate::{
    byte_reader::ByteReader,
    utils::{to_f32, to_f64, to_i32, to_i64, to_u16},
};

/// Constant pool container type
pub type ConstantPoolContainer = BTreeMap<u16, ConstantPoolInfo>;

/// Base trait to store specialised constant pool data entries
trait ConstantPoolInfoData {
    /// Cast to the concreate type that implements this trait
    fn as_concrete_type(&self) -> &dyn Any;
}

/// Constant pool tags
// TODO: remove debug directive
#[derive(Debug)]
pub enum Tag {
    /// UTF-8 string
    ConstantUtf8,

    /// Integer value
    ConstantInteger,

    /// Floating-point value
    ConstantFloat,

    /// Long value
    ConstantLong,

    /// Double value
    ConstantDouble,

    /// Class structure
    ConstantClass,

    /// Reference to a UTF-8 string
    ConstantString,

    /// Reference to a field
    ConstantFieldRef,

    /// Reference to a method
    ConstantMethodRef,

    /// Reference to an interface method
    ConstantInterfaceMethodRef,

    /// Name and type information
    ConstantNameAndType,

    /// Method handle
    ConstantMethodHandle,

    /// Method type
    ConstantMethodType,

    /// Dynamically-computed entity
    ConstantDynamic,

    /// Dynamically-computed call site
    ConstantInvokeDynamic,

    /// Module
    ConstantModule,

    /// Package
    ConstantPackage,
}

impl Tag {
    /// Convert a "tag" (u8) into its matching enum type, panics if no matching value could be found
    fn from_tag(tag: &u8) -> Self {
        match tag {
            1 => Self::ConstantUtf8,
            3 => Self::ConstantInteger,
            4 => Self::ConstantFloat,
            5 => Self::ConstantLong,
            6 => Self::ConstantDouble,
            7 => Self::ConstantClass,
            8 => Self::ConstantString,
            9 => Self::ConstantFieldRef,
            10 => Self::ConstantMethodRef,
            11 => Self::ConstantInterfaceMethodRef,
            12 => Self::ConstantNameAndType,
            15 => Self::ConstantMethodHandle,
            16 => Self::ConstantMethodType,
            17 => Self::ConstantDynamic,
            18 => Self::ConstantInvokeDynamic,
            19 => Self::ConstantModule,
            20 => Self::ConstantPackage,
            _ => panic!("Unknown tag: {}", tag),
        }
    }
}

/// Bytecode behaviours for method handles
pub enum MethodHandleType {
    /// getfield C.f:T
    RefGetField,

    /// getstatic C.f:T
    RefGetStatic,

    /// putfield C.f:T
    RefPutField,

    /// putstatic C.f:T
    RefPutStatic,

    /// invokevirtual C.m:(A*)T
    RefInvokeVirtual,

    /// invokestatic C.m:(A*)T
    RefInvokeStatic,

    /// invokespecial C.m:(A*)T
    RefInvokeSpecial,

    /// new C; dup; invokespecial C.<init>:(A*)V
    RefNewInvokeSpecial,

    /// invokeinterface C.m:(A*)T
    RefInvokeInterface,
}

impl MethodHandleType {
    /// Convert a "kind" (u8) into its matching enum type, panics if no matching value could be found
    fn from_kind(kind: &u8) -> Self {
        match kind {
            1 => Self::RefGetField,
            2 => Self::RefGetStatic,
            3 => Self::RefPutField,
            4 => Self::RefPutStatic,
            5 => Self::RefInvokeVirtual,
            6 => Self::RefInvokeStatic,
            7 => Self::RefInvokeSpecial,
            8 => Self::RefNewInvokeSpecial,
            9 => Self::RefInvokeInterface,
            _ => panic!("Unknown method handle type: {}", kind),
        }
    }
}

/// Represents an entity in the constant pool
pub struct ConstantPoolInfo {
    /// Identifies the type of data this entity represents
    pub tag: Tag,

    /// Data associated with this entity
    data: Box<dyn ConstantPoolInfoData>,
}

impl ConstantPoolInfo {
    /// Create a new constant pool entity from a class file binary blob
    pub fn new(reader: &mut ByteReader, index: u16) -> Self {
        let tag = reader.read_n_bytes(1);

        match Tag::from_tag(&tag[0]) {
            Tag::ConstantUtf8 => Self {
                tag: Tag::ConstantUtf8,
                data: Box::new(Self::read_data_as_utf8(reader, index)),
            },
            Tag::ConstantInteger => Self {
                tag: Tag::ConstantInteger,
                data: Box::new(Self::read_data_as_integer(reader, index)),
            },
            Tag::ConstantFloat => Self {
                tag: Tag::ConstantFloat,
                data: Box::new(Self::read_data_as_float(reader, index)),
            },
            Tag::ConstantLong => Self {
                tag: Tag::ConstantLong,
                data: Box::new(Self::read_data_as_long(reader, index)),
            },
            Tag::ConstantDouble => Self {
                tag: Tag::ConstantDouble,
                data: Box::new(Self::read_data_as_double(reader, index)),
            },
            Tag::ConstantClass => Self {
                tag: Tag::ConstantClass,
                data: Box::new(Self::read_data_as_class(reader, index)),
            },
            Tag::ConstantString => Self {
                tag: Tag::ConstantString,
                data: Box::new(Self::read_data_as_string(reader, index)),
            },
            Tag::ConstantFieldRef => Self {
                tag: Tag::ConstantFieldRef,
                data: Box::new(Self::read_data_as_field_ref(reader, index)),
            },
            Tag::ConstantMethodRef => Self {
                tag: Tag::ConstantMethodRef,
                data: Box::new(Self::read_data_as_method_ref(reader, index)),
            },
            Tag::ConstantInterfaceMethodRef => Self {
                tag: Tag::ConstantInterfaceMethodRef,
                data: Box::new(Self::read_data_as_interface_method_ref(reader, index)),
            },
            Tag::ConstantNameAndType => Self {
                tag: Tag::ConstantNameAndType,
                data: Box::new(Self::read_data_as_name_and_type(reader, index)),
            },
            Tag::ConstantMethodHandle => Self {
                tag: Tag::ConstantMethodHandle,
                data: Box::new(Self::read_data_as_method_handle(reader, index)),
            },
            Tag::ConstantMethodType => Self {
                tag: Tag::ConstantMethodType,
                data: Box::new(Self::read_data_as_method_type(reader, index)),
            },
            Tag::ConstantDynamic => Self {
                tag: Tag::ConstantDynamic,
                data: Box::new(Self::read_data_as_dynamic(reader, index)),
            },
            Tag::ConstantInvokeDynamic => Self {
                tag: Tag::ConstantInvokeDynamic,
                data: Box::new(Self::read_data_as_invoke_dynamic(reader, index)),
            },
            Tag::ConstantModule => Self {
                tag: Tag::ConstantModule,
                data: Box::new(Self::read_data_as_module(reader, index)),
            },
            Tag::ConstantPackage => Self {
                tag: Tag::ConstantPackage,
                data: Box::new(Self::read_data_as_package(reader, index)),
            },
        }
    }

    /// Read the data blob as an UTF-8 constant pool entry
    fn read_data_as_utf8(reader: &mut ByteReader, constant_pool_index: u16) -> ConstantUtf8Info {
        let length = to_u16(reader.read_n_bytes(2));

        ConstantUtf8Info {
            constant_pool_index,
            length,
            string: String::from_utf8_lossy(&reader.read_n_bytes(usize::from(length))).to_string(),
        }
    }

    /// Read the data blob as an integer constant pool entry
    fn read_data_as_integer(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantIntegerInfo {
        ConstantIntegerInfo {
            constant_pool_index,
            value: to_i32(&reader.read_n_bytes(4)),
        }
    }

    /// Read the data blob as a float constant pool entry
    fn read_data_as_float(reader: &mut ByteReader, constant_pool_index: u16) -> ConstantFloatInfo {
        ConstantFloatInfo {
            constant_pool_index,
            value: to_f32(&reader.read_n_bytes(4)),
        }
    }

    /// Read the data blob as a long constant pool entry
    fn read_data_as_long(reader: &mut ByteReader, constant_pool_index: u16) -> ConstantLongInfo {
        ConstantLongInfo {
            constant_pool_index,
            value: to_i64(&reader.read_n_bytes(8)),
        }
    }

    /// Read the data blob as a double constant pool entry
    fn read_data_as_double(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantDoubleInfo {
        ConstantDoubleInfo {
            constant_pool_index,
            value: to_f64(&reader.read_n_bytes(8)),
        }
    }

    /// Read the data blob as a class constant pool entry
    fn read_data_as_class(reader: &mut ByteReader, constant_pool_index: u16) -> ConstantClassInfo {
        ConstantClassInfo {
            constant_pool_index,
            name_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a string constant pool entry
    fn read_data_as_string(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantStringInfo {
        ConstantStringInfo {
            constant_pool_index,
            string_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a field reference constant pool entry
    fn read_data_as_field_ref(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantFieldRefInfo {
        ConstantFieldRefInfo {
            constant_pool_index,
            class_index: to_u16(&reader.read_n_bytes(2)),
            name_and_type_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a method reference constant pool entry
    fn read_data_as_method_ref(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantMethodRefInfo {
        ConstantMethodRefInfo {
            constant_pool_index,
            class_index: to_u16(&reader.read_n_bytes(2)),
            name_and_type_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as an interface method reference constant pool entry
    fn read_data_as_interface_method_ref(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantInterfaceMethodRefInfo {
        ConstantInterfaceMethodRefInfo {
            constant_pool_index,
            class_index: to_u16(&reader.read_n_bytes(2)),
            name_and_type_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a name and type constant pool entry
    fn read_data_as_name_and_type(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantNameAndTypeInfo {
        ConstantNameAndTypeInfo {
            constant_pool_index,
            name_index: to_u16(&reader.read_n_bytes(2)),
            descriptor_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a method handle constant pool entry
    fn read_data_as_method_handle(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantMethodHandleInfo {
        ConstantMethodHandleInfo {
            constant_pool_index,
            reference_kind: MethodHandleType::from_kind(&reader.read_n_bytes(1)[0]),
            reference_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a method type constant pool entry
    fn read_data_as_method_type(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantMethodTypeInfo {
        ConstantMethodTypeInfo {
            constant_pool_index,
            descriptor_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a dynamic constant pool entry
    fn read_data_as_dynamic(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantDynamicInfo {
        ConstantDynamicInfo {
            constant_pool_index,
            bootstrap_method_attr_index: to_u16(&reader.read_n_bytes(2)),
            name_and_type_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as an invoke dynamic constant pool entry
    fn read_data_as_invoke_dynamic(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantInvokeDynamicInfo {
        ConstantInvokeDynamicInfo {
            constant_pool_index,
            bootstrap_method_attr_index: to_u16(&reader.read_n_bytes(2)),
            name_and_type_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a module constant pool entry
    fn read_data_as_module(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantModuleInfo {
        ConstantModuleInfo {
            constant_pool_index,
            name_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Read the data blob as a package constant pool entry
    fn read_data_as_package(
        reader: &mut ByteReader,
        constant_pool_index: u16,
    ) -> ConstantPackageInfo {
        ConstantPackageInfo {
            constant_pool_index,
            name_index: to_u16(&reader.read_n_bytes(2)),
        }
    }

    /// Cast to as an UTF-8 constant pool entry
    pub fn try_cast_into_utf8(&self) -> Option<&ConstantUtf8Info> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantUtf8Info>()
    }

    /// Cast to an integer constant pool entry
    pub fn try_cast_into_integer(&self) -> Option<&ConstantIntegerInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantIntegerInfo>()
    }

    /// Cast to a float constant pool entry
    pub fn try_cast_into_float(&self) -> Option<&ConstantFloatInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantFloatInfo>()
    }

    /// Cast to a long constant pool entry
    pub fn try_cast_into_long(&self) -> Option<&ConstantLongInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantLongInfo>()
    }

    /// Cast to a double constant pool entry
    pub fn try_cast_into_double(&self) -> Option<&ConstantDoubleInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantDoubleInfo>()
    }

    /// Cast to a class constant pool entry
    pub fn try_cast_into_class(&self) -> Option<&ConstantClassInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantClassInfo>()
    }

    /// Cast to a string constant pool entry
    pub fn try_cast_into_string(&self) -> Option<&ConstantStringInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantStringInfo>()
    }

    /// Cast to a field reference constant pool entry
    pub fn try_cast_into_field_ref(&self) -> Option<&ConstantFieldRefInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantFieldRefInfo>()
    }

    /// Cast to a method reference constant pool entry
    pub fn try_cast_into_method_ref(&self) -> Option<&ConstantMethodRefInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantMethodRefInfo>()
    }

    /// Cast to an interface method reference constant pool entry
    pub fn try_cast_into_interface_method_ref(&self) -> Option<&ConstantInterfaceMethodRefInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantInterfaceMethodRefInfo>()
    }

    /// Cast to a name and type constant pool entry
    pub fn try_cast_into_name_and_type(&self) -> Option<&ConstantNameAndTypeInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantNameAndTypeInfo>()
    }

    /// Cast to a method handle constant pool entry
    pub fn try_cast_into_method_handle(&self) -> Option<&ConstantMethodHandleInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantMethodHandleInfo>()
    }

    /// Cast to a method type constant pool entry
    pub fn try_cast_into_method_type(&self) -> Option<&ConstantMethodTypeInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantMethodTypeInfo>()
    }

    /// Cast to a dynamic constant pool entry
    pub fn try_cast_into_dynamic(&self) -> Option<&ConstantDynamicInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantDynamicInfo>()
    }

    /// Cast to an invoke dynamic constant pool entry
    pub fn try_cast_into_invoke_dynamic(&self) -> Option<&ConstantInvokeDynamicInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantInvokeDynamicInfo>()
    }

    /// Cast to a module constant pool entry
    pub fn try_cast_into_module(&self) -> Option<&ConstantModuleInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantModuleInfo>()
    }

    /// Cast to a package constant pool entry
    pub fn try_cast_into_package(&self) -> Option<&ConstantPackageInfo> {
        self.data
            .as_concrete_type()
            .downcast_ref::<ConstantPackageInfo>()
    }
}

/// Constant pool UTF-8 string
pub struct ConstantUtf8Info {
    pub constant_pool_index: u16,
    pub length: u16,
    pub string: String,
}

impl ConstantPoolInfoData for ConstantUtf8Info {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool integer
pub struct ConstantIntegerInfo {
    pub constant_pool_index: u16,
    pub value: i32,
}

impl ConstantPoolInfoData for ConstantIntegerInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool float
pub struct ConstantFloatInfo {
    pub constant_pool_index: u16,
    pub value: f32,
}

impl ConstantPoolInfoData for ConstantFloatInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool long
pub struct ConstantLongInfo {
    pub constant_pool_index: u16,
    pub value: i64,
}

impl ConstantPoolInfoData for ConstantLongInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool double
pub struct ConstantDoubleInfo {
    pub constant_pool_index: u16,
    pub value: f64,
}

impl ConstantPoolInfoData for ConstantDoubleInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool class
// TODO: remove debug directive
#[derive(Debug, Clone)]
pub struct ConstantClassInfo {
    pub constant_pool_index: u16,
    pub name_index: u16,
}

impl ConstantPoolInfoData for ConstantClassInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool string
pub struct ConstantStringInfo {
    pub constant_pool_index: u16,
    pub string_index: u16,
}

impl ConstantPoolInfoData for ConstantStringInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool field reference
pub struct ConstantFieldRefInfo {
    pub constant_pool_index: u16,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantPoolInfoData for ConstantFieldRefInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool method reference
pub struct ConstantMethodRefInfo {
    pub constant_pool_index: u16,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantPoolInfoData for ConstantMethodRefInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool interface method reference
pub struct ConstantInterfaceMethodRefInfo {
    pub constant_pool_index: u16,
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantPoolInfoData for ConstantInterfaceMethodRefInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool name and type
pub struct ConstantNameAndTypeInfo {
    pub constant_pool_index: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl ConstantPoolInfoData for ConstantNameAndTypeInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool method handle
pub struct ConstantMethodHandleInfo {
    pub constant_pool_index: u16,
    pub reference_kind: MethodHandleType,
    pub reference_index: u16,
}

impl ConstantPoolInfoData for ConstantMethodHandleInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool method type
pub struct ConstantMethodTypeInfo {
    pub constant_pool_index: u16,
    pub descriptor_index: u16,
}

impl ConstantPoolInfoData for ConstantMethodTypeInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool dynamic
pub struct ConstantDynamicInfo {
    pub constant_pool_index: u16,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantPoolInfoData for ConstantDynamicInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool invoke dynamic
pub struct ConstantInvokeDynamicInfo {
    pub constant_pool_index: u16,
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

impl ConstantPoolInfoData for ConstantInvokeDynamicInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool module
pub struct ConstantModuleInfo {
    pub constant_pool_index: u16,
    pub name_index: u16,
}

impl ConstantPoolInfoData for ConstantModuleInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Constant pool package
pub struct ConstantPackageInfo {
    pub constant_pool_index: u16,
    pub name_index: u16,
}

impl ConstantPoolInfoData for ConstantPackageInfo {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}
