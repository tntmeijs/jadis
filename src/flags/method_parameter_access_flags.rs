use crate::utils::bitmask_matches;

use super::Flags;

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

mod tests {
    use super::Flags;
    use super::MethodParameterAccessFlags;

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
