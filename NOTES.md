# Notes

## Introduction
This document is used to save any information I deem useful whilst implementing
the JVM in Rust. I don't recommend using this as your source of truth when it
comes to the JVM, but I do think my notes provide a useful addition to the
official documentation.

You can consider these notes a summary of the things I read in the official JVM
documentation.

The official Java 17 JVM documentation can be found here:
[JVM SE17](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-1.html).
If you wish to implement a JVM for older and / or newer Java versions, please
make sure to use the corresponding version of the JVM documentation.

## Class file format
A JVM doesn't know anything about the Java programming language. Instead, it
simply knows how to interpret a set of predefined instructions specified in the
`class` file format.

Usually a `class` file is stored in an actual file, but this is not necessary.
Some specific use cases may warrant a different approach.

This file contains:
- Instructions for the Java Virtual Machine.
- A symbol table.
- Other relevant information / metadata.

The only real requirement a JVM has is that it must be able to correctly
interpret the `class` file format. All other things are not necessarily
optional, but are left vague on purpose to allow JVM developers to implement
them however JVM developers see fit.

## Data types
Like Java, the JVM operates on two data types:
- Primitives.
- Reference types.

An object is either allocated as a dynamic class instance, or an array.
References to objects are of type `reference`. Values of type `reference` can
be compared to pointers to objects (see C or C++, for example). More than one
reference to an object may exist.

Objects are *always* operated on, passed, and tested value values of type
`reference`.

A JVM can assume that nearly all type checking is done at compile type by a
compiler, and does not have to be done by the JVM. The JVM distinguishes its
operand types using speciallized instructions. For example: `iadd` operates on
integers, `dadd` operates on doubles, and so on.

