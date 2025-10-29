//! Apis__internal Service
//!
//! Auto-generated service module for > internal

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > internal
pub struct Apis__internalService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__internalService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
    }
    /// Get storageversion resource handler
    pub fn storageversion(&self) -> resources::Storageversion<'_> {
        resources::Storageversion::new(self.provider)
    }
    /// Get v1alpha1 resource handler
    pub fn v1alpha1(&self) -> resources::V1alpha1<'_> {
        resources::V1alpha1::new(self.provider)
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
