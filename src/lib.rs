//! K8s Provider for Hemmer
//!
//! Auto-generated unified provider from k8s SDK version v1
//!
//! This provider includes multiple services:
//! - apis_storage
//! - apis_rbac
//! - apis_events
//! - apis_flowcontrol
//! - version
//! - openid
//! - apis_apps
//! - apis_scheduling
//! - apis
//! - apis_storagemigration
//! - apis_certificates
//! - apis_batch
//! - apis_node
//! - apis_resource
//! - apis_networking
//! - apis_apiregistration
//! - apis_internal
//! - logs
//! - apis_policy
//! - apis_authorization
//! - apis_apiextensions
//! - apis_autoscaling
//! - api
//! - apis_authentication
//! - apis_admissionregistration
//! - apis_coordination

pub mod api;
pub mod apis;
pub mod apis_admissionregistration;
pub mod apis_apiextensions;
pub mod apis_apiregistration;
pub mod apis_apps;
pub mod apis_authentication;
pub mod apis_authorization;
pub mod apis_autoscaling;
pub mod apis_batch;
pub mod apis_certificates;
pub mod apis_coordination;
pub mod apis_events;
pub mod apis_flowcontrol;
pub mod apis_internal;
pub mod apis_networking;
pub mod apis_node;
pub mod apis_policy;
pub mod apis_rbac;
pub mod apis_resource;
pub mod apis_scheduling;
pub mod apis_storage;
pub mod apis_storagemigration;
pub mod logs;
pub mod openid;
pub mod version;

use async_trait::async_trait;
use hemmer_core::Result;
use hemmer_provider::{
    ProviderConfig, ProviderExecutor, ResourceInput, ResourceOutput, ResourcePlan,
};
use thiserror::Error;

/// Provider error types
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("SDK error: {0}")]
    SdkError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for provider operations
pub type Result<T> = std::result::Result<T, ProviderError>;

/// Unified provider client for K8s
pub struct K8sProvider {
    kube_client: kube::Client,
    runtime: tokio::runtime::Runtime,
}

impl K8sProvider {
    /// Create a new unified provider instance
    pub fn new() -> Result<Self> {
        // Create Tokio runtime for async operations
        // This ensures async AWS SDK calls work when the provider is loaded as a dynamic library
        let runtime = tokio::runtime::Runtime::new().map_err(|e| {
            ProviderError::SdkError(format!("Failed to create Tokio runtime: {}", e))
        })?;

        // Initialize Kubernetes client using the runtime
        let kube_client = runtime.block_on(async {
            kube::Client::try_default().await.map_err(|e| {
                ProviderError::SdkError(format!("Failed to create kube client: {}", e))
            })
        })?;

        Ok(Self {
            kube_client,
            runtime,
        })
    }

