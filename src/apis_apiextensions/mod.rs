//! Apis_apiextensions Service
//!
//! Auto-generated service module for apis_apiextensions

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_apiextensions
pub struct Apis_apiextensionsService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_apiextensionsService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get apiextension resource handler
    pub fn apiextension(&self) -> resources::Apiextension<'_> {
        resources::Apiextension::new(self.provider)
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
