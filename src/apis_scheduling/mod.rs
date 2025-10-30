//! Apis_scheduling Service
//!
//! Auto-generated service module for apis_scheduling

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_scheduling
pub struct Apis_schedulingService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_schedulingService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get scheduling resource handler
    pub fn scheduling(&self) -> resources::Scheduling<'_> {
        resources::Scheduling::new(self.provider)
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
