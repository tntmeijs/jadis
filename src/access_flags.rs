//! Contains class access and property modifiers

use crate::utils::bitmask_matches;

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

impl ClassAccessFlags {
    /// Fetch all active access flags from a value
    pub fn from_u16(value: u16) -> Vec<Self> {
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

impl FieldAccessFlags {
    /// Fetch all active access flags from a value
    pub fn from_u16(value: u16) -> Vec<Self> {
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

mod tests {
    #[test]
    fn test_class_access_flag_public() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x0001)[0],
            crate::access_flags::ClassAccessFlags::AccPublic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_final() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x0010)[0],
            crate::access_flags::ClassAccessFlags::AccFinal,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_super() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x0020)[0],
            crate::access_flags::ClassAccessFlags::AccSuper,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_interface() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x0200)[0],
            crate::access_flags::ClassAccessFlags::AccInterface,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_abstract() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x0400)[0],
            crate::access_flags::ClassAccessFlags::AccAbstract,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_synthetic() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x1000)[0],
            crate::access_flags::ClassAccessFlags::AccSynthetic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_annotation() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x2000)[0],
            crate::access_flags::ClassAccessFlags::AccAnnotation,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_enum() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x4000)[0],
            crate::access_flags::ClassAccessFlags::AccEnum,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_flag_module() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x8000)[0],
            crate::access_flags::ClassAccessFlags::AccModule,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_class_access_multiple_flags() {
        assert_eq!(
            crate::access_flags::ClassAccessFlags::from_u16(0x4420),
            vec![
                crate::access_flags::ClassAccessFlags::AccSuper,
                crate::access_flags::ClassAccessFlags::AccAbstract,
                crate::access_flags::ClassAccessFlags::AccEnum
            ],
            "Incorrect access flags returned"
        );
    }

    #[test]
    fn test_field_access_flag_public() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x0001)[0],
            crate::access_flags::FieldAccessFlags::AccPublic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_final() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x0002)[0],
            crate::access_flags::FieldAccessFlags::AccPrivate,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_super() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x0004)[0],
            crate::access_flags::FieldAccessFlags::AccProtected,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_interface() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x0008)[0],
            crate::access_flags::FieldAccessFlags::AccStatic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_abstract() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x0010)[0],
            crate::access_flags::FieldAccessFlags::AccFinal,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_synthetic() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x0040)[0],
            crate::access_flags::FieldAccessFlags::AccVolatile,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_annotation() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x0080)[0],
            crate::access_flags::FieldAccessFlags::AccTransient,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_enum() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x1000)[0],
            crate::access_flags::FieldAccessFlags::AccSynthetic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_flag_module() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x4000)[0],
            crate::access_flags::FieldAccessFlags::AccEnum,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_field_access_multiple_flags() {
        assert_eq!(
            crate::access_flags::FieldAccessFlags::from_u16(0x5082),
            vec![
                crate::access_flags::FieldAccessFlags::AccPrivate,
                crate::access_flags::FieldAccessFlags::AccTransient,
                crate::access_flags::FieldAccessFlags::AccSynthetic,
                crate::access_flags::FieldAccessFlags::AccEnum
            ],
            "Incorrect access flags returned"
        );
    }
}
