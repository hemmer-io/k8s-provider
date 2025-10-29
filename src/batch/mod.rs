//! Apis__batch_openapi Service
//!
//! Auto-generated service module for > batch

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > batch
pub struct Apis__batch_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__batch_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get batch resource handler
    pub fn batch(&self) -> resources::Batch<'_> {
        resources::Batch::new(self.provider)
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
