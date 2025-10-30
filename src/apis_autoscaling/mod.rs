//! Apis_autoscaling Service
//!
//! Auto-generated service module for apis_autoscaling

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_autoscaling
pub struct Apis_autoscalingService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_autoscalingService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get horizontalpodautoscaler resource handler
    pub fn horizontalpodautoscaler(&self) -> resources::Horizontalpodautoscaler<'_> {
        resources::Horizontalpodautoscaler::new(self.provider)
    }
    /// Get v2 resource handler
    pub fn v2(&self) -> resources::V2<'_> {
        resources::V2::new(self.provider)
    }
    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
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
