//! Contains all flags used within the JVM

// Re-export modules to make it easy to use for any code outside of this module
pub use class_access_flags::ClassAccessFlags;
pub use field_access_flags::FieldAccessFlags;
pub use method_access_flags::MethodAccessFlags;
pub use method_parameter_access_flags::MethodParameterAccessFlags;
pub use module_exports_flags::ModuleExportsFlags;
pub use module_flags::ModuleFlags;
pub use module_opens_flags::ModuleOpensFlags;
pub use module_requires_flags::ModuleRequiresFlags;
pub use nested_class_access_flags::NestedClassAccessFlags;

mod class_access_flags;
mod field_access_flags;
mod method_access_flags;
mod method_parameter_access_flags;
mod module_exports_flags;
mod module_flags;
mod module_opens_flags;
mod module_requires_flags;
mod nested_class_access_flags;

/// Base trait for all flag types
pub trait Flags {
    type AccessFlagType;

    /// Fetch all flags from a value
    fn from_u16(value: u16) -> Vec<Self::AccessFlagType>;
}
