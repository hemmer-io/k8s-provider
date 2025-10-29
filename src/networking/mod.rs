//! Apis__networking Service
//!
//! Auto-generated service module for > networking

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > networking
pub struct Apis__networkingService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__networkingService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get networking_k8s_io resource handler
    pub fn networking_k8s_io(&self) -> resources::Networking_k8s_io<'_> {
        resources::Networking_k8s_io::new(self.provider)
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
