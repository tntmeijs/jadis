//! Java Virtual Machine language disassembler
//!
//! Provides functionality to disassemble .class files.
//! The goal of this module is to provide a similar functionality as a JDK's [`javap`](https://docs.oracle.com/javase/7/docs/technotes/tools/windows/javap.html) command.
//! Obviously it is not a direct replacement as this module has been written for educational purposes.
//! However, the disassembler should function well enough that it can theoretically be used as a drop-in replacement for [`javap`](https://docs.oracle.com/javase/7/docs/technotes/tools/windows/javap.html).

use crate::{byte_reader::ByteReader, class_file::ClassFile};

/// Controls which access level shows up in the output
pub enum DisassemblerVisibility {
    /// Show only public classes and members
    PUBLIC,

    /// Show protected/public classes and members
    PROTECTED,

    /// Show package/protected/public classes and members (default)
    PACKAGE,

    /// Show all classes and members
    PRIVATE,
}

/// Data needed to create a disassembler
pub struct DisassemberConfig {
    /// Class and member visibility setting
    visibility: DisassemblerVisibility,

    /// Indicates whether line numbers should be shown
    show_line_numbers: bool,

    /// Indicates whether assembly instructions should be shown
    show_instructions: bool,

    /// Indicates whether type signatures should be shown
    show_type_signatures: bool,

    /// Indicates whether system information should be shown
    show_system_info: bool,

    /// Indicates whether final constants should be shown
    show_final_constants: bool,
}

/// Java Virtual Machine disassembler
pub struct Disassembler<'a> {
    /// Used to customize the disassembler's behaviour
    config: &'a DisassemberConfig,

    /// Disassembled class file information
    class: ClassFile,
}

impl DisassemberConfig {
    /// Create a new disassember instance
    pub fn new() -> Self {
        Self {
            visibility: DisassemblerVisibility::PACKAGE,
            show_line_numbers: false,
            show_instructions: false,
            show_type_signatures: false,
            show_system_info: false,
            show_final_constants: false,
        }
    }

    /// Filter which visibility level should show up in the output
    pub fn with_visibility(&mut self, visibility: DisassemblerVisibility) {
        self.visibility = visibility;
    }

    /// Show line numbers
    pub fn show_line_numbers(&mut self) {
        self.show_line_numbers = true;
    }

    /// Show assembly instructions
    pub fn show_assembly_instructions(&mut self) {
        self.show_instructions = true;
    }

    /// Show type signatures
    pub fn show_type_signatures(&mut self) {
        self.show_type_signatures = true;
    }

    /// Show system information
    pub fn show_system_info(&mut self) {
        self.show_system_info = true;
    }

    /// Show final constants
    pub fn show_final_constants(&mut self) {
        self.show_final_constants = true;
    }
}

impl<'a> Disassembler<'a> {
    pub fn new(config: &'a DisassemberConfig, reader: &mut ByteReader) -> Self {
        let class = ClassFile::new(reader);

        // TODO: remove
        // DEBUG OUTPUT
        println!("Magic number: {:#08x}", class.magic);
        println!("Version: {}.{}", class.major_version, class.minor_version);
        println!("Constant pool count: {}", class.constant_pool_count);
        println!("Constant pool contents:");

        for entry in &class.constant_pool {
            match entry.tag {
                crate::constant_pool::Tag::ConstantUtf8 => {
                    let concrete = entry.try_cast_into_utf8();
                    println!("#{} = Utf8", entry.index);
                }
                crate::constant_pool::Tag::ConstantInteger => {
                    let concrete = entry.try_cast_into_integer();
                    println!("#{} = Integer", entry.index);
                }
                crate::constant_pool::Tag::ConstantFloat => {
                    let concrete = entry.try_cast_into_float();
                    println!("#{} = Float", entry.index);
                }
                crate::constant_pool::Tag::ConstantLong => {
                    let concrete = entry.try_cast_into_long();
                    println!("#{} = Long", entry.index);
                }
                crate::constant_pool::Tag::ConstantDouble => {
                    let concrete = entry.try_cast_into_double();
                    println!("#{} = Double", entry.index);
                }
                crate::constant_pool::Tag::ConstantClass => {
                    let concrete = entry.try_cast_into_class();
                    println!("#{} = Class", entry.index);
                }
                crate::constant_pool::Tag::ConstantString => {
                    let concrete = entry.try_cast_into_string();
                    println!("#{} = String", entry.index);
                }
                crate::constant_pool::Tag::ConstantFieldRef => {
                    let concrete = entry.try_cast_into_field_ref();
                    println!("#{} = FieldRef", entry.index);
                }
                crate::constant_pool::Tag::ConstantMethodRef => {
                    let concrete = entry.try_cast_into_method_ref();
                    println!("#{} = MethodRef", entry.index);
                }
                crate::constant_pool::Tag::ConstantInterfaceMethodRef => {
                    let concrete = entry.try_cast_into_interface_method_ref();
                    println!("#{} = InterfaceMethodRef", entry.index);
                }
                crate::constant_pool::Tag::ConstantNameAndType => {
                    let concrete = entry.try_cast_into_name_and_type();
                    println!("#{} = ConstantNameAndType", entry.index);
                }
                crate::constant_pool::Tag::ConstantMethodHandle => {
                    let concrete = entry.try_cast_into_method_handle();
                    println!("#{} = MethodHandle", entry.index);
                }
                crate::constant_pool::Tag::ConstantMethodType => {
                    let concrete = entry.try_cast_into_method_type();
                    println!("#{} = MethodType", entry.index);
                }
                crate::constant_pool::Tag::ConstantDynamic => {
                    let concrete = entry.try_cast_into_dynamic();
                    println!("#{} = Dynamic", entry.index);
                }
                crate::constant_pool::Tag::ConstantInvokeDynamic => {
                    let concrete = entry.try_cast_into_invoke_dynamic();
                    println!("#{} = InvokeDynamic", entry.index);
                }
                crate::constant_pool::Tag::ConstantModule => {
                    let concrete = entry.try_cast_into_module();
                    println!("#{} = Module", entry.index);
                }
                crate::constant_pool::Tag::ConstantPackage => {
                    let concrete = entry.try_cast_into_package();
                    println!("#{} = Package", entry.index);
                }
            }
        }

        Self { config, class }
    }
}
