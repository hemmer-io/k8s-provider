//! Resource modules

pub mod coordination;
pub use coordination::Coordination;
pub mod leasecandidate;
pub use leasecandidate::Leasecandidate;
pub mod v1alpha2;
pub use v1alpha2::V1alpha2;
pub mod lease;
pub use lease::Lease;
pub mod v1;
pub use v1::V1;
pub mod leasecandidate;
pub use leasecandidate::Leasecandidate;
pub mod v1beta1;
pub use v1beta1::V1beta1;

