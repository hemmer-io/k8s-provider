//! Apis_authentication Service
//!
//! Auto-generated service module for apis_authentication

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_authentication
pub struct Apis_authenticationService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_authenticationService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
    /// Get selfsubjectreview resource handler
    pub fn selfsubjectreview(&self) -> resources::Selfsubjectreview<'_> {
        resources::Selfsubjectreview::new(self.provider)
    }
    /// Get tokenreview resource handler
    pub fn tokenreview(&self) -> resources::Tokenreview<'_> {
        resources::Tokenreview::new(self.provider)
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
