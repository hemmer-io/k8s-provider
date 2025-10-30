//! Apis_node Service
//!
//! Auto-generated service module for apis_node

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_node
pub struct Apis_nodeService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_nodeService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get node resource handler
    pub fn node(&self) -> resources::Node<'_> {
        resources::Node::new(self.provider)
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
