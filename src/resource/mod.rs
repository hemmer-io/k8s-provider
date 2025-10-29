//! Apis__resource Service
//!
//! Auto-generated service module for > resource

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > resource
pub struct Apis__resourceService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__resourceService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get resourceslice resource handler
    pub fn resourceslice(&self) -> resources::Resourceslice<'_> {
        resources::Resourceslice::new(self.provider)
    }
    /// Get resourceclaimtemplate resource handler
    pub fn resourceclaimtemplate(&self) -> resources::Resourceclaimtemplate<'_> {
        resources::Resourceclaimtemplate::new(self.provider)
    }
    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
    }
    /// Get deviceclasse resource handler
    pub fn deviceclasse(&self) -> resources::Deviceclasse<'_> {
        resources::Deviceclasse::new(self.provider)
    }
    /// Get resourceclaim resource handler
    pub fn resourceclaim(&self) -> resources::Resourceclaim<'_> {
        resources::Resourceclaim::new(self.provider)
    }
    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
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
