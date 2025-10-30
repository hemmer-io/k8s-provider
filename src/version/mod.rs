//! Version Service
//!
//! Auto-generated service module for version

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for version
pub struct VersionService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> VersionService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get version resource handler
    pub fn version(&self) -> resources::Version<'_> {
        resources::Version::new(self.provider)
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
