//! Resource modules

pub mod role;
pub use role::Role;
pub mod clusterrolebinding;
pub use clusterrolebinding::Clusterrolebinding;
pub mod rolebinding;
pub use rolebinding::Rolebinding;
pub mod v1;
pub use v1::V1;
pub mod clusterrole;
pub use clusterrole::Clusterrole;
