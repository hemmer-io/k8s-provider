//! Resource modules

pub mod storagemigration;
pub use storagemigration::Storagemigration;
pub mod storageversionmigration;
pub use storageversionmigration::Storageversionmigration;
pub mod v1beta1;
pub use v1beta1::V1beta1;
pub mod statu;
pub use statu::Statu;

