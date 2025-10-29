//! Apis__rbac Service
//!
//! Auto-generated service module for > rbac

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > rbac
pub struct Apis__rbacService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__rbacService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get role resource handler
    pub fn role(&self) -> resources::Role<'_> {
        resources::Role::new(self.provider)
    }
    /// Get clusterrolebinding resource handler
    pub fn clusterrolebinding(&self) -> resources::Clusterrolebinding<'_> {
        resources::Clusterrolebinding::new(self.provider)
    }
    /// Get rolebinding resource handler
    pub fn rolebinding(&self) -> resources::Rolebinding<'_> {
        resources::Rolebinding::new(self.provider)
    }
    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
    /// Get clusterrole resource handler
    pub fn clusterrole(&self) -> resources::Clusterrole<'_> {
        resources::Clusterrole::new(self.provider)
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
