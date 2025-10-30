//! Apis_events Service
//!
//! Auto-generated service module for apis_events

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for apis_events
pub struct Apis_eventsService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Apis_eventsService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
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
