//! Apis__apps_openapi Service
//!
//! Auto-generated service module for > apps

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > apps
pub struct Apis__apps_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__apps_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get app resource handler
    pub fn app(&self) -> resources::App<'_> {
        resources::App::new(self.provider)
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
