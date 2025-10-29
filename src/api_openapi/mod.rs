//! Api_openapi Service
//!
//! Auto-generated service module for api_openapi

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for api_openapi
pub struct Api_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Api_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get api resource handler
    pub fn api(&self) -> resources::Api<'_> {
        resources::Api::new(self.provider)
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
