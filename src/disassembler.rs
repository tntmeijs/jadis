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

        // TODO: remove debug printing

        println!("Magic number: {:#08x}", class.magic);
        println!("Version: {}.{}", class.major_version, class.minor_version);
        println!("This class: #{}", class.this_class.constant_pool_index);

        if class.super_class.is_some() {
            println!(
                "Super class: #{}",
                class.super_class.as_ref().unwrap().constant_pool_index
            );
        } else {
            println!("Super class: NONE");
        }

        println!("Constant pool count: {}", class.constant_pool_count);
        println!("Constant pool contents:");

        for entry in &class.constant_pool {
            match entry.tag {
                crate::constant_pool::Tag::ConstantUtf8 => {
                    let concrete = entry.try_cast_into_utf8().unwrap();
                    println!("#{} = Utf8", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantInteger => {
                    let concrete = entry.try_cast_into_integer().unwrap();
                    println!("#{} = Integer", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantFloat => {
                    let concrete = entry.try_cast_into_float().unwrap();
                    println!("#{} = Float", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantLong => {
                    let concrete = entry.try_cast_into_long().unwrap();
                    println!("#{} = Long", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantDouble => {
                    let concrete = entry.try_cast_into_double().unwrap();
                    println!("#{} = Double", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantClass => {
                    let concrete = entry.try_cast_into_class().unwrap();
                    println!("#{} = Class", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantString => {
                    let concrete = entry.try_cast_into_string().unwrap();
                    println!("#{} = String", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantFieldRef => {
                    let concrete = entry.try_cast_into_field_ref().unwrap();
                    println!("#{} = FieldRef", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantMethodRef => {
                    let concrete = entry.try_cast_into_method_ref().unwrap();
                    println!("#{} = MethodRef", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantInterfaceMethodRef => {
                    let concrete = entry.try_cast_into_interface_method_ref().unwrap();
                    println!("#{} = InterfaceMethodRef", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantNameAndType => {
                    let concrete = entry.try_cast_into_name_and_type().unwrap();
                    println!("#{} = ConstantNameAndType", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantMethodHandle => {
                    let concrete = entry.try_cast_into_method_handle().unwrap();
                    println!("#{} = MethodHandle", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantMethodType => {
                    let concrete = entry.try_cast_into_method_type().unwrap();
                    println!("#{} = MethodType", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantDynamic => {
                    let concrete = entry.try_cast_into_dynamic().unwrap();
                    println!("#{} = Dynamic", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantInvokeDynamic => {
                    let concrete = entry.try_cast_into_invoke_dynamic().unwrap();
                    println!("#{} = InvokeDynamic", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantModule => {
                    let concrete = entry.try_cast_into_module().unwrap();
                    println!("#{} = Module", concrete.constant_pool_index);
                }
                crate::constant_pool::Tag::ConstantPackage => {
                    let concrete = entry.try_cast_into_package().unwrap();
                    println!("#{} = Package", concrete.constant_pool_index);
                }
            }
        }

        println!("Access flags:");

        for flag in &class.access_flags {
            println!("\t- {:?}", flag);
        }

        Self { config, class }
    }
}
