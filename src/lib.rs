#![doc = include_str!("../README.md")]

pub use viking_macros_enum::EnumDisplay;
pub use viking_macros_enum::EnumVec;

/// When derived on an enum, all variants of the enum are added to a [Vec].
pub trait EnumVec {
    fn all_variants() -> Vec<Self>
    where
        Self: Sized;
}
