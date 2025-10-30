//! Resource modules

pub mod cronjob;
pub use cronjob::Cronjob;
pub mod job;
pub use job::Job;
pub mod v1;
pub use v1::V1;
pub mod statu;
pub use statu::Statu;

