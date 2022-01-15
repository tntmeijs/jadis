use crate::utils::bitmask_matches;

use super::Flags;

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

mod tests {
    use super::ClassAccessFlags;
    use super::Flags;

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
