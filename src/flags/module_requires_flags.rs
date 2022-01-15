use crate::utils::bitmask_matches;

use super::Flags;

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

mod tests {
    use super::Flags;
    use super::ModuleRequiresFlags;

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
