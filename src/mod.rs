//!  Service
//!
//! Auto-generated service module for 

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for 
pub struct Service<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Service<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get openid_configuration resource handler
    pub fn openid_configuration(&self) -> resources::Openid_configuration<'_> {
        resources::Openid_configuration::new(self.provider)
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
