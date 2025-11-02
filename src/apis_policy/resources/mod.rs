//! Resource modules

pub mod v1;
pub use v1::V1;
pub mod poddisruptionbudget;
pub use poddisruptionbudget::Poddisruptionbudget;
pub mod statu;
pub use statu::Statu;
pub mod policy;
pub use policy::Policy;

