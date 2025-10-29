//! Openid__v1__jwks_openapi Service
//!
//! Auto-generated service module for > openid_v1_jwks

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > openid_v1_jwks
pub struct Openid__v1__jwks_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Openid__v1__jwks_openapiService<'a> {
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
