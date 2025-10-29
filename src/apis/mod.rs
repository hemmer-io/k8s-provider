//! Apis_openapi Service
//!
//! Auto-generated service module for > apis

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > apis
pub struct Apis_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_openapiService<'a> {
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
