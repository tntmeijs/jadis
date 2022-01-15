//! Contains class access and property modifiers

use crate::utils::bitmask_matches;

/// Base trait for all access flag types
pub trait AccessFlags {
    type AccessFlagType;

    /// Fetch all active access flags from a value
    fn from_u16(value: u16) -> Vec<Self::AccessFlagType>;
}

/// Class access and property flags
// TODO: remove debug directive
#[derive(Debug, PartialEq)]
pub enum ClassAccessFlags {
    /// Declared public; may be accessed from outside its package
    AccPublic,

    /// Declared final; no subclasses allowed
    AccFinal,

    /// Treat superclass methods specially when invoked by the `invokespecial` instruction
    AccSuper,

    /// Is an interface, not a class
    AccInterface,

    /// Declared abstract; must not be instantiated
    AccAbstract,

    /// Declared synthetic; not present in the source code
    AccSynthetic,

    /// Declared as an annotation interface
    AccAnnotation,

    /// Declared as an enum class
    AccEnum,

    /// Is a module, not a class or interface
    AccModule,
}

impl AccessFlags for ClassAccessFlags {
    type AccessFlagType = ClassAccessFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

        if bitmask_matches(value, 0x0001) {
            flags.push(Self::AccPublic);
        }

        if bitmask_matches(value, 0x0010) {
            flags.push(Self::AccFinal);
        }

        if bitmask_matches(value, 0x0020) {
            flags.push(Self::AccSuper);
        }

        if bitmask_matches(value, 0x0200) {
            flags.push(Self::AccInterface);
        }

        if bitmask_matches(value, 0x0400) {
            flags.push(Self::AccAbstract);
        }

        if bitmask_matches(value, 0x1000) {
            flags.push(Self::AccSynthetic);
        }

        if bitmask_matches(value, 0x2000) {
            flags.push(Self::AccAnnotation);
        }

        if bitmask_matches(value, 0x4000) {
            flags.push(Self::AccEnum);
        }

        if bitmask_matches(value, 0x8000) {
            flags.push(Self::AccModule);
        }

        assert!(flags.len() > 0, "Bitmask does not specify ANY access flags");
        flags
    }
}

/// Field access and property flags
// TODO: remove debug directive
#[derive(Debug, PartialEq)]
pub enum FieldAccessFlags {
    /// Declared public; may be accessed from outside its package
    AccPublic,

    /// Declared private; accessible only within the defining class and other classes belonging to the same nest [§5.4.4](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-5.html#jvms-5.4.4)
    AccPrivate,

    /// Declared protected; may be accessed within subclasses
    AccProtected,

    /// Declared static
    AccStatic,

    /// Declared final; never directly assigned to after object construction (JLS §17.5)
    AccFinal,

    /// Declared volatile; cannot be cached
    AccVolatile,

    /// Declared transient; not written or read by a persistent object manager
    AccTransient,

    /// Declared synthetic; not present in the source code
    AccSynthetic,

    /// Declared as an element of an enum class
    AccEnum,
}

impl AccessFlags for FieldAccessFlags {
    type AccessFlagType = FieldAccessFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

        if bitmask_matches(value, 0x0001) {
            flags.push(Self::AccPublic);
        }

        if bitmask_matches(value, 0x0002) {
            flags.push(Self::AccPrivate);
        }

        if bitmask_matches(value, 0x0004) {
            flags.push(Self::AccProtected);
        }

        if bitmask_matches(value, 0x0008) {
            flags.push(Self::AccStatic);
        }

        if bitmask_matches(value, 0x0010) {
            flags.push(Self::AccFinal);
        }

        if bitmask_matches(value, 0x0040) {
            flags.push(Self::AccVolatile);
        }

        if bitmask_matches(value, 0x0080) {
            flags.push(Self::AccTransient);
        }

        if bitmask_matches(value, 0x1000) {
            flags.push(Self::AccSynthetic);
        }

        if bitmask_matches(value, 0x4000) {
            flags.push(Self::AccEnum);
        }

