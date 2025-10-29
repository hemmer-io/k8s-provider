//! Apis__apiextensions Service
//!
//! Auto-generated service module for > apiextensions

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > apiextensions
pub struct Apis__apiextensionsService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__apiextensionsService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get apiextensions_k8s_io resource handler
    pub fn apiextensions_k8s_io(&self) -> resources::Apiextensions_k8s_io<'_> {
        resources::Apiextensions_k8s_io::new(self.provider)
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
