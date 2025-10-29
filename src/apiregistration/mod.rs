//! Apis__apiregistration Service
//!
//! Auto-generated service module for > apiregistration

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > apiregistration
pub struct Apis__apiregistrationService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__apiregistrationService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get apiregistration_k8s_io resource handler
    pub fn apiregistration_k8s_io(&self) -> resources::Apiregistration_k8s_io<'_> {
        resources::Apiregistration_k8s_io::new(self.provider)
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
