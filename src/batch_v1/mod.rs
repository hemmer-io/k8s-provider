//! Apis__batch__v1_openapi Service
//!
//! Auto-generated service module for > batch_v1

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > batch_v1
pub struct Apis__batch__v1_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__batch__v1_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get cronjob resource handler
    pub fn cronjob(&self) -> resources::Cronjob<'_> {
        resources::Cronjob::new(self.provider)
    }
    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
    }
    /// Get job resource handler
    pub fn job(&self) -> resources::Job<'_> {
        resources::Job::new(self.provider)
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
