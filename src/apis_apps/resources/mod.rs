//! Resource modules

pub mod controllerrevision;
pub use controllerrevision::Controllerrevision;
pub mod statu;
pub use statu::Statu;
pub mod scale;
pub use scale::Scale;
pub mod deployment;
pub use deployment::Deployment;
pub mod replicaset;
pub use replicaset::Replicaset;
pub mod daemonset;
pub use daemonset::Daemonset;
pub mod v1;
pub use v1::V1;
pub mod statefulset;
pub use statefulset::Statefulset;
pub mod app;
pub use app::App;

