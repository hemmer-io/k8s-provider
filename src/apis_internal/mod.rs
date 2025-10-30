//! Apis_internal Service
//!
//! Auto-generated service module for apis_internal

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_internal
pub struct Apis_internalService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_internalService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get v1alpha1 resource handler
    pub fn v1alpha1(&self) -> resources::V1alpha1<'_> {
        resources::V1alpha1::new(self.provider)
    }
    /// Get storageversion resource handler
    pub fn storageversion(&self) -> resources::Storageversion<'_> {
        resources::Storageversion::new(self.provider)
    }
    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
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
