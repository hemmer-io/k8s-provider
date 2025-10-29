//! Apis__certificates Service
//!
//! Auto-generated service module for > certificates

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > certificates
pub struct Apis__certificatesService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__certificatesService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get v1beta1 resource handler
    pub fn v1beta1(&self) -> resources::V1beta1<'_> {
        resources::V1beta1::new(self.provider)
    }
    /// Get clustertrustbundle resource handler
    pub fn clustertrustbundle(&self) -> resources::Clustertrustbundle<'_> {
        resources::Clustertrustbundle::new(self.provider)
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