        assert!(flags.len() > 0, "Bitmask does not specify ANY access flags");
        flags
    }
}

/// Method access and property flags
// TODO: remove debug directive
#[derive(Debug, PartialEq)]
pub enum MethodAccessFlags {
    /// Declared public; may be accessed from outside its package
    AccPublic,

    /// Declared private; accessible only within the defining class and other classes belonging to the same nest [§5.4.4](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-5.html#jvms-5.4.4)
    AccPrivate,

    /// Declared protected; may be accessed within subclasses
    AccProtected,

    /// Declared static
    AccStatic,

    /// Declared final; must not be overridden [§5.4.5](https://docs.oracle.com/javase/specs/jvms/se17/html/jvms-5.html#jvms-5.4.5)
    AccFinal,

    /// Declared synchronized; invocation is wrapped by a monitor use
    AccSynchronized,

    /// A bridge method, generated by the compiler
    AccBridge,

    /// Declared with variable number of arguments
    AccVarArgs,

    /// Declared native; implemented in a language other than the Java programming language
    AccNative,

    /// Declared abstract; no implementation is provided
    AccAbstract,

    /// In a class file whose major version number is at least 46 and at most 60: Declared strictfp
    AccStrict,

    /// Declared synthetic; not present in the source code
    AccSynthetic,
}

impl AccessFlags for MethodAccessFlags {
    type AccessFlagType = MethodAccessFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

        if bitmask_matches(value, 0x0001) {
            flags.push(Self::AccPublic);
        }

        if bitmask_matches(value, 0x0002) {
            flags.push(Self::AccPrivate);
        }

        if bitmask_matches(value, 0x0004) {
            flags.push(Self::AccProtected);
        }

        if bitmask_matches(value, 0x0008) {
            flags.push(Self::AccStatic);
        }

        if bitmask_matches(value, 0x0010) {
            flags.push(Self::AccFinal);
        }

        if bitmask_matches(value, 0x0020) {
            flags.push(Self::AccSynchronized);
        }

        if bitmask_matches(value, 0x0040) {
            flags.push(Self::AccBridge);
        }

        if bitmask_matches(value, 0x0080) {
            flags.push(Self::AccVarArgs);
        }

        if bitmask_matches(value, 0x0100) {
            flags.push(Self::AccNative);
        }

        if bitmask_matches(value, 0x0400) {
            flags.push(Self::AccAbstract);
        }

        if bitmask_matches(value, 0x0800) {
            flags.push(Self::AccStrict);
        }

        if bitmask_matches(value, 0x1000) {
            flags.push(Self::AccSynthetic);
        }

        assert!(flags.len() > 0, "Bitmask does not specify ANY access flags");
        flags
    }
}

/// Nested class access and property flags
// TODO: remove debug directive
#[derive(Debug, PartialEq)]
pub enum NestedClassAccessFlags {
    /// Marked or implicitly public in source
    AccPublic,

    /// Marked private in source
    AccPrivate,

    /// Marked protected in source
    AccProtected,

    /// Marked or implicitly static in source
    AccStatic,

    /// Marked or implicitly final in source
    AccFinal,

    /// Was an interface in source
    AccInterface,

    /// Marked or implicitly abstract in source.
    AccAbstract,

    /// Declared synthetic; not present in the source code
    AccSynthetic,

    /// Declared as an annotation interface
    AccAnnotation,

    /// Declared as an enum class
    AccEnum,
}

impl AccessFlags for NestedClassAccessFlags {
    type AccessFlagType = NestedClassAccessFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

        if bitmask_matches(value, 0x0001) {
            flags.push(Self::AccPublic);
        }

        if bitmask_matches(value, 0x0002) {
            flags.push(Self::AccPrivate);
        }

        if bitmask_matches(value, 0x0004) {
            flags.push(Self::AccProtected);
        }

        if bitmask_matches(value, 0x0008) {
            flags.push(Self::AccStatic);
        }

        if bitmask_matches(value, 0x0010) {
            flags.push(Self::AccFinal);
        }

