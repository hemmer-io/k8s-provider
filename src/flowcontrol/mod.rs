//! Apis__flowcontrol Service
//!
//! Auto-generated service module for > flowcontrol

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > flowcontrol
pub struct Apis__flowcontrolService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__flowcontrolService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get prioritylevelconfiguration resource handler
    pub fn prioritylevelconfiguration(&self) -> resources::Prioritylevelconfiguration<'_> {
        resources::Prioritylevelconfiguration::new(self.provider)
    }
    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
    }
    /// Get flowschema resource handler
    pub fn flowschema(&self) -> resources::Flowschema<'_> {
        resources::Flowschema::new(self.provider)
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
