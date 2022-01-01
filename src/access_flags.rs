//! Contains class access and property modifiers

use crate::utils::bitmask_matches;

/// Class access and property modifiers
// TODO: remove debug directive
#[derive(Debug, PartialEq)]
pub enum AccessFlags {
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

impl AccessFlags {
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

mod tests {
    #[test]
    fn test_flag_public() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x0001)[0],
            crate::access_flags::AccessFlags::AccPublic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_flag_final() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x0010)[0],
            crate::access_flags::AccessFlags::AccFinal,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_flag_super() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x0020)[0],
            crate::access_flags::AccessFlags::AccSuper,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_flag_interface() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x0200)[0],
            crate::access_flags::AccessFlags::AccInterface,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_flag_abstract() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x0400)[0],
            crate::access_flags::AccessFlags::AccAbstract,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_flag_synthetic() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x1000)[0],
            crate::access_flags::AccessFlags::AccSynthetic,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_flag_annotation() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x2000)[0],
            crate::access_flags::AccessFlags::AccAnnotation,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_flag_enum() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x4000)[0],
            crate::access_flags::AccessFlags::AccEnum,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_flag_module() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x8000)[0],
            crate::access_flags::AccessFlags::AccModule,
            "Incorrect access flag returned"
        );
    }

    #[test]
    fn test_multiple_flags() {
        assert_eq!(
            crate::access_flags::AccessFlags::from_u16(0x4420),
            vec![
                crate::access_flags::AccessFlags::AccSuper,
                crate::access_flags::AccessFlags::AccAbstract,
                crate::access_flags::AccessFlags::AccEnum
            ],
            "Incorrect access flags returned"
        );
    }
}
