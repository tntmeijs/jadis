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
    visibility: DisassemblerVisibility,
    show_line_numbers: bool,
    show_instructions: bool,
    show_type_signatures: bool,
    show_system_info: bool,
    show_final_constants: bool,
}

/// Java Virtual Machine disassembler
pub struct Disassembler<'a> {
    config: &'a DisassemberConfig,
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

        Self { config, class }
    }
}
