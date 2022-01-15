use crate::utils::bitmask_matches;

use super::Flags;

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

mod tests {
    use super::FieldAccessFlags;
    use super::Flags;

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
