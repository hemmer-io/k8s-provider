//! Resource modules

pub mod batch;
pub use batch::Batch;
pub mod job;
pub use job::Job;
pub mod statu;
pub use statu::Statu;
pub mod cronjob;
pub use cronjob::Cronjob;
pub mod v1;
pub use v1::V1;

