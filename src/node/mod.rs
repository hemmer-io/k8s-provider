//! Apis__node Service
//!
//! Auto-generated service module for > node

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > node
pub struct Apis__nodeService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__nodeService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get node_k8s_io resource handler
    pub fn node_k8s_io(&self) -> resources::Node_k8s_io<'_> {
        resources::Node_k8s_io::new(self.provider)
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
