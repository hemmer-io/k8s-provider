//! Apis__apps__v1_openapi Service
//!
//! Auto-generated service module for > apps_v1

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > apps_v1
pub struct Apis__apps__v1_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__apps__v1_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get replicaset resource handler
    pub fn replicaset(&self) -> resources::Replicaset<'_> {
        resources::Replicaset::new(self.provider)
    }
    /// Get statefulset resource handler
    pub fn statefulset(&self) -> resources::Statefulset<'_> {
        resources::Statefulset::new(self.provider)
    }
    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
    }
    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
    /// Get scale resource handler
    pub fn scale(&self) -> resources::Scale<'_> {
        resources::Scale::new(self.provider)
    }
    /// Get daemonset resource handler
    pub fn daemonset(&self) -> resources::Daemonset<'_> {
        resources::Daemonset::new(self.provider)
    }
    /// Get deployment resource handler
    pub fn deployment(&self) -> resources::Deployment<'_> {
        resources::Deployment::new(self.provider)
    }
    /// Get controllerrevision resource handler
    pub fn controllerrevision(&self) -> resources::Controllerrevision<'_> {
        resources::Controllerrevision::new(self.provider)
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
