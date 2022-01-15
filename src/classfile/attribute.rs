//! Provides functionality to simplify working with attribute structures
//!
//! Reference: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7

use std::any::Any;

use crate::{
    byte_reader::ByteReader,
    utils::{to_u16, to_u32},
};
use crate::flags::{Flags, MethodParameterAccessFlags, ModuleExportsFlags, ModuleFlags, ModuleOpensFlags, ModuleRequiresFlags, NestedClassAccessFlags};

use super::ConstantPoolContainer;

/// Base trait to store specialised attributes
trait Attribute {
    /// Cast to the concreate type that implements this trait
    fn as_concrete_type(&self) -> &dyn Any;
}

/// Attribute types
// TODO: remove debug directive
#[derive(Debug)]
pub enum AttributeType {
    /// See [§4.7.2](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.2)
    ConstantValue,

    /// See [§4.7.3](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3)
    Code,

    /// See [§4.7.4](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.4)
    StackMapTable,

    /// See [§4.7.5](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.5)
    Exceptions,

    /// See [§4.7.6](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.6)
    InnerClasses,

    /// See [§4.7.7](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.7)
    EnclosingMethod,

    /// See [§4.7.8](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.8)
    Synthetic,

    /// See [§4.7.9](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.9)
    Signature,

    /// See [§4.7.10](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.10)
    SourceFile,

    /// See [§4.7.11](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.11)
    SourceDebugExtension,

    /// See [§4.7.12](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.12)
    LineNumberTable,

    /// See [§4.7.13](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.13)
    LocalVariableTable,

    /// See [§4.7.14](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.14)
    LocalVariableTypeTable,

    /// See [§4.7.15](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.15)
    Deprecated,

    /// See [§4.7.16](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.16)
    RuntimeVisibleAnnotations,

    /// See [§4.7.17](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.17)
    RuntimeInvisibleAnnotations,

    /// See [§4.7.18](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.18)
    RuntimeVisibleParameterAnnotations,

    /// See [§4.7.19](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.19)
    RuntimeInvisibleParameterAnnotations,

    /// See [§4.7.20](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.20)
    RuntimeVisibleTypeAnnotations,

    /// See [§4.7.21](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.21)
    RuntimeInvisibleTypeAnnotations,

    /// See [§4.7.22](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.22)
    AnnotationDefault,

    /// See [§4.7.23](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.23)
    BootstrapMethods,

    /// See [§4.7.24](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.24)
    MethodParameters,

    /// See [§4.7.25](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.25)
    Module,

    /// See [§4.7.26](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.26)
    ModulePackages,

    /// See [§4.7.27](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.27)
    ModuleMainClass,

    /// See [§4.7.28](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.28)
    NestHost,

    /// See [§4.7.29](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.29)
    NestMembers,

    /// See [§4.7.30](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.30)
    Record,

    /// See [§4.7.31](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.31)
    PermittedSubclasses,
}

/// Represents an attribute
pub struct AttributeInfo {
    /// Identifies the type of attribute this structure represents
    pub attribute_type: AttributeType,

    /// Data associated with this attribute
    data: Box<dyn Attribute>,
}

