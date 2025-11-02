//! Resource modules

pub mod rbac.authorization;
pub use rbac.authorization::Rbac.authorization;
pub mod rolebinding;
pub use rolebinding::Rolebinding;
pub mod clusterrolebinding;
pub use clusterrolebinding::Clusterrolebinding;
pub mod v1;
pub use v1::V1;
pub mod clusterrole;
pub use clusterrole::Clusterrole;
pub mod role;
pub use role::Role;

