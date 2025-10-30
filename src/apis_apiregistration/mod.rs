//! Apis_apiregistration Service
//!
//! Auto-generated service module for apis_apiregistration

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_apiregistration
pub struct Apis_apiregistrationService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_apiregistrationService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get apiregistration resource handler
    pub fn apiregistration(&self) -> resources::Apiregistration<'_> {
        resources::Apiregistration::new(self.provider)
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
