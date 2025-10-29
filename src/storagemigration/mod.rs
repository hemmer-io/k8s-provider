//! Apis__storagemigration Service
//!
//! Auto-generated service module for > storagemigration

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > storagemigration
pub struct Apis__storagemigrationService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__storagemigrationService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get storagemigration_k8s_io resource handler
    pub fn storagemigration_k8s_io(&self) -> resources::Storagemigration_k8s_io<'_> {
        resources::Storagemigration_k8s_io::new(self.provider)
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
