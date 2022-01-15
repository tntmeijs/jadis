use crate::utils::bitmask_matches;

use super::Flags;

/// Module opens flags
#[derive(Debug, PartialEq)]
pub enum ModuleOpensFlags {
    /// Indicates that this opening was not explicitly or implicitly declared in the source of the
    /// module declaration
    AccSynthetic,

    /// Indicates that this opening was implicitly declared in the source of the module declaration
    AccMandated,
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
    use super::Flags;
    use super::ModuleOpensFlags;

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
