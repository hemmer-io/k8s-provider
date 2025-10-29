//! Apis__policy_openapi Service
//!
//! Auto-generated service module for > policy

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > policy
pub struct Apis__policy_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__policy_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get policy resource handler
    pub fn policy(&self) -> resources::Policy<'_> {
        resources::Policy::new(self.provider)
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
