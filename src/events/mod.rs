//! Apis__events Service
//!
//! Auto-generated service module for > events

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > events
pub struct Apis__eventsService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis__eventsService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get events_k8s_io resource handler
    pub fn events_k8s_io(&self) -> resources::Events_k8s_io<'_> {
        resources::Events_k8s_io::new(self.provider)
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