## Primitive types and values
### Primitive types
- Numeric type.
  - Integral type.
    - `byte`
      - Default value is zero.
      - 8-bit.
      - Signed
        [two's-complement](https://en.wikipedia.org/wiki/Two%27s_complement)
        integer.
    - `short`
      - Default value is zero.
      - 16-bit.
      - Signed
        [two's-complement](https://en.wikipedia.org/wiki/Two%27s_complement)
        integer.
    - `int`
      - Default value is zero.
      - 32-bit.
      - Signed
        [two's-complement](https://en.wikipedia.org/wiki/Two%27s_complement)
        integer.
    - `long`
      - Default value is zero.
      - 64-bit.
      - Signed
        [two's-complement](https://en.wikipedia.org/wiki/Two%27s_complement)
        integer.
    - `char`
      - 16-bit unsigned integer.
      - Represents [UTF-16](https://en.wikipedia.org/wiki/UTF-16)-encoded
        [Unicode](https://en.wikipedia.org/wiki/Unicode) values.
      - Default value is `\u0000`, also known as `NULL`.
  - Floating-point type.
    - `float`
      - Default value is positive zero.
      - 32-bit IEEE 754 binary32 format
    - `double`
      - Default value is positive zero.
      - 64-bit IEEE 754 binary64 format
- Boolean type.
  - `true`
  - `false`
  - Default value is `false`.
- Return address type.
  - `returnAddress`
  - Pointer to an [opcode](https://nl.wikipedia.org/wiki/Opcode).
  - Of all primitive types, `returnAddress` is not directly associated with a
    Java programming language type.
  - Cannot be modified by the running program.

### Integral values
- `byte`: from -128 to +127, inclusive.
- `short`: from -32768 to +32767, inclusive.
- `int`: from -2147483648 to +2147483647, inclusive
- `long`: from -9223372036854775808 to 9223372036854775807, inclusive
- `char`: from 0 to 65535, inclusive

### Reference types
- `class`
- `array`
  - Items in the array are of type `element`.
    - Of type:
      - `primitive`
      - `class`
      - `interface`
- `interface`

A reference type may always be the special `null` reference type. This
reference has no run-time type and can be cast to *any* type. The default value
of a reference type is `null`.

The specification does not mandate a concrete value encoding `null`.

## Stack
Each virtual machine thread has a private stack which is created at the same
time as the thread. Stack memory is never manipulated directly, which is why
frames on the stack may be heap allocated. The memory of a stack does not need
to be continuous.

The JVM designer is free to set an initial stack size, as well as its minimum
and maximum size.

### Exceptions
- `StackOverflowError`: thrown when a computation requires a stack larger than
  the maximum size permitted by the JVM.
- `OutOfMemoryError`: **if a stack can be dynamically expanded**, and expansion
  is attempted but fails due to insufficient memory, or insufficient memory
  is available to create the initial stack for a new thread, this exception
  will be thrown.

## Heap
The heap is shared across *all* JVM threads. The heap is the run-time area from
which memory for all class instances and arrays is allocated.

The heap is created upon JVM start-up. Objects are never explicitly deallocated
from the heap. This is the responsibility of the garbage collector.

Heap memory may be fixed or dynamically expanded and contracted. In addition to
this, the heap memory itself does not need to be a continuous block of memory.

It is up to the JVM developer to determine the best way to manage heap memory.
Also, a JVM is allowed to give end users control over the initial heap size, if
the heap can be dynamically expanded and contracted, and the minimum and
maximum heap size.

### Exceptions
- `OutOfMemoryError`: thrown when a computation requires more heap memory than
  can be made available.

## Method area
- Shared among all JVM threads.
- Stores per-class structures such as:
  - Run-time constant pool.
  - Field and method data.
  - Code for methods and constructors, including special methods used in class,
    interface, and instance initialization.
- Created on start-up of the virtual machine.

Even though the method area is logically part of the [heap](#heap), simple
implementation may choose not to compress, nor garbage collect this bit of
memory at all.

The method area's memory does not need to be continuous and can either be fixed
in size, or dynamically sized depending on the JVM's implementation.

A JVM may provide its end user with control over the sizing behaviour of the
method area.

### Exceptions
- `OutOfMemoryError`: thrown if memory in the method area cannot be made
  available to satisfy an allocation request.

## Run-time constant pool
Per-class or per-interface run-time representation of the `constant_pool` table
in a `class` file.

Serves as a [symbol table](https://en.wikipedia.org/wiki/Symbol_table), but
contains a wider range of data than a typical symbol table in conventional
programming languages.

It contains various types of constants, ranging from numeric literals known at
compile-time to method and field references that must be resolved at run-time.

Each run-time constant pool is allocated from the [method area](#method-area)
and is constructed when a class or interface is created.

Note that the `long` and `double` entries take up 8 bytes in total. This means that a high byte and a low byte need to be
combined using big endian. Also, since these entries take up a lot of space, they'll also occupy two indices in the constant
pool.

### Exceptions
- `OutOfMemoryError`: thrown if the construction of the run-time constant pool
  requires more memory than can be made available in the method area.

## Native method stack
Used to support methods written in a language other than Java. For instance,
[C](https://en.wikipedia.org/wiki/C_(programming_language)). Also known as
`native` methods in Java.

Typically allocated upon thread creation, and are constructed when the thread
is created.

### Exceptions
- `StackOverflowError`: thrown if a computation requires a larger native method
  stack than permitted by the JVM.
- `OutOfMemoryError`: thrown if the initial native memory stack of a thread
  cannot be created due to insufficient memory, or if there is not enough
  memory available to dynamically resize a native memory stack.

## Frames
A frame is used to store data, partial results, method return values, as well
as to perform dynamic linking and to dispatch exceptions.

A new frame is created every time a method is invoked. A frame is destroyed
once the method invocation completes (with or without exceptions).

Frames are allocated from the JVM's [stack](#stack) of the thread that creates
the frame. Each frame has its own array of local variables, operand stack, and
a reference to the [run-time constant pool](#run-time-constant-pool) of the
class of the current method.

A frame may be extended with additional information, such as debugging
information.

Only one frame per thread is active. This is the frame for the currently
executing method. This frame is called the *current frame*, and the method is
called the *current method*. The class in which the current method is defined,
is called the *current class*. Operations on variables and the operand stack
usually refer to the current frame.

On method return, the current frame passes back its information to the previous
frame (if any). The current frame is discarded and the previous frame (if any)
becomes the new current frame.

A frame is created and owned by a thread. A thread cannot access a frame that belongs to a different thread.
