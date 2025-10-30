//! Openid Service
//!
//! Auto-generated service module for openid

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for openid
pub struct OpenidService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> OpenidService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get jwk resource handler
    pub fn jwk(&self) -> resources::Jwk<'_> {
        resources::Jwk::new(self.provider)
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
