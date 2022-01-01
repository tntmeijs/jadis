//! # Introduction
//! The Jadis crate is a Java Virtual Machine language disassembler written in Rust.
//! This project has been written for educational purposes, however, Jadis can theoretically be used as a drop-in replacement of [`javap`](https://docs.oracle.com/javase/7/docs/technotes/tools/windows/javap.html).
//!
//! # Using Jadis
//! It is fairly straightforward to use Jadis. Simply run the project and use the same command-line arguments you would use with [`javap`](https://docs.oracle.com/javase/7/docs/technotes/tools/windows/javap.html).
//! Please do note that the output format could be slightly different than the output you are used to.
//! Having said that, though, the data in the output should be identical to the output of the disassembler that came with your JDK.
//!
//! | option | description |
//! | --- | --- |
//! | --bootclasspath | Override location of bootstrap class files |
//! | --class-path | Specify where to find user class files |
//! | --classpath | Specify where to find user class files |
//! | -c | Disassemble the code |
//! | --constants | Show final constants |
//! | --cp | Specify where to find user class files |
//! | -h, --help | Print this help message |
//! | -J | Specify a VM option |
//! | -l | Print line number and local variable tables |
//! | -m, --module | Specify module containing classes to be disassembled |
//! | --module-path | Specify where to find application modules |
//! | --multi-release | Specify the version to use in multi-release JAR files |
//! | --package | Show package/protected/public classes and members (default) |
//! | -p, --private | Show all classes and members |
//! | --protected | Show protected/public classes and members |
//! | --public | Show only public classes and members |
//! | -s | Print internal type signatures |
//! | --sysinfo | Show system info (path, size, date, SHA-256 hash) of class being processed |
//! | --system | Specify where to find system modules |
//! | -V, --version | Version information |
//! | -v, --verbose | Print additional information |

mod access_flags;
mod byte_reader;
mod class_file;
mod constant_pool;
mod disassembler;
mod utils;

use byte_reader::ByteReader;
use clap::{App, AppSettings, Arg};
use disassembler::{DisassemberConfig, Disassembler, DisassemblerVisibility};

/// Application entry point
fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .help_message("Print this help message")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .setting(AppSettings::AllowExternalSubcommands)
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Print additional information"),
        )
        .arg(
            Arg::with_name("version")
                .long("version")
                .help("Version information"),
        )
        .arg(
            Arg::with_name("line")
                .short("l")
                .help("Print line number and local variable tables"),
        )
        .arg(
            Arg::with_name("public")
                .long("public")
                .help("Show only public classes and members"),
        )
        .arg(
            Arg::with_name("protected")
                .long("protected")
                .help("Show protected/public classes and members"),
        )
        .arg(
            Arg::with_name("package")
                .long("package")
                .help("Show package/protected/public classes and members (default)"),
        )
        .arg(
            Arg::with_name("private")
                .short("p")
                .long("private")
                .help("Show all classes and members"),
        )
        .arg(
            Arg::with_name("code")
                .short("c")
                .help("Disassemble the code"),
        )
        .arg(
            Arg::with_name("signatures")
                .short("s")
                .help("Print internal type signatures"),
        )
        .arg(
            Arg::with_name("sysinfo")
                .long("sysinfo")
                .help("Show system info (path, size, date, SHA-256 hash) of class being processed"),
        )
        .arg(
            Arg::with_name("constants")
                .long("constants")
                .help("Show final constants"),
        )
        .arg(
            Arg::with_name("module")
                .short("m")
                .long("module")
                .help("Specify module containing classes to be disassembled"),
        )
        .arg(Arg::with_name("jvm").short("J").help("Specify a VM option"))
        .arg(
            Arg::with_name("module-path")
                .long("module-path")
                .help("Specify where to find application modules"),
        )
        .arg(
            Arg::with_name("system")
                .long("system")
                .help("Specify where to find system modules"),
        )
        .arg(
            Arg::with_name("class-path")
                .long("class-path")
                .help("Specify where to find user class files"),
        )
        .arg(
            Arg::with_name("classpath")
                .long("classpath")
                .help("Specify where to find user class files"),
        )
        .arg(
            Arg::with_name("cp")
                .long("cp")
                .help("Specify where to find user class files"),
        )
        .arg(
            Arg::with_name("bootclasspath")
                .long("bootclasspath")
                .help("Override location of bootstrap class files"),
        )
        .arg(
            Arg::with_name("multi-release")
                .long("multi-release")
                .help("Specify the version to use in multi-release JAR files"),
        )
        .get_matches();

    let mut disassembler_config = DisassemberConfig::new();

    if matches.is_present("verbose") {
        //
    } else if matches.is_present("version") {
        //
    } else if matches.is_present("line") {
        disassembler_config.show_line_numbers();
    } else if matches.is_present("public") {
        disassembler_config.with_visibility(DisassemblerVisibility::PUBLIC);
    } else if matches.is_present("protected") {
        disassembler_config.with_visibility(DisassemblerVisibility::PROTECTED);
    } else if matches.is_present("package") {
        disassembler_config.with_visibility(DisassemblerVisibility::PACKAGE);
    } else if matches.is_present("private") {
        disassembler_config.with_visibility(DisassemblerVisibility::PRIVATE);
    } else if matches.is_present("code") {
        disassembler_config.show_assembly_instructions();
    } else if matches.is_present("signatures") {
        disassembler_config.show_type_signatures();
    } else if matches.is_present("sysinfo") {
        disassembler_config.show_system_info();
    } else if matches.is_present("constants") {
        disassembler_config.show_final_constants();
    } else if matches.is_present("module") {
        todo!();
    } else if matches.is_present("jvm") {
        todo!();
    } else if matches.is_present("module-path") {
        todo!();
    } else if matches.is_present("system") {
        todo!();
    } else if matches.is_present("class-path") {
        todo!();
    } else if matches.is_present("classpath") {
        todo!();
    } else if matches.is_present("cp") {
        todo!();
    } else if matches.is_present("bootclasspath") {
        todo!();
    } else if matches.is_present("multi-release") {
        todo!();
    }

    // The last argument should always be the class to disassemble
    if let Some(file_to_disassemble) = std::env::args().last().to_owned() {
        let mut file = ByteReader::new(&file_to_disassemble);
        Disassembler::new(&disassembler_config, &mut file);
    }
}
