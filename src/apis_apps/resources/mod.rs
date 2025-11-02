//! Resource modules

pub mod deployment;
pub use deployment::Deployment;
pub mod daemonset;
pub use daemonset::Daemonset;
pub mod replicaset;
pub use replicaset::Replicaset;
pub mod v1;
pub use v1::V1;
pub mod statu;
pub use statu::Statu;
pub mod scale;
pub use scale::Scale;
pub mod statefulset;
pub use statefulset::Statefulset;
pub mod controllerrevision;
pub use controllerrevision::Controllerrevision;
pub mod app;
pub use app::App;

