//! Apis__authorization Service
//!
//! Auto-generated service module for > authorization

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > authorization
pub struct Apis__authorizationService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__authorizationService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get localsubjectaccessreview resource handler
    pub fn localsubjectaccessreview(&self) -> resources::Localsubjectaccessreview<'_> {
        resources::Localsubjectaccessreview::new(self.provider)
    }
    /// Get selfsubjectaccessreview resource handler
    pub fn selfsubjectaccessreview(&self) -> resources::Selfsubjectaccessreview<'_> {
        resources::Selfsubjectaccessreview::new(self.provider)
    }
    /// Get selfsubjectrulesreview resource handler
    pub fn selfsubjectrulesreview(&self) -> resources::Selfsubjectrulesreview<'_> {
        resources::Selfsubjectrulesreview::new(self.provider)
    }
    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
    /// Get subjectaccessreview resource handler
    pub fn subjectaccessreview(&self) -> resources::Subjectaccessreview<'_> {
        resources::Subjectaccessreview::new(self.provider)
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
