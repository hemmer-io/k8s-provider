//! Apis_storage Service
//!
//! Auto-generated service module for apis_storage

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_storage
pub struct Apis_storageService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_storageService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get volumeattributesclasse resource handler
    pub fn volumeattributesclasse(&self) -> resources::Volumeattributesclasse<'_> {
        resources::Volumeattributesclasse::new(self.provider)
    }
    /// Get v1beta1 resource handler
    pub fn v1beta1(&self) -> resources::V1beta1<'_> {
        resources::V1beta1::new(self.provider)
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
