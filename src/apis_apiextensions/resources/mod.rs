//! Resource modules

pub mod statu;
pub use statu::Statu;
pub mod v1;
pub use v1::V1;
pub mod customresourcedefinition;
pub use customresourcedefinition::Customresourcedefinition;
pub mod apiextension;
pub use apiextension::Apiextension;

