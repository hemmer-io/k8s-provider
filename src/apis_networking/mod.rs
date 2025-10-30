//! Apis_networking Service
//!
//! Auto-generated service module for apis_networking

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_networking
pub struct Apis_networkingService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_networkingService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get networking resource handler
    pub fn networking(&self) -> resources::Networking<'_> {
        resources::Networking::new(self.provider)
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
