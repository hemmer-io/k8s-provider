//! Resource modules

pub mod autoscaling;
pub use autoscaling::Autoscaling;
pub mod horizontalpodautoscaler;
pub use horizontalpodautoscaler::Horizontalpodautoscaler;
pub mod v1;
pub use v1::V1;
pub mod statu;
pub use statu::Statu;
pub mod v2;
pub use v2::V2;
pub mod statu;
pub use statu::Statu;
pub mod horizontalpodautoscaler;
pub use horizontalpodautoscaler::Horizontalpodautoscaler;

