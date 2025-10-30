//! Apis_coordination Service
//!
//! Auto-generated service module for apis_coordination

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_coordination
pub struct Apis_coordinationService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_coordinationService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get leasecandidate resource handler
    pub fn leasecandidate(&self) -> resources::Leasecandidate<'_> {
        resources::Leasecandidate::new(self.provider)
    }
    /// Get v1beta1 resource handler
    pub fn v1beta1(&self) -> resources::V1beta1<'_> {
        resources::V1beta1::new(self.provider)
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
