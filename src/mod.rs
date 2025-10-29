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

    /// Get openid-configuration resource handler
    pub fn openid-configuration(&self) -> resources::Openid-configuration<'_> {
        resources::Openid-configuration::new(self.provider)
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
