//! Api__v1_openapi Service
//!
//! Auto-generated service module for > core_v1

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for > core_v1
pub struct Api__v1_openapiService<'a> {
    provider: &'a crate::KubernetesProvider,
}

impl<'a> Api__v1_openapiService<'a> {
    pub(crate) fn new(provider: &'a crate::KubernetesProvider) -> Self {
        Self { provider }
    }

    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get namespace resource handler
    pub fn namespace(&self) -> resources::Namespace<'_> {
        resources::Namespace::new(self.provider)
    }
    /// Get scale resource handler
    pub fn scale(&self) -> resources::Scale<'_> {
        resources::Scale::new(self.provider)
    }
    /// Get eviction resource handler
    pub fn eviction(&self) -> resources::Eviction<'_> {
        resources::Eviction::new(self.provider)
    }
    /// Get ephemeralcontainer resource handler
    pub fn ephemeralcontainer(&self) -> resources::Ephemeralcontainer<'_> {
        resources::Ephemeralcontainer::new(self.provider)
    }
    /// Get statu resource handler
    pub fn statu(&self) -> resources::Statu<'_> {
        resources::Statu::new(self.provider)
    }
    /// Get binding resource handler
    pub fn binding(&self) -> resources::Binding<'_> {
        resources::Binding::new(self.provider)
    }
    /// Get secret resource handler
    pub fn secret(&self) -> resources::Secret<'_> {
        resources::Secret::new(self.provider)
    }
    /// Get podtemplate resource handler
    pub fn podtemplate(&self) -> resources::Podtemplate<'_> {
        resources::Podtemplate::new(self.provider)
    }
    /// Get resize resource handler
    pub fn resize(&self) -> resources::Resize<'_> {
        resources::Resize::new(self.provider)
    }
    /// Get proxy resource handler
    pub fn proxy(&self) -> resources::Proxy<'_> {
        resources::Proxy::new(self.provider)
    }
    /// Get configmap resource handler
    pub fn configmap(&self) -> resources::Configmap<'_> {
        resources::Configmap::new(self.provider)
    }
    /// Get pod resource handler
    pub fn pod(&self) -> resources::Pod<'_> {
        resources::Pod::new(self.provider)
    }
    /// Get limitrange resource handler
    pub fn limitrange(&self) -> resources::Limitrange<'_> {
        resources::Limitrange::new(self.provider)
    }
    /// Get exec resource handler
    pub fn exec(&self) -> resources::Exec<'_> {
        resources::Exec::new(self.provider)
    }
    /// Get componentstatuse resource handler
    pub fn componentstatuse(&self) -> resources::Componentstatuse<'_> {
        resources::Componentstatuse::new(self.provider)
    }
    /// Get portforward resource handler
    pub fn portforward(&self) -> resources::Portforward<'_> {
        resources::Portforward::new(self.provider)
    }
    /// Get v1 resource handler
    pub fn v1(&self) -> resources::V1<'_> {
        resources::V1::new(self.provider)
    }
    /// Get node resource handler
    pub fn node(&self) -> resources::Node<'_> {
        resources::Node::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get resourcequota resource handler
    pub fn resourcequota(&self) -> resources::Resourcequota<'_> {
        resources::Resourcequota::new(self.provider)
    }
    /// Get log resource handler
    pub fn log(&self) -> resources::Log<'_> {
        resources::Log::new(self.provider)
    }
    /// Get finalize resource handler
    pub fn finalize(&self) -> resources::Finalize<'_> {
        resources::Finalize::new(self.provider)
    }
    /// Get replicationcontroller resource handler
    pub fn replicationcontroller(&self) -> resources::Replicationcontroller<'_> {
        resources::Replicationcontroller::new(self.provider)
    }
    /// Get serviceaccount resource handler
    pub fn serviceaccount(&self) -> resources::Serviceaccount<'_> {
        resources::Serviceaccount::new(self.provider)
    }
    /// Get token resource handler
    pub fn token(&self) -> resources::Token<'_> {
        resources::Token::new(self.provider)
    }
    /// Get persistentvolumeclaim resource handler
    pub fn persistentvolumeclaim(&self) -> resources::Persistentvolumeclaim<'_> {
        resources::Persistentvolumeclaim::new(self.provider)
    }
    /// Get attach resource handler
    pub fn attach(&self) -> resources::Attach<'_> {
        resources::Attach::new(self.provider)
    }
    /// Get persistentvolume resource handler
    pub fn persistentvolume(&self) -> resources::Persistentvolume<'_> {
        resources::Persistentvolume::new(self.provider)
    }
    /// Get service resource handler
    pub fn service(&self) -> resources::Service<'_> {
        resources::Service::new(self.provider)
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
