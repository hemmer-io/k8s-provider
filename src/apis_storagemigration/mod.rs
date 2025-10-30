//! Apis_storagemigration Service
//!
//! Auto-generated service module for apis_storagemigration

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_storagemigration
pub struct Apis_storagemigrationService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_storagemigrationService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get storagemigration resource handler
    pub fn storagemigration(&self) -> resources::Storagemigration<'_> {
        resources::Storagemigration::new(self.provider)
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
