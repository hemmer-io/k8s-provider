//! Resource modules

pub mod api;
pub use api::Api;
pub mod discovery;
pub use discovery::Discovery;
pub mod endpointslice;
pub use endpointslice::Endpointslice;
pub mod v1;
pub use v1::V1;

