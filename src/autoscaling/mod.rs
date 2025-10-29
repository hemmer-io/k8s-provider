//! Apis__autoscaling_openapi Service
//!
//! Auto-generated service module for > autoscaling

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > autoscaling
pub struct Apis__autoscaling_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__autoscaling_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get autoscaling resource handler
    pub fn autoscaling(&self) -> resources::Autoscaling<'_> {
        resources::Autoscaling::new(self.provider)
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
