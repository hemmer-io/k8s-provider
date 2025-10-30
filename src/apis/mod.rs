//! Apis Service
//!
//! Auto-generated service module for apis

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis
pub struct ApisService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> ApisService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
    /// Get endpointslice resource handler
    pub fn endpointslice(&self) -> resources::Endpointslice<'_> {
        resources::Endpointslice::new(self.provider)
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
