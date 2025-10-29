//! Logs_openapi Service
//!
//! Auto-generated service module for > logs

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > logs
pub struct Logs_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Logs_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get log resource handler
    pub fn log(&self) -> resources::Log<'_> {
        resources::Log::new(self.provider)
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
