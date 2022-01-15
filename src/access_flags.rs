//! Contains class access and property modifiers

use crate::utils::bitmask_matches;

/// Base trait for all flag types
pub trait Flags {
    type AccessFlagType;

    /// Fetch all flags from a value
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

impl Flags for ClassAccessFlags {
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

impl Flags for FieldAccessFlags {
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

impl Flags for MethodAccessFlags {
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

impl Flags for NestedClassAccessFlags {
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
    AccMandated,
}

impl Flags for MethodParameterAccessFlags {
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

/// Module flags
#[derive(Debug, PartialEq)]
pub enum ModuleFlags {
    /// Indicates that this module is open
    AccOpen,

    /// Indicates that this module was not explicitly or implicitly declared
    AccSynthetic,

    /// Indicates that this module was implicitly declared
    AccMandated,
}

impl Flags for ModuleFlags {
    type AccessFlagType = ModuleFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

        if bitmask_matches(value, 0x0020) {
            flags.push(Self::AccOpen);
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

/// Module requires flags
#[derive(Debug, PartialEq)]
pub enum ModuleRequiresFlags {
    /// Indicates that any module which depends on the current module, implicitly declares a
    /// dependence on the module indicated by this entry
    AccTransitive,

    /// Indicates that this dependence is mandatory in the static phase, i.e., at compile time, but
    /// is optional in the dynamic phase, i.e., at run time
    AccStaticPhase,

    /// Indicates that this dependence was not explicitly or implicitly declared in the source of
    /// the module declaration
    AccSynthetic,

    /// Indicates that this dependence was implicitly declared in the source of the module
    /// declaration
    AccMandated,
}

impl Flags for ModuleRequiresFlags {
    type AccessFlagType = ModuleRequiresFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

        if bitmask_matches(value, 0x0020) {
            flags.push(Self::AccTransitive);
        }

        if bitmask_matches(value, 0x0040) {
            flags.push(Self::AccStaticPhase);
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

/// Module exports flags
#[derive(Debug, PartialEq)]
pub enum ModuleExportsFlags {
    /// Indicates that this export was not explicitly or implicitly declared in the source of the
    /// module declaration
    AccSynthetic,

    /// Indicates that this export was implicitly declared in the source of the module declaration
    AccMandated
}

impl Flags for ModuleExportsFlags {
    type AccessFlagType = ModuleExportsFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

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

/// Module opens flags
#[derive(Debug, PartialEq)]
pub enum ModuleOpensFlags {
    /// Indicates that this opening was not explicitly or implicitly declared in the source of the
    /// module declaration
    AccSynthetic,

    /// Indicates that this opening was implicitly declared in the source of the module declaration
    AccMandated
}

impl Flags for ModuleOpensFlags {
    type AccessFlagType = ModuleOpensFlags;

    fn from_u16(value: u16) -> Vec<Self::AccessFlagType> {
        let mut flags = vec![];

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
    mod class_access {
        use crate::access_flags::{ClassAccessFlags, Flags};

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
    }

    mod field_access {
        use crate::access_flags::{FieldAccessFlags, Flags};

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
    }

    mod method_access {
        use crate::access_flags::{MethodAccessFlags, Flags};

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
    }

    mod nested_class {
        use crate::access_flags::{NestedClassAccessFlags, Flags};

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
    }

    mod method_parameter {
        use crate::access_flags::{MethodParameterAccessFlags, Flags};

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

    mod module {
        use crate::access_flags::{ModuleFlags, Flags};

        #[test]
        fn test_module_access_flag_open() {
            assert_eq!(
                ModuleFlags::from_u16(0x0020)[0],
                ModuleFlags::AccOpen,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_access_flag_synthetic() {
            assert_eq!(
                ModuleFlags::from_u16(0x1000)[0],
                ModuleFlags::AccSynthetic,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_access_flag_mandated() {
            assert_eq!(
                ModuleFlags::from_u16(0x8000)[0],
                ModuleFlags::AccMandated,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_access_multiple_flags() {
            assert_eq!(
                ModuleFlags::from_u16(0x9020),
                vec![
                    ModuleFlags::AccOpen,
                    ModuleFlags::AccSynthetic,
                    ModuleFlags::AccMandated
                ],
                "Incorrect access flags returned"
            );
        }
    }

    mod module_requires {
        use crate::access_flags::{ModuleRequiresFlags, Flags};

        #[test]
        fn test_module_requires_access_flag_transitive() {
            assert_eq!(
                ModuleRequiresFlags::from_u16(0x0020)[0],
                ModuleRequiresFlags::AccTransitive,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_requires_access_flag_static_phase() {
            assert_eq!(
                ModuleRequiresFlags::from_u16(0x0040)[0],
                ModuleRequiresFlags::AccStaticPhase,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_requires_access_flag_synthetic() {
            assert_eq!(
                ModuleRequiresFlags::from_u16(0x1000)[0],
                ModuleRequiresFlags::AccSynthetic,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_requires_access_flag_mandated() {
            assert_eq!(
                ModuleRequiresFlags::from_u16(0x8000)[0],
                ModuleRequiresFlags::AccMandated,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_requires_access_multiple_flags() {
            assert_eq!(
                ModuleRequiresFlags::from_u16(0x9060),
                vec![
                    ModuleRequiresFlags::AccTransitive,
                    ModuleRequiresFlags::AccStaticPhase,
                    ModuleRequiresFlags::AccSynthetic,
                    ModuleRequiresFlags::AccMandated
                ],
                "Incorrect access flags returned"
            );
        }
    }

    mod module_exports {
        use crate::access_flags::{ModuleExportsFlags, Flags};

        #[test]
        fn test_module_exports_access_flag_synthetic() {
            assert_eq!(
                ModuleExportsFlags::from_u16(0x1000)[0],
                ModuleExportsFlags::AccSynthetic,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_exports_access_flag_mandated() {
            assert_eq!(
                ModuleExportsFlags::from_u16(0x8000)[0],
                ModuleExportsFlags::AccMandated,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_exports_access_multiple_flags() {
            assert_eq!(
                ModuleExportsFlags::from_u16(0x9000),
                vec![
                    ModuleExportsFlags::AccSynthetic,
                    ModuleExportsFlags::AccMandated
                ],
                "Incorrect access flags returned"
            );
        }
    }

    mod module_opens {
        use crate::access_flags::{ModuleOpensFlags, Flags};

        #[test]
        fn test_module_opens_access_flag_synthetic() {
            assert_eq!(
                ModuleOpensFlags::from_u16(0x1000)[0],
                ModuleOpensFlags::AccSynthetic,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_opens_access_flag_mandated() {
            assert_eq!(
                ModuleOpensFlags::from_u16(0x8000)[0],
                ModuleOpensFlags::AccMandated,
                "Incorrect access flag returned"
            );
        }

        #[test]
        fn test_module_opens_access_multiple_flags() {
            assert_eq!(
                ModuleOpensFlags::from_u16(0x9000),
                vec![
                    ModuleOpensFlags::AccSynthetic,
                    ModuleOpensFlags::AccMandated
                ],
                "Incorrect access flags returned"
            );
        }
    }
}
