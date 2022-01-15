use crate::utils::bitmask_matches;

use super::Flags;

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

mod tests {
    use super::Flags;
    use super::NestedClassAccessFlags;

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