    /// Get apis_storage service handler
    pub fn apis_storage(&self) -> apis_storage::Apis_storageService<'_> {
        apis_storage::Apis_storageService::new(self)
    }
    /// Get apis_rbac service handler
    pub fn apis_rbac(&self) -> apis_rbac::Apis_rbacService<'_> {
        apis_rbac::Apis_rbacService::new(self)
    }
    /// Get apis_events service handler
    pub fn apis_events(&self) -> apis_events::Apis_eventsService<'_> {
        apis_events::Apis_eventsService::new(self)
    }
    /// Get apis_flowcontrol service handler
    pub fn apis_flowcontrol(&self) -> apis_flowcontrol::Apis_flowcontrolService<'_> {
        apis_flowcontrol::Apis_flowcontrolService::new(self)
    }
    /// Get version service handler
    pub fn version(&self) -> version::VersionService<'_> {
        version::VersionService::new(self)
    }
    /// Get openid service handler
    pub fn openid(&self) -> openid::OpenidService<'_> {
        openid::OpenidService::new(self)
    }
    /// Get apis_apps service handler
    pub fn apis_apps(&self) -> apis_apps::Apis_appsService<'_> {
        apis_apps::Apis_appsService::new(self)
    }
    /// Get apis_scheduling service handler
    pub fn apis_scheduling(&self) -> apis_scheduling::Apis_schedulingService<'_> {
        apis_scheduling::Apis_schedulingService::new(self)
    }
    /// Get apis service handler
    pub fn apis(&self) -> apis::ApisService<'_> {
        apis::ApisService::new(self)
    }
    /// Get apis_storagemigration service handler
    pub fn apis_storagemigration(&self) -> apis_storagemigration::Apis_storagemigrationService<'_> {
        apis_storagemigration::Apis_storagemigrationService::new(self)
    }
    /// Get apis_certificates service handler
    pub fn apis_certificates(&self) -> apis_certificates::Apis_certificatesService<'_> {
        apis_certificates::Apis_certificatesService::new(self)
    }
    /// Get apis_batch service handler
    pub fn apis_batch(&self) -> apis_batch::Apis_batchService<'_> {
        apis_batch::Apis_batchService::new(self)
    }
    /// Get apis_node service handler
    pub fn apis_node(&self) -> apis_node::Apis_nodeService<'_> {
        apis_node::Apis_nodeService::new(self)
    }
    /// Get apis_resource service handler
    pub fn apis_resource(&self) -> apis_resource::Apis_resourceService<'_> {
        apis_resource::Apis_resourceService::new(self)
    }
    /// Get apis_networking service handler
    pub fn apis_networking(&self) -> apis_networking::Apis_networkingService<'_> {
        apis_networking::Apis_networkingService::new(self)
    }
    /// Get apis_apiregistration service handler
    pub fn apis_apiregistration(&self) -> apis_apiregistration::Apis_apiregistrationService<'_> {
        apis_apiregistration::Apis_apiregistrationService::new(self)
    }
    /// Get apis_internal service handler
    pub fn apis_internal(&self) -> apis_internal::Apis_internalService<'_> {
        apis_internal::Apis_internalService::new(self)
    }
    /// Get logs service handler
    pub fn logs(&self) -> logs::LogsService<'_> {
        logs::LogsService::new(self)
    }
    /// Get apis_policy service handler
    pub fn apis_policy(&self) -> apis_policy::Apis_policyService<'_> {
        apis_policy::Apis_policyService::new(self)
    }
    /// Get apis_authorization service handler
    pub fn apis_authorization(&self) -> apis_authorization::Apis_authorizationService<'_> {
        apis_authorization::Apis_authorizationService::new(self)
    }
    /// Get apis_apiextensions service handler
    pub fn apis_apiextensions(&self) -> apis_apiextensions::Apis_apiextensionsService<'_> {
        apis_apiextensions::Apis_apiextensionsService::new(self)
    }
    /// Get apis_autoscaling service handler
    pub fn apis_autoscaling(&self) -> apis_autoscaling::Apis_autoscalingService<'_> {
        apis_autoscaling::Apis_autoscalingService::new(self)
    }
    /// Get api service handler
    pub fn api(&self) -> api::ApiService<'_> {
        api::ApiService::new(self)
    }
    /// Get apis_authentication service handler
    pub fn apis_authentication(&self) -> apis_authentication::Apis_authenticationService<'_> {
        apis_authentication::Apis_authenticationService::new(self)
    }
    /// Get apis_admissionregistration service handler
    pub fn apis_admissionregistration(
        &self,
    ) -> apis_admissionregistration::Apis_admissionregistrationService<'_> {
        apis_admissionregistration::Apis_admissionregistrationService::new(self)
    }
    /// Get apis_coordination service handler
    pub fn apis_coordination(&self) -> apis_coordination::Apis_coordinationService<'_> {
        apis_coordination::Apis_coordinationService::new(self)
    }

    /// Get reference to the Tokio runtime for executing async operations
    pub(crate) fn runtime(&self) -> &tokio::runtime::Runtime {
        &self.runtime
    }
}

/// Implement ProviderExecutor trait for Hemmer integration
#[async_trait]
impl ProviderExecutor for K8sProvider {
    /// Configure the provider with authentication and settings
    async fn configure(&mut self, config: ProviderConfig) -> Result<()> {
        // Configuration is already handled in new()
        // Additional runtime configuration can be added here
        Ok(())
    }

