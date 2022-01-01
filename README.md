# Jadis
Jadis is a simple **Ja**va bytecode **Dis**assembler written in Rust.
The goal of this project is to provide an alternative to the `javap` command we all know and love.

Please note that Jadis is an educational project.
This means that for any serious work, you should consider using the Java disassembler that came with your JDK.

This specific disassembler has been written using the Java Virtual Machine Specification for Java SE 17 edition.
You can find the full specification file here:
[https://docs.oracle.com/javase/specs/jvms/se17/html/index.html](https://docs.oracle.com/javase/specs/jvms/se17/html/index.html)

All code in this project has been documented using Rust's automatically generated documentation.
Please go to [https://jadis.tahar.dev](https://jadis.tahar.dev) to view this project's documentation.

## Personal goals
- Learn how the JVM's bytecode is structured.
- Parse bytecode.
- Display assembly instructions.
- Learn Rust.
- Create a more user-friendly GUI to disassemble class files.
  
## Future improvements
- Use [https://github.com/gyscos/Cursive](Cursive) to create a nice terminal GUI.
- Add the ability to execute `javac` from within Jadis.
- Disassemble multiple files at once.
- View generated assembly code next to written Java code.

## Usage rights
Since this project is purely an educational project, I have decided to open-source everything and release it under the MIT license.

All I am asking you is to give me some credit if you end up using (parts of) this code in your own projects.

Happy hacking! 😊
