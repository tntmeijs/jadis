# Jadis
Jadis is a simple **Ja**va bytecode **Dis**assembler written in Rust. The goal of this project is to recreate some of the functionality of the `javap` command.

Please note that Jadis is an educational project. This means that for any serious work, you should consider using the Java disassembler that came with your JDK.

This specific disassembler has been written using the Java Virtual Machine Specification for Java SE 17 edition.
You can find the full specification file here: [https://docs.oracle.com/javase/specs/jvms/se17/html/index.html](https://docs.oracle.com/javase/specs/jvms/se17/html/index.html)

## Goals
- [ ] Learn how Java's bytecode is structured.
- [ ] Parse JVM bytecode.
- [ ] Display information about bytecode.
- [ ] Recreate `javap` commands.
  - [ ] `-h`
  - [ ] `-v`
  - [ ] `-l`
  - [ ] `-public`
  - [ ] `-protected`
  - [ ] `-package`
  - [ ] `-p`
  - [ ] `-c`
  - [ ] `-s`
  - [ ] `-sysinfo`
  - [ ] `-constants`
  - [ ] `-m`
  - [ ] `-J`
  - [ ] `--module-path`
  - [ ] `--system`
  - [ ] `--class-path`
  - [ ] `-classpath`
  - [ ] `-cp`
  - [ ] `-bootclasspath`
  - [ ] `--multi-release`

## Usage rights
Since this project is purely an educational project, I have decided to open-source everything and release it under the MIT license.

All I am asking you is to give me some credit if you end up using (parts of) this code in your own projects.

Happy hacking! 😊
