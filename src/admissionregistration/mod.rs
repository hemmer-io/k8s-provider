//! Apis__admissionregistration Service
//!
//! Auto-generated service module for admissionregistration

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for admissionregistration
pub struct Apis__admissionregistrationService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__admissionregistrationService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get mutatingwebhookconfiguration resource handler
    pub fn mutatingwebhookconfiguration(&self) -> resources::Mutatingwebhookconfiguration<'_> {
        resources::Mutatingwebhookconfiguration::new(self.provider)
    }
    /// Get validatingadmissionpolicie resource handler
    pub fn validatingadmissionpolicie(&self) -> resources::Validatingadmissionpolicie<'_> {
        resources::Validatingadmissionpolicie::new(self.provider)
    }
    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
    }
    /// Get validatingwebhookconfiguration resource handler
    pub fn validatingwebhookconfiguration(&self) -> resources::Validatingwebhookconfiguration<'_> {
        resources::Validatingwebhookconfiguration::new(self.provider)
    }
    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
    /// Get validatingadmissionpolicybinding resource handler
    pub fn validatingadmissionpolicybinding(
        &self,
    ) -> resources::Validatingadmissionpolicybinding<'_> {
        resources::Validatingadmissionpolicybinding::new(self.provider)
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
