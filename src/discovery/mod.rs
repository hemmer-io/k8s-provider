//! Apis__discovery Service
//!
//! Auto-generated service module for > discovery

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > discovery
pub struct Apis__discoveryService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__discoveryService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get endpointslice resource handler
    pub fn endpointslice(&self) -> resources::Endpointslice<'_> {
        resources::Endpointslice::new(self.provider)
    }
    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
