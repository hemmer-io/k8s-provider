//! Apis__scheduling Service
//!
//! Auto-generated service module for > scheduling

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > scheduling
pub struct Apis__schedulingService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__schedulingService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get scheduling_k8s_io resource handler
    pub fn scheduling_k8s_io(&self) -> resources::Scheduling_k8s_io<'_> {
        resources::Scheduling_k8s_io::new(self.provider)
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
