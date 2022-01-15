use crate::utils::bitmask_matches;

use super::Flags;

/// Module exports flags
#[derive(Debug, PartialEq)]
pub enum ModuleExportsFlags {
    /// Indicates that this export was not explicitly or implicitly declared in the source of the
    /// module declaration
    AccSynthetic,

    /// Indicates that this export was implicitly declared in the source of the module declaration
    AccMandated,
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

mod tests {
    use super::Flags;
    use super::ModuleExportsFlags;

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