        if bitmask_matches(value, 0x0200) {
            flags.push(Self::AccInterface);
        }

        if bitmask_matches(value, 0x0400) {
            flags.push(Self::AccAbstract);
        }

        if bitmask_matches(value, 0x1000) {
            flags.push(Self::AccSynthetic);
        }

        if bitmask_matches(value, 0x2000) {
            flags.push(Self::AccAnnotation);
        }

        if bitmask_matches(value, 0x4000) {
            flags.push(Self::AccEnum);
        }

        assert!(flags.len() > 0, "Bitmask does not specify ANY access flags");
        flags
    }
}

/// Method parameter access flags
#[derive(Debug, PartialEq)]
pub enum MethodParameterAccessFlags {
    /// Indicates that the formal parameter was declared `final`
    AccFinal,

    /// Indicates that the formal parameter was not explicitly or implicitly declared in sourcecode,
    /// according to the specification of the language in which the source code was written
    ///
    /// The formal parameter is an implementation artifact of the compiler which produced this class
    /// file
    AccSynthetic,

    /// Indicates that the formal parameter was implicitly declared in source code, according to the
    /// specification of the language in which the source code was written
    ///
    /// The formal parameter is mandated by a language specification, so all compilers for the
    /// language must emit it
    AccMandated
}

impl AccessFlags for MethodParameterAccessFlags {
    type AccessFlagType = MethodParameterAccessFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

        if bitmask_matches(value, 0x0010) {
            flags.push(Self::AccFinal);
        }

        if bitmask_matches(value, 0x1000) {
            flags.push(Self::AccSynthetic);
        }

        if bitmask_matches(value, 0x8000) {
            flags.push(Self::AccMandated);
        }

        assert!(flags.len() > 0, "Bitmask does not specify ANY access flags");
        flags
    }
}

mod tests {
    use crate::access_flags::{AccessFlags, ClassAccessFlags, FieldAccessFlags, MethodAccessFlags, NestedClassAccessFlags, MethodParameterAccessFlags};

