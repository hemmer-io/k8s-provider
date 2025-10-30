//! Apis_apps Service
//!
//! Auto-generated service module for apis_apps

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_apps
pub struct Apis_appsService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_appsService<'a> {
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
