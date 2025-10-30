//! Apis_policy Service
//!
//! Auto-generated service module for apis_policy

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_policy
pub struct Apis_policyService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_policyService<'a> {
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
