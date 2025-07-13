/// When derived on an enum, all variants of the enum are added to a [Vec].
pub trait EnumVec {
    fn all_variants() -> Vec<Self>
    where
        Self: Sized;
}