    /// Plan changes to a resource (diff current vs desired state)
    async fn plan(
        &self,
        resource_type: &str,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // Dispatch to appropriate service based on resource_type
        // Format: "service_name.resource_name" (e.g., "s3.bucket")
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "apis_storage" => {
                self.apis_storage()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_rbac" => {
                self.apis_rbac()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_events" => {
                self.apis_events()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_flowcontrol" => {
                self.apis_flowcontrol()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "version" => {
                self.version()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "openid" => {
                self.openid()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_apps" => {
                self.apis_apps()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_scheduling" => {
                self.apis_scheduling()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis" => {
                self.apis()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_storagemigration" => {
                self.apis_storagemigration()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_certificates" => {
                self.apis_certificates()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_batch" => {
                self.apis_batch()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_node" => {
                self.apis_node()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_resource" => {
                self.apis_resource()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_networking" => {
                self.apis_networking()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_apiregistration" => {
                self.apis_apiregistration()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_internal" => {
                self.apis_internal()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "logs" => {
                self.logs()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_policy" => {
                self.apis_policy()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_authorization" => {
                self.apis_authorization()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_apiextensions" => {
                self.apis_apiextensions()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_autoscaling" => {
                self.apis_autoscaling()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "api" => {
                self.api()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_authentication" => {
                self.apis_authentication()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_admissionregistration" => {
                self.apis_admissionregistration()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            "apis_coordination" => {
                self.apis_coordination()
                    .plan_resource(resource_name, current_state, desired_input)
                    .await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Create a new resource
    async fn create(&self, resource_type: &str, input: ResourceInput) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "apis_storage" => {
                self.apis_storage()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_rbac" => self.apis_rbac().create_resource(resource_name, input).await,
            "apis_events" => {
                self.apis_events()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_flowcontrol" => {
                self.apis_flowcontrol()
                    .create_resource(resource_name, input)
                    .await
            }
            "version" => self.version().create_resource(resource_name, input).await,
            "openid" => self.openid().create_resource(resource_name, input).await,
            "apis_apps" => self.apis_apps().create_resource(resource_name, input).await,
            "apis_scheduling" => {
                self.apis_scheduling()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis" => self.apis().create_resource(resource_name, input).await,
            "apis_storagemigration" => {
                self.apis_storagemigration()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_certificates" => {
                self.apis_certificates()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_batch" => {
                self.apis_batch()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_node" => self.apis_node().create_resource(resource_name, input).await,
            "apis_resource" => {
                self.apis_resource()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_networking" => {
                self.apis_networking()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_apiregistration" => {
                self.apis_apiregistration()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_internal" => {
                self.apis_internal()
                    .create_resource(resource_name, input)
                    .await
            }
            "logs" => self.logs().create_resource(resource_name, input).await,
            "apis_policy" => {
                self.apis_policy()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_authorization" => {
                self.apis_authorization()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_apiextensions" => {
                self.apis_apiextensions()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_autoscaling" => {
                self.apis_autoscaling()
                    .create_resource(resource_name, input)
                    .await
            }
            "api" => self.api().create_resource(resource_name, input).await,
            "apis_authentication" => {
                self.apis_authentication()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_admissionregistration" => {
                self.apis_admissionregistration()
                    .create_resource(resource_name, input)
                    .await
            }
            "apis_coordination" => {
                self.apis_coordination()
                    .create_resource(resource_name, input)
                    .await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Read/refresh resource state
    async fn read(&self, resource_type: &str, id: &str) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "apis_storage" => self.apis_storage().read_resource(resource_name, id).await,
            "apis_rbac" => self.apis_rbac().read_resource(resource_name, id).await,
            "apis_events" => self.apis_events().read_resource(resource_name, id).await,
            "apis_flowcontrol" => {
                self.apis_flowcontrol()
                    .read_resource(resource_name, id)
                    .await
            }
            "version" => self.version().read_resource(resource_name, id).await,
            "openid" => self.openid().read_resource(resource_name, id).await,
            "apis_apps" => self.apis_apps().read_resource(resource_name, id).await,
            "apis_scheduling" => {
                self.apis_scheduling()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis" => self.apis().read_resource(resource_name, id).await,
            "apis_storagemigration" => {
                self.apis_storagemigration()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis_certificates" => {
                self.apis_certificates()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis_batch" => self.apis_batch().read_resource(resource_name, id).await,
            "apis_node" => self.apis_node().read_resource(resource_name, id).await,
            "apis_resource" => self.apis_resource().read_resource(resource_name, id).await,
            "apis_networking" => {
                self.apis_networking()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis_apiregistration" => {
                self.apis_apiregistration()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis_internal" => self.apis_internal().read_resource(resource_name, id).await,
            "logs" => self.logs().read_resource(resource_name, id).await,
            "apis_policy" => self.apis_policy().read_resource(resource_name, id).await,
            "apis_authorization" => {
                self.apis_authorization()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis_apiextensions" => {
                self.apis_apiextensions()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis_autoscaling" => {
                self.apis_autoscaling()
                    .read_resource(resource_name, id)
                    .await
            }
            "api" => self.api().read_resource(resource_name, id).await,
            "apis_authentication" => {
                self.apis_authentication()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis_admissionregistration" => {
                self.apis_admissionregistration()
                    .read_resource(resource_name, id)
                    .await
            }
            "apis_coordination" => {
                self.apis_coordination()
                    .read_resource(resource_name, id)
                    .await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Update an existing resource
    async fn update(
        &self,
        resource_type: &str,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "apis_storage" => {
                self.apis_storage()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_rbac" => {
                self.apis_rbac()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_events" => {
                self.apis_events()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_flowcontrol" => {
                self.apis_flowcontrol()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "version" => {
                self.version()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "openid" => {
                self.openid()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_apps" => {
                self.apis_apps()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_scheduling" => {
                self.apis_scheduling()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis" => self.apis().update_resource(resource_name, id, input).await,
            "apis_storagemigration" => {
                self.apis_storagemigration()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_certificates" => {
                self.apis_certificates()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_batch" => {
                self.apis_batch()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_node" => {
                self.apis_node()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_resource" => {
                self.apis_resource()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_networking" => {
                self.apis_networking()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_apiregistration" => {
                self.apis_apiregistration()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_internal" => {
                self.apis_internal()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "logs" => self.logs().update_resource(resource_name, id, input).await,
            "apis_policy" => {
                self.apis_policy()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_authorization" => {
                self.apis_authorization()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_apiextensions" => {
                self.apis_apiextensions()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_autoscaling" => {
                self.apis_autoscaling()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "api" => self.api().update_resource(resource_name, id, input).await,
            "apis_authentication" => {
                self.apis_authentication()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_admissionregistration" => {
                self.apis_admissionregistration()
                    .update_resource(resource_name, id, input)
                    .await
            }
            "apis_coordination" => {
                self.apis_coordination()
                    .update_resource(resource_name, id, input)
                    .await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }

    /// Delete a resource
    async fn delete(&self, resource_type: &str, id: &str) -> Result<()> {
        let parts: Vec<&str> = resource_type.split('.').collect();
        if parts.len() != 2 {
            return Err(hemmer_core::HemmerError::Provider(format!(
                "Invalid resource type format: {}. Expected 'service.resource'",
                resource_type
            )));
        }

        let service_name = parts[0];
        let resource_name = parts[1];

        match service_name {
            "apis_storage" => self.apis_storage().delete_resource(resource_name, id).await,
            "apis_rbac" => self.apis_rbac().delete_resource(resource_name, id).await,
            "apis_events" => self.apis_events().delete_resource(resource_name, id).await,
            "apis_flowcontrol" => {
                self.apis_flowcontrol()
                    .delete_resource(resource_name, id)
                    .await
            }
            "version" => self.version().delete_resource(resource_name, id).await,
            "openid" => self.openid().delete_resource(resource_name, id).await,
            "apis_apps" => self.apis_apps().delete_resource(resource_name, id).await,
            "apis_scheduling" => {
                self.apis_scheduling()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis" => self.apis().delete_resource(resource_name, id).await,
            "apis_storagemigration" => {
                self.apis_storagemigration()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_certificates" => {
                self.apis_certificates()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_batch" => self.apis_batch().delete_resource(resource_name, id).await,
            "apis_node" => self.apis_node().delete_resource(resource_name, id).await,
            "apis_resource" => {
                self.apis_resource()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_networking" => {
                self.apis_networking()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_apiregistration" => {
                self.apis_apiregistration()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_internal" => {
                self.apis_internal()
                    .delete_resource(resource_name, id)
                    .await
            }
            "logs" => self.logs().delete_resource(resource_name, id).await,
            "apis_policy" => self.apis_policy().delete_resource(resource_name, id).await,
            "apis_authorization" => {
                self.apis_authorization()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_apiextensions" => {
                self.apis_apiextensions()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_autoscaling" => {
                self.apis_autoscaling()
                    .delete_resource(resource_name, id)
                    .await
            }
            "api" => self.api().delete_resource(resource_name, id).await,
            "apis_authentication" => {
                self.apis_authentication()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_admissionregistration" => {
                self.apis_admissionregistration()
                    .delete_resource(resource_name, id)
                    .await
            }
            "apis_coordination" => {
                self.apis_coordination()
                    .delete_resource(resource_name, id)
                    .await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown service: {}",
                service_name
            ))),
        }
    }
}

/// Factory function to create a provider instance
///
/// This is the entry point called by Hemmer when loading the provider as a dynamic library.
#[no_mangle]
pub extern "C" fn create_provider() -> *mut dyn ProviderExecutor {
    match K8sProvider::new() {
        Ok(provider) => Box::into_raw(Box::new(provider)) as *mut dyn ProviderExecutor,
        Err(_) => std::ptr::null_mut(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        // Provider creation test
        // Note: This will fail without proper credentials
        // let provider = K8sProvider::new();
        // assert!(provider.is_ok());
    }
}