impl AttributeInfo {
    /// Create a new attribute from a class file binary blob
    pub fn new(reader: &mut ByteReader, constant_pool: &ConstantPoolContainer) -> Self {
        let attribute_name_index = to_u16(&reader.read_n_bytes(2));
        let attribute_length = to_u32(&reader.read_n_bytes(4));
        let name = constant_pool
            .get(&attribute_name_index)
            .expect(&format!(
                "Unable to read the attribute's name from the constant pool at index {}",
                attribute_name_index,
            ))
            .try_cast_into_utf8()
            .expect("Attribute's name index does not refer to a valid UTF-8 constant pool entry")
            .string
            .as_str();

        // Using the constant pool's UTF-8 string, match against all known attribute types
        match name {
            "ConstantValue" => {
                let attribute_type = AttributeType::ConstantValue;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_constant_value(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "Code" => {
                let attribute_type = AttributeType::Code;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_code(
                        reader,
                        attribute_name_index,
                        attribute_length,
                        constant_pool,
                    )),
                }
            }
            "StackMapTable" => {
                let attribute_type = AttributeType::StackMapTable;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_stack_map_table(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "Exceptions" => {
                let attribute_type = AttributeType::Exceptions;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_exceptions(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "InnerClasses" => {
                let attribute_type = AttributeType::InnerClasses;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_inner_classes(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "EnclosingMethod" => {
                let attribute_type = AttributeType::EnclosingMethod;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_enclosing_method(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "Synthetic" => {
                let attribute_type = AttributeType::Synthetic;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_synthetic(
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "Signature" => {
                let attribute_type = AttributeType::Signature;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_signature(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "SourceFile" => {
                let attribute_type = AttributeType::SourceFile;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_source_file(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "SourceDebugExtension" => {
                let attribute_type = AttributeType::SourceDebugExtension;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_source_debug_extension(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "LineNumberTable" => {
                let attribute_type = AttributeType::LineNumberTable;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_line_number_table(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "LocalVariableTable" => {
                let attribute_type = AttributeType::LocalVariableTable;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_local_variable_table(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "LocalVariableTypeTable" => {
                let attribute_type = AttributeType::LocalVariableTypeTable;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_local_variable_type_table(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "Deprecated" => {
                let attribute_type = AttributeType::Deprecated;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_deprecated(
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "RuntimeVisibleAnnotations" => {
                let attribute_type = AttributeType::RuntimeVisibleAnnotations;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_runtime_visible_annotations(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "RuntimeInvisibleAnnotations" => {
                let attribute_type = AttributeType::RuntimeInvisibleAnnotations;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_runtime_invisible_annotations(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "RuntimeVisibleParameterAnnotations" => {
                let attribute_type = AttributeType::RuntimeVisibleParameterAnnotations;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_runtime_visible_parameter_annotations(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "RuntimeInvisibleParameterAnnotations" => {
                let attribute_type = AttributeType::RuntimeInvisibleParameterAnnotations;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_runtime_invisible_parameter_annotations(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "RuntimeVisibleTypeAnnotations" => {
                let attribute_type = AttributeType::RuntimeVisibleTypeAnnotations;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_runtime_visible_type_annotations(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "RuntimeInvisibleTypeAnnotations" => {
                let attribute_type = AttributeType::RuntimeInvisibleTypeAnnotations;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_runtime_invisible_type_annotations(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "AnnotationDefault" => {
                let attribute_type = AttributeType::AnnotationDefault;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_annotation_default(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "BootstrapMethods" => {
                let attribute_type = AttributeType::BootstrapMethods;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_bootstrap_methods(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "MethodParameters" => {
                let attribute_type = AttributeType::MethodParameters;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_method_parameters(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "Module" => {
                let attribute_type = AttributeType::Module;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_module(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "ModulePackages" => {
                let attribute_type = AttributeType::ModulePackages;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_module_packages(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "ModuleMainClass" => {
                let attribute_type = AttributeType::ModuleMainClass;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_module_main_class(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "NestHost" => {
                let attribute_type = AttributeType::NestHost;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_nest_host(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "NestMembers" => {
                let attribute_type = AttributeType::NestMembers;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_nest_members(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            "Record" => {
                let attribute_type = AttributeType::Record;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_record(
                        reader,
                        attribute_name_index,
                        attribute_length,
                        constant_pool,
                    )),
                }
            }
            "PermittedSubclasses" => {
                let attribute_type = AttributeType::PermittedSubclasses;
                Self {
                    attribute_type,
                    data: Box::new(Self::read_data_as_permitted_subclasses(
                        reader,
                        attribute_name_index,
                        attribute_length,
                    )),
                }
            }
            _ => panic!("Unknown attribute: \"{}\"", name),
        }
    }

    /// Read the data blob as a constant value attribute
    fn read_data_as_constant_value(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeConstantValue {
        assert_eq!(
            attribute_length, 2,
            "Constant value attributes should have a length of 2"
        );

        let constantvalue_index = to_u16(&reader.read_n_bytes(2));

        AttributeConstantValue {
            attribute_name_index,
            attribute_length,
            constantvalue_index,
        }
    }

    /// Read the data blob as a code attribute
    fn read_data_as_code(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
        constant_pool: &ConstantPoolContainer,
    ) -> AttributeCode {
        let max_stack = to_u16(&reader.read_n_bytes(2));
        let max_locals = to_u16(&reader.read_n_bytes(2));
        let code_length = to_u32(&reader.read_n_bytes(4));

        let code = reader.read_n_bytes(code_length as usize);
        let exception_table_length = to_u16(&reader.read_n_bytes(2));

        let mut exception_table = vec![];
        for _ in 0..exception_table_length {
            let start_pc = to_u16(&reader.read_n_bytes(2));
            let end_pc = to_u16(&reader.read_n_bytes(2));
            let handler_pc = to_u16(&reader.read_n_bytes(2));
            let catch_type = to_u16(&reader.read_n_bytes(2));

            exception_table.push(ExceptionTableEntry {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            });
        }

        let attributes_count = to_u16(&reader.read_n_bytes(2));

        let mut attributes = vec![];
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(reader, constant_pool));
        }

        AttributeCode {
            attribute_name_index,
            attribute_length,
            max_stack,
            max_locals,
            code: code.to_vec(),
            exception_table,
            attributes,
        }
    }

    /// Read the data blob as a stack map table attribute
    fn read_data_as_stack_map_table(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeStackMapTable {
        todo!();
        // TODO: implement attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.4
        // Simply skip this attribute's data
        reader.read_n_bytes(std::convert::TryInto::try_into(attribute_length as u32).unwrap());
        AttributeStackMapTable {}
    }

    /// Read the data blob as an exceptions attribute
    fn read_data_as_exceptions(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeExceptions {
        let number_of_exceptions = to_u16(&reader.read_n_bytes(2));

        let mut exception_index_table = vec![];
        for _ in 0..number_of_exceptions {
            exception_index_table.push(to_u16(&reader.read_n_bytes(2)));
        }

        AttributeExceptions {
            attribute_name_index,
            attribute_length,
            number_of_exceptions,
            exception_index_table,
        }
    }

    /// Read the data blob as an inner classes attribute
    fn read_data_as_inner_classes(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeInnerClasses {
        let number_of_classes = to_u16(&reader.read_n_bytes(2));
        let mut classes = vec![];

        for _ in 0..number_of_classes {
            let inner_class_info_index = to_u16(&reader.read_n_bytes(2));
            let outer_class_info_index = to_u16(&reader.read_n_bytes(2));
            let inner_name_index = to_u16(&reader.read_n_bytes(2));
            let inner_class_access_flags = NestedClassAccessFlags::from_u16(to_u16(&reader.read_n_bytes(2)));

            classes.push(InnerClassEntry {
                inner_class_info_index,
                outer_class_info_index,
                inner_name_index,
                inner_class_access_flags,
            })
        }

        AttributeInnerClasses {
            attribute_name_index,
            attribute_length,
            classes,
        }
    }

    /// Read the data blob as an enclosing method attribute
    fn read_data_as_enclosing_method(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeEnclosingMethod {
        let class_index = to_u16(&reader.read_n_bytes(2));
        let method_index = to_u16(&reader.read_n_bytes(2));

        AttributeEnclosingMethod {
            attribute_name_index,
            attribute_length,
            class_index,
            method_index,
        }
    }

    /// Read the data blob as a synthetic attribute
    fn read_data_as_synthetic(
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeSynthetic {
        AttributeSynthetic {
            attribute_name_index,
            attribute_length,
        }
    }

    /// Read the data blob as a signature attribute
    fn read_data_as_signature(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeSignature {
        let signature_index = to_u16(&reader.read_n_bytes(2));

        AttributeSignature {
            attribute_name_index,
            attribute_length,
            signature_index,
        }
    }

    /// Read the data blob as a source file attribute
    fn read_data_as_source_file(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeSourceFile {
        let sourcefile_index = to_u16(&reader.read_n_bytes(2));

        AttributeSourceFile {
            attribute_name_index,
            attribute_length,
            sourcefile_index,
        }
    }

    /// Read the data blob as a source debug extension attribute
    fn read_data_as_source_debug_extension(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeSourceDebugExtension {
        let debug_extension = reader.read_n_bytes(attribute_length as usize);

        AttributeSourceDebugExtension {
            attribute_name_index,
            attribute_length,
            debug_extension,
        }
    }

    /// Read the data blob as a line number table attribute
    fn read_data_as_line_number_table(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeLineNumberTable {
        let line_number_table_length = to_u16(&reader.read_n_bytes(2));

        let mut line_number_table = vec![];
        for _ in 0..line_number_table_length {
            let start_pc = to_u16(&reader.read_n_bytes(2));
            let line_number = to_u16(&reader.read_n_bytes(2));

            line_number_table.push(LineNumberTableEntry {
                start_pc,
                line_number,
            });
        }

        AttributeLineNumberTable {
            attribute_name_index,
            attribute_length,
            line_number_table,
        }
    }

    /// Read the data blob as a local variable table attribute
    fn read_data_as_local_variable_table(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeLocalVariableTable {
        let mut local_variable_table = vec![];
        let local_variable_table_length = to_u16(&reader.read_n_bytes(2));
        for _ in 0..local_variable_table_length {
            let start_pc = to_u16(&reader.read_n_bytes(2));
            let length = to_u16(&reader.read_n_bytes(2));
            let name_index = to_u16(&reader.read_n_bytes(2));
            let descriptor_index = to_u16(&reader.read_n_bytes(2));
            let index = to_u16(&reader.read_n_bytes(2));

            local_variable_table.push(LocalVariableTableEntry {
                start_pc,
                length,
                name_index,
                descriptor_index,
                index,
            });
        }

        AttributeLocalVariableTable {
            attribute_name_index,
            attribute_length,
            local_variable_table,
        }
    }

    /// Read the data blob as a local variable type table attribute
    fn read_data_as_local_variable_type_table(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeLocalVariableTypeTable {
        let mut local_variable_type_table = vec![];
        let local_variable_type_table_length = to_u16(&reader.read_n_bytes(2));
        for _ in 0..local_variable_type_table_length {
            let start_pc = to_u16(&reader.read_n_bytes(2));
            let length = to_u16(&reader.read_n_bytes(2));
            let name_index = to_u16(&reader.read_n_bytes(2));
            let signature_index = to_u16(&reader.read_n_bytes(2));
            let index = to_u16(&reader.read_n_bytes(2));

            local_variable_type_table.push(LocalVariableTypeTableEntry {
                start_pc,
                length,
                name_index,
                signature_index,
                index,
            });
        }

        AttributeLocalVariableTypeTable {
            attribute_name_index,
            attribute_length,
            local_variable_type_table,
        }
    }

    /// Read the data blob as a deprecated attribute
    fn read_data_as_deprecated(
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeDeprecated {
        AttributeDeprecated {
            attribute_name_index,
            attribute_length,
        }
    }

    /// Read the data blob as a runtime visible annotations attribute
    fn read_data_as_runtime_visible_annotations(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeRuntimeVisibleAnnotations {
        todo!();
        // TODO: implement attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.16
        // Simply skip this attribute's data
        reader.read_n_bytes(std::convert::TryInto::try_into(attribute_length as u32).unwrap());
        AttributeRuntimeVisibleAnnotations {}
    }

    /// Read the data blob as a runtime invisible annotations attribute
    fn read_data_as_runtime_invisible_annotations(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeRuntimeInvisibleAnnotations {
        todo!();
        // TODO: implement attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.17
        // Simply skip this attribute's data
        reader.read_n_bytes(std::convert::TryInto::try_into(attribute_length as u32).unwrap());
        AttributeRuntimeInvisibleAnnotations {}
    }

    /// Read the data blob as a runtime visible parameter annotations attribute
    fn read_data_as_runtime_visible_parameter_annotations(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeRuntimeVisibleParameterAnnotations {
        todo!();
        // TODO: implement attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.18
        // Simply skip this attribute's data
        reader.read_n_bytes(std::convert::TryInto::try_into(attribute_length as u32).unwrap());
        AttributeRuntimeVisibleParameterAnnotations {}
    }

    /// Read the data blob as a runtime invisible parameter annotations attribute
    fn read_data_as_runtime_invisible_parameter_annotations(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeRuntimeInvisibleParameterAnnotations {
        todo!();
        // TODO: implement attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.19
        // Simply skip this attribute's data
        reader.read_n_bytes(std::convert::TryInto::try_into(attribute_length as u32).unwrap());
        AttributeRuntimeInvisibleParameterAnnotations {}
    }

    /// Read the data blob as a runtime visible type annotations attribute
    fn read_data_as_runtime_visible_type_annotations(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeRuntimeVisibleTypeAnnotations {
        todo!();
        // TODO: implement attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.20
        // Simply skip this attribute's data
        reader.read_n_bytes(std::convert::TryInto::try_into(attribute_length as u32).unwrap());
        AttributeRuntimeVisibleTypeAnnotations {}
    }

    /// Read the data blob as a runtime invisible type annotations attribute
    fn read_data_as_runtime_invisible_type_annotations(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeRuntimeInvisibleTypeAnnotations {
        todo!();
        // TODO: implement attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.21
        // Simply skip this attribute's data
        reader.read_n_bytes(std::convert::TryInto::try_into(attribute_length as u32).unwrap());
        AttributeRuntimeInvisibleTypeAnnotations {}
    }

    /// Read the data blob as an annotation default attribute
    fn read_data_as_annotation_default(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeAnnotationDefault {
        todo!();
        // TODO: implement attribute: https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.22
        // Simply skip this attribute's data
        reader.read_n_bytes(std::convert::TryInto::try_into(attribute_length as u32).unwrap());
        AttributeAnnotationDefault {}
    }

    /// Read the data blob as a bootstrap methods attribute
    fn read_data_as_bootstrap_methods(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeBootstrapMethods {
        let num_bootstrap_methods = to_u16(&reader.read_n_bytes(2));

        let mut bootstrap_methods = vec![];
        for _ in 0..num_bootstrap_methods {
            let bootstrap_method_ref = to_u16(&reader.read_n_bytes(2));
            let num_bootstrap_arguments = to_u16(&reader.read_n_bytes(2));

            let mut bootstrap_arguments = vec![];
            for _ in 0..num_bootstrap_arguments {
                bootstrap_arguments.push(to_u16(&reader.read_n_bytes(2)));
            }

            bootstrap_methods.push(BootstrapMethodEntry { bootstrap_method_ref, bootstrap_arguments });
        }

        AttributeBootstrapMethods {
            attribute_name_index,
            attribute_length,
            bootstrap_methods,
        }
    }

    /// Read the data blob as a method parameters attribute
    fn read_data_as_method_parameters(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeMethodParameters {
        let parameters_count = to_u16(&reader.read_n_bytes(2));
        let mut parameters = vec![];

        for _ in 0..parameters_count {
            let name_index = to_u16(&reader.read_n_bytes(2));
            let access_flags = MethodParameterAccessFlags::from_u16(to_u16(&reader.read_n_bytes(2)));

            parameters.push(MethodParameterEntry { name_index, access_flags });
        }

        AttributeMethodParameters {
            attribute_name_index,
            attribute_length,
            parameters,
        }
    }

    /// Read the data blob as a module attribute
    fn read_data_as_module(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeModule {
        let module_name_index = to_u16(&reader.read_n_bytes(2));
        let module_flags = ModuleFlags::from_u16(to_u16(&reader.read_n_bytes(2)));
        let module_version_index = to_u16(&reader.read_n_bytes(2));

        let mut requires = vec![];
        let requires_count = to_u16(&reader.read_n_bytes(2));
        for _ in 0..requires_count {
            let requires_index = to_u16(&reader.read_n_bytes(2));
            let requires_flags = ModuleRequiresFlags::from_u16(to_u16(&reader.read_n_bytes(2)));
            let requires_version_index = to_u16(&reader.read_n_bytes(2));

            requires.push(ModuleRequiresEntry {
                requires_index,
                requires_flags,
                requires_version_index,
            });
        }

        let mut exports = vec![];
        let exports_count = to_u16(&reader.read_n_bytes(2));
        for _ in 0..exports_count {
            let exports_index = to_u16(&reader.read_n_bytes(2));
            let exports_flags = ModuleExportsFlags::from_u16(to_u16(&reader.read_n_bytes(2)));

            let mut exports_to_index = vec![];
            let exports_to_count = to_u16(&reader.read_n_bytes(2));
            for _ in 0..exports_to_count {
                exports_to_index.push(to_u16(&reader.read_n_bytes(2)));
            }

            exports.push(ModuleExportsEntry {
                exports_index,
                exports_flags,
                exports_to_index,
            });
        }

        let mut opens = vec![];
        let opens_count = to_u16(&reader.read_n_bytes(2));
        for _ in 0..opens_count {
            let opens_index = to_u16(&reader.read_n_bytes(2));
            let opens_flags = ModuleOpensFlags::from_u16(to_u16(&reader.read_n_bytes(2)));

            let mut opens_to_index = vec![];
            let opens_to_count = to_u16(&reader.read_n_bytes(2));
            for _ in 0..opens_to_count {
                opens_to_index.push(to_u16(&reader.read_n_bytes(2)));
            }

            opens.push(ModuleOpensEntry {
                opens_index,
                opens_flags,
                opens_to_index,
            });
        }

        let mut uses_index = vec![];
        let uses_count = to_u16(&reader.read_n_bytes(2));
        for _ in 0..uses_count {
            uses_index.push(to_u16(&reader.read_n_bytes(2)));
        }

        let mut provides = vec![];
        let mut provides_count = to_u16(&reader.read_n_bytes(2));
        for _ in 0..provides_count {
            let provides_index = to_u16(&reader.read_n_bytes(2));

            let mut provides_with_index = vec![];
            let mut provides_with_count = to_u16(&reader.read_n_bytes(2));
            for _ in 0..provides_with_count {
                provides_with_index.push(to_u16(&reader.read_n_bytes(2)));
            }

            provides.push(ModuleProvidesEntry {
                provides_index,
                provides_with_count,
                provides_with_index,
            });
        }

        AttributeModule {
            attribute_name_index,
            attribute_length,
            module_name_index,
            module_flags,
            module_version_index,
            requires,
            exports,
            opens,
            uses_index,
            provides,
        }
    }

    /// Read the data blob as a module packages attribute
    fn read_data_as_module_packages(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeModulePackages {
        let mut package_index = vec![];
        let package_count = to_u16(&reader.read_n_bytes(2));
        for _ in 0..package_count {
            package_index.push(to_u16(&reader.read_n_bytes(2)));
        }

        AttributeModulePackages {
            attribute_name_index,
            attribute_length,
            package_index,
        }
    }

    /// Read the data blob as a module main class attribute
    fn read_data_as_module_main_class(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeModuleMainClass {
        let main_class_index = to_u16(&reader.read_n_bytes(2));

        AttributeModuleMainClass {
            attribute_name_index,
            attribute_length,
            main_class_index,
        }
    }

    /// Read the data blob as a nest host attribute
    fn read_data_as_nest_host(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeNestHost {
        let host_class_index = to_u16(&reader.read_n_bytes(2));

        AttributeNestHost {
            attribute_name_index,
            attribute_length,
            host_class_index,
        }
    }

    /// Read the data blob as a nest members attribute
    fn read_data_as_nest_members(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributeNestMembers {
        let mut classes = vec![];
        let number_of_classes = to_u16(&reader.read_n_bytes(2));
        for _ in 0..number_of_classes {
            classes.push(to_u16(&reader.read_n_bytes(2)));
        }

        AttributeNestMembers {
            attribute_name_index,
            attribute_length,
            classes,
        }
    }

    /// Read the data blob as a record attribute
    fn read_data_as_record(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
        constant_pool: &ConstantPoolContainer,
    ) -> AttributeRecord {
        let mut components = vec![];
        let components_count = to_u16(&reader.read_n_bytes(2));
        for _ in 0..components_count {
            let name_index = to_u16(&reader.read_n_bytes(2));
            let descriptor_index = to_u16(&reader.read_n_bytes(2));

            let mut attributes = vec![];
            let attributes_count = to_u16(&reader.read_n_bytes(2));
            for _ in 0..attributes_count {
                attributes.push(AttributeInfo::new(reader, constant_pool));
            }

            components.push(RecordComponentInfo {
                name_index,
                descriptor_index,
                attributes,
            });
        }

        AttributeRecord {
            attribute_name_index,
            attribute_length,
            components,
        }
    }

    /// Read the data blob as a permitted subclasses attribute
    fn read_data_as_permitted_subclasses(
        reader: &mut ByteReader,
        attribute_name_index: u16,
        attribute_length: u32,
    ) -> AttributePermittedSubclasses {
        let mut classes = vec![];
        let number_of_classes = to_u16(&reader.read_n_bytes(2));
        for _ in 0..number_of_classes {
            classes.push(to_u16(&reader.read_n_bytes(2)));
        }

        AttributePermittedSubclasses {
            attribute_name_index,
            attribute_length,
            classes,
        }
    }
}

/// Represents the value of a constant expression
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.2
pub struct AttributeConstantValue {
    /// Index into the constant pool that stores the attribute's name
    attribute_name_index: u16,

    /// Length of the attribute (must equal two)
    attribute_length: u32,

    /// Index into the constant pool which gives the value represented by this attribute
    constantvalue_index: u16,
}

impl Attribute for AttributeConstantValue {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Describes an exception handler in the code array
struct ExceptionTableEntry {
    /// Start of the range in the code array at which the exception handler is active
    start_pc: u16,

    /// End of the range in the code array at which the exception handler is active
    end_pc: u16,

    /// Indicates the start of the exception handler
    handler_pc: u16,

    /// The entry in the constant pool at this index represents a class of exceptions that this exception handler is designated
    /// to catch
    catch_type: u16,
}

/// A code attribute contains the Java Virtual Machine instructions and auxilary information for a method, including an instance
/// initialization method and a class or interface initialization method
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.3
pub struct AttributeCode {
    /// Index into the constant pool that stores the attribute's name
    attribute_name_index: u16,

    /// Indicates the length of the attribute (excluding the initial six bytes)
    attribute_length: u32,

    /// Maximum depth of the operand stack of this method
    max_stack: u16,

    /// Maximum number of local variables in the local variable array allocated upon invocation of this method
    max_locals: u16,

    /// Java Virtual Machine code that implements this method
    code: Vec<u8>,

    /// Described exceptions handles in the code array
    exception_table: Vec<ExceptionTableEntry>,

    /// Attributes associated with this code attribute
    attributes: Vec<AttributeInfo>,
}

impl Attribute for AttributeCode {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

pub struct AttributeStackMapTable {}

impl Attribute for AttributeStackMapTable {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Exceptions attributes indicate which checked exceptions a method may throw
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.5
pub struct AttributeExceptions {
    attribute_name_index: u16,
    attribute_length: u32,
    number_of_exceptions: u16,
    exception_index_table: Vec<u16>,
}

impl Attribute for AttributeExceptions {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Represents a class entry in the inner classes attribute
struct InnerClassEntry {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: Vec<NestedClassAccessFlags>,
}

/// Used inside a class file structure to provide information about the class or interface
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.6
pub struct AttributeInnerClasses {
    attribute_name_index: u16,
    attribute_length: u32,
    classes: Vec<InnerClassEntry>,
}

impl Attribute for AttributeInnerClasses {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// A class must have an enclosing method attribute if and only if it represents a local class or an anonymous class
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.7
pub struct AttributeEnclosingMethod {
    attribute_name_index: u16,
    attribute_length: u32,
    class_index: u16,
    method_index: u16,
}

impl Attribute for AttributeEnclosingMethod {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Synthetic attributes represent class members that do not appear in the source code
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.8
pub struct AttributeSynthetic {
    attribute_name_index: u16,
    attribute_length: u32,
}

impl Attribute for AttributeSynthetic {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// A Signature attribute stores a signature for a class, interface, constructor, method, field, or record component
/// whose declaration in the Java programming language uses type variables or parameterized types
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.9
pub struct AttributeSignature {
    attribute_name_index: u16,
    attribute_length: u32,
    signature_index: u16,
}

impl Attribute for AttributeSignature {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Source file attributes represent the name of the source file from which this class file was compiled
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.10
pub struct AttributeSourceFile {
    attribute_name_index: u16,
    attribute_length: u32,
    sourcefile_index: u16,
}

impl Attribute for AttributeSourceFile {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Holds extended debugging information which has no semantic effect on the Java Virtual Machine
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.11
pub struct AttributeSourceDebugExtension {
    attribute_name_index: u16,
    attribute_length: u32,
    debug_extension: Vec<u8>,
}

impl Attribute for AttributeSourceDebugExtension {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Represents an entry in the line number table in a line number table attribute
struct LineNumberTableEntry {
    /// Indicates the index into the code array at which the code for a new line in the original source file begins
    start_pc: u16,

    /// Gives the corresponding line number in the original source file
    line_number: u16,
}

/// A line number table attribute may be used by debuggers to determine which part of the code array corresponds to a given
/// line number in the original source file
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.12
pub struct AttributeLineNumberTable {
    attribute_name_index: u16,
    attribute_length: u32,
    line_number_table: Vec<LineNumberTableEntry>,
}

impl Attribute for AttributeLineNumberTable {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Indicates a range of code array offsets within which a local variable has a value, and indicates
/// the index into the local variable array of the current frame at which that local variable can be
/// found
struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

/// May be used by debuggers to determine the value of a given local variable during the execution
/// of a method
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.13
pub struct AttributeLocalVariableTable {
    attribute_name_index: u16,
    attribute_length: u32,
    local_variable_table: Vec<LocalVariableTableEntry>,
}

impl Attribute for AttributeLocalVariableTable {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Indicates a range of code array offsets within which a local variable has a value, and indicates
/// the index into the local variable array of the current frame at which that local variable can be
/// found
struct LocalVariableTypeTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

/// May be used by debuggers to determine the value of a given local variable during the execution
/// of a method
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.14
pub struct AttributeLocalVariableTypeTable {
    attribute_name_index: u16,
    attribute_length: u32,
    local_variable_type_table: Vec<LocalVariableTypeTableEntry>,
}

impl Attribute for AttributeLocalVariableTypeTable {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// The deprecated attribute is used to indicate that the class, interface, method, or field has been superseded
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.15
pub struct AttributeDeprecated {
    attribute_name_index: u16,
    attribute_length: u32,
}

impl Attribute for AttributeDeprecated {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

pub struct AttributeRuntimeVisibleAnnotations {}

impl Attribute for AttributeRuntimeVisibleAnnotations {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

pub struct AttributeRuntimeInvisibleAnnotations {}

impl Attribute for AttributeRuntimeInvisibleAnnotations {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

pub struct AttributeRuntimeVisibleParameterAnnotations {}

impl Attribute for AttributeRuntimeVisibleParameterAnnotations {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

pub struct AttributeRuntimeInvisibleParameterAnnotations {}

impl Attribute for AttributeRuntimeInvisibleParameterAnnotations {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

pub struct AttributeRuntimeVisibleTypeAnnotations {}

impl Attribute for AttributeRuntimeVisibleTypeAnnotations {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

pub struct AttributeRuntimeInvisibleTypeAnnotations {}

impl Attribute for AttributeRuntimeInvisibleTypeAnnotations {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

pub struct AttributeAnnotationDefault {}

impl Attribute for AttributeAnnotationDefault {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Represents a bootstrap method information entry
struct BootstrapMethodEntry {
    /// Index into the constant pool pointing to a method handle information structure
    bootstrap_method_ref: u16,

    /// Indices into the constant pool that point to bootstrap method arguments
    bootstrap_arguments: Vec<u16>,
}

/// Records bootstrap methods used to produce dynamically-computed constants and dynamically-computed call sites
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.23
pub struct AttributeBootstrapMethods {
    attribute_name_index: u16,
    attribute_length: u32,
    bootstrap_methods: Vec<BootstrapMethodEntry>,
}

impl Attribute for AttributeBootstrapMethods {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Represents information about a method parameter
struct MethodParameterEntry {
    /// Index into the constant pool representing a valid unqualified name denoting a formal parameter
    name_index: u16,

    /// Parameter access flags
    access_flags: Vec<MethodParameterAccessFlags>,
}

/// Records information about the formal parameters of a method, such as their names
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.24
pub struct AttributeMethodParameters {
    attribute_name_index: u16,
    attribute_length: u32,
    parameters: Vec<MethodParameterEntry>,
}

impl Attribute for AttributeMethodParameters {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Specifies a dependence of the current module
struct ModuleRequiresEntry {
    requires_index: u16,
    requires_flags: Vec<ModuleRequiresFlags>,
    requires_version_index: u16,
}

/// Indicates the number of entries in the exports table
struct ModuleExportsEntry {
    exports_index: u16,
    exports_flags: Vec<ModuleExportsFlags>,
    exports_to_index: Vec<u16>,
}

/// Specifies a package opened by the current module, such that all types in the package, and all
/// their members, may be accessed from outside the current module via the reflection libraries of
/// the Java SE Platform, possibly from a limited set of "friend" modules.
struct ModuleOpensEntry {
    opens_index: u16,
    opens_flags: Vec<ModuleOpensFlags>,
    opens_to_index: Vec<u16>,
}

/// Represents a service implementation for a given service interface
struct ModuleProvidesEntry {
    provides_index: u16,
    provides_with_count: u16,
    provides_with_index: Vec<u16>,
}

/// The Module attribute indicates the modules required by a module; the packages exported and
/// opened by a module; and the services used and provided by a module
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.25
pub struct AttributeModule {
    attribute_name_index: u16,
    attribute_length: u32,
    module_name_index: u16,
    module_flags: Vec<ModuleFlags>,
    module_version_index: u16,
    requires: Vec<ModuleRequiresEntry>,
    exports: Vec<ModuleExportsEntry>,
    opens: Vec<ModuleOpensEntry>,
    uses_index: Vec<u16>,
    provides: Vec<ModuleProvidesEntry>,
}

impl Attribute for AttributeModule {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// The ModulePackages attribute indicates all the packages of a module that are exported or opened
/// by the Module attribute, as well as all the packages of the service implementations recorded in
/// the Module attribute
///
/// The ModulePackages attribute may also indicate packages in the module that are neither exported
/// nor opened nor contain service implementations
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.26
pub struct AttributeModulePackages {
    attribute_name_index: u16,
    attribute_length: u32,
    package_index: Vec<u16>,
}

impl Attribute for AttributeModulePackages {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// The ModuleMainClass attribute indicates the main class of a module
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.27
pub struct AttributeModuleMainClass {
    attribute_name_index: u16,
    attribute_length: u32,
    main_class_index: u16,
}

impl Attribute for AttributeModuleMainClass {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// The NestHost attribute records the nest host of the nest to which the current class or interface
/// claims to belong
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.28
pub struct AttributeNestHost {
    attribute_name_index: u16,
    attribute_length: u32,
    host_class_index: u16,
}

impl Attribute for AttributeNestHost {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// The NestMembers attribute records the classes and interfaces that are authorized to claim
/// membership in the nest hosted by the current class or interface
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.29
pub struct AttributeNestMembers {
    attribute_name_index: u16,
    attribute_length: u32,
    classes: Vec<u16>,
}

impl Attribute for AttributeNestMembers {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// Specifies a record component of the current class
struct RecordComponentInfo {
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

///  The Record attribute indicates that the current class is a record class, and stores information
/// about the record components of the record class
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.30
pub struct AttributeRecord {
    attribute_name_index: u16,
    attribute_length: u32,
    components: Vec<RecordComponentInfo>,
}

impl Attribute for AttributeRecord {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}

/// The PermittedSubclasses attribute records the classes and interfaces that are authorized to
/// directly extend or implement the current class or interface
///
/// https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-4.html#jvms-4.7.31
pub struct AttributePermittedSubclasses {
    attribute_name_index: u16,
    attribute_length: u32,
    classes: Vec<u16>,
}

impl Attribute for AttributePermittedSubclasses {
    fn as_concrete_type(&self) -> &dyn Any {
        self
    }
}
