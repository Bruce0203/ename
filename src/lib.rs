pub use ename_derive::VariantName;

pub trait VariantName {
    fn variant_name(&self) -> &'static str;
}
