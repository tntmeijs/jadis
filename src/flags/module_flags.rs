use crate::utils::bitmask_matches;

use super::Flags;

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

mod tests {
    use super::Flags;
    use super::ModuleFlags;

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