    #[test]
    fn test_class_access_flag_public() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x0001)[0],
            ClassAccessFlags::AccPublic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_final() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x0010)[0],
            ClassAccessFlags::AccFinal,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_super() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x0020)[0],
            ClassAccessFlags::AccSuper,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_interface() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x0200)[0],
            ClassAccessFlags::AccInterface,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_abstract() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x0400)[0],
            ClassAccessFlags::AccAbstract,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_synthetic() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x1000)[0],
            ClassAccessFlags::AccSynthetic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_annotation() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x2000)[0],
            ClassAccessFlags::AccAnnotation,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_enum() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x4000)[0],
            ClassAccessFlags::AccEnum,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_module() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x8000)[0],
            ClassAccessFlags::AccModule,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_multiple_flags() {
        assert_eq!(
            ClassAccessFlags::from_u16(0x4420),
            vec![
                ClassAccessFlags::AccSuper,
                ClassAccessFlags::AccAbstract,
                ClassAccessFlags::AccEnum
            ],
            "Incorrect access flags returned"
        );
    }

    #[test]
    fn test_field_access_flag_public() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x0001)[0],
            FieldAccessFlags::AccPublic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_final() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x0002)[0],
            FieldAccessFlags::AccPrivate,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_super() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x0004)[0],
            FieldAccessFlags::AccProtected,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_interface() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x0008)[0],
            FieldAccessFlags::AccStatic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_abstract() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x0010)[0],
            FieldAccessFlags::AccFinal,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_synthetic() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x0040)[0],
            FieldAccessFlags::AccVolatile,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_annotation() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x0080)[0],
            FieldAccessFlags::AccTransient,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_enum() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x1000)[0],
            FieldAccessFlags::AccSynthetic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_module() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x4000)[0],
            FieldAccessFlags::AccEnum,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_multiple_flags() {
        assert_eq!(
            FieldAccessFlags::from_u16(0x5082),
            vec![
                FieldAccessFlags::AccPrivate,
                FieldAccessFlags::AccTransient,
                FieldAccessFlags::AccSynthetic,
                FieldAccessFlags::AccEnum
            ],
            "Incorrect access flags returned"
        );
    }

    #[test]
    fn test_method_access_flag_public() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0001)[0],
            MethodAccessFlags::AccPublic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_private() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0002)[0],
            MethodAccessFlags::AccPrivate,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_protected() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0004)[0],
            MethodAccessFlags::AccProtected,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_static() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0008)[0],
            MethodAccessFlags::AccStatic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_final() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0010)[0],
            MethodAccessFlags::AccFinal,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_synchronized() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0020)[0],
            MethodAccessFlags::AccSynchronized,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_bridge() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0040)[0],
            MethodAccessFlags::AccBridge,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_varargs() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0080)[0],
            MethodAccessFlags::AccVarArgs,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_native() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0100)[0],
            MethodAccessFlags::AccNative,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_abstract() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0400)[0],
            MethodAccessFlags::AccAbstract,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_strict() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x0800)[0],
            MethodAccessFlags::AccStrict,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_flag_synthetic() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x1000)[0],
            MethodAccessFlags::AccSynthetic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_access_multiple_flags() {
        assert_eq!(
            MethodAccessFlags::from_u16(0x1533),
            vec![
                MethodAccessFlags::AccPublic,
                MethodAccessFlags::AccPrivate,
                MethodAccessFlags::AccFinal,
                MethodAccessFlags::AccSynchronized,
                MethodAccessFlags::AccNative,
                MethodAccessFlags::AccAbstract,
                MethodAccessFlags::AccSynthetic,
            ],
            "Incorrect access flags returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_public() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x0001)[0],
            NestedClassAccessFlags::AccPublic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_private() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x0002)[0],
            NestedClassAccessFlags::AccPrivate,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_protected() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x0004)[0],
            NestedClassAccessFlags::AccProtected,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_static() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x0008)[0],
            NestedClassAccessFlags::AccStatic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_final() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x0010)[0],
            NestedClassAccessFlags::AccFinal,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_synchronized() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x0200)[0],
            NestedClassAccessFlags::AccInterface,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_bridge() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x0400)[0],
            NestedClassAccessFlags::AccAbstract,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_varargs() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x1000)[0],
            NestedClassAccessFlags::AccSynthetic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_native() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x2000)[0],
            NestedClassAccessFlags::AccAnnotation,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_flag_abstract() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x4000)[0],
            NestedClassAccessFlags::AccEnum,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_nested_class_access_multiple_flags() {
        assert_eq!(
            NestedClassAccessFlags::from_u16(0x7617),
            vec![
                NestedClassAccessFlags::AccPublic,
                NestedClassAccessFlags::AccPrivate,
                NestedClassAccessFlags::AccProtected,
                NestedClassAccessFlags::AccFinal,
                NestedClassAccessFlags::AccInterface,
                NestedClassAccessFlags::AccAbstract,
                NestedClassAccessFlags::AccSynthetic,
                NestedClassAccessFlags::AccAnnotation,
                NestedClassAccessFlags::AccEnum,
            ],
            "Incorrect access flags returned"
        );
    }

    #[test]
    fn test_method_parameter_access_flag_final() {
        assert_eq!(
            MethodParameterAccessFlags::from_u16(0x0010)[0],
            MethodParameterAccessFlags::AccFinal,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_parameter_access_flag_synthetic() {
        assert_eq!(
            MethodParameterAccessFlags::from_u16(0x1000)[0],
            MethodParameterAccessFlags::AccSynthetic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_parameter_access_flag_mandated() {
        assert_eq!(
            MethodParameterAccessFlags::from_u16(0x8000)[0],
            MethodParameterAccessFlags::AccMandated,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_method_parameter_access_multiple_flags() {
        assert_eq!(
            MethodParameterAccessFlags::from_u16(0x9010),
            vec![
                MethodParameterAccessFlags::AccFinal,
                MethodParameterAccessFlags::AccSynthetic,
                MethodParameterAccessFlags::AccMandated
            ],
            "Incorrect access flags returned"
        );
    }
}
