# Kubernetes Provider for Hemmer

Auto-generated unified provider for kubernetes using SDK version v1.

This provider includes **63 services** with a total of **185 resources**.

## Services

### Apis_internal (1 resources)

- **Internal** [R] - get information of a group

### Apis_events (2 resources)

- **V1** [R] - get available resources
- **Event** [CRUD] - create an Event

### Apis_storage (1 resources)

- **Storage** [R] - get information of a group

### Apis_resource (2 resources)

- **V1alpha3** [R] - get available resources
- **Devicetaintrule** [CRUD] - create a DeviceTaintRule

###  (1 resources)

- **Openid_configuration** [R] - get service account issuer OpenID configuration, also known as the 'OIDC discovery doc'

### Logs (1 resources)

- **Log** [R] - 

### Apis_storagemigration (3 resources)

- **Storageversionmigration** [CRUD] - create a StorageVersionMigration
- **Statu** [RU] - read status of the specified StorageVersionMigration
- **V1alpha1** [R] - get available resources

### Apis_storage (8 resources)

- **Volumeattributesclasse** [CRUD] - create a VolumeAttributesClass
- **Csistoragecapacitie** [CRUD] - create a CSIStorageCapacity
- **Csinode** [CRUD] - create a CSINode
- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified VolumeAttachment
- **Volumeattachment** [CRUD] - create a VolumeAttachment
- **Storageclasse** [CRUD] - create a StorageClass
- **Csidriver** [CRUD] - create a CSIDriver

### Apis (1 resources)

- **Api** [R] - get available API versions

### Apis_resource (6 resources)

- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Deviceclasse** [CRUD] - create a DeviceClass
- **V1beta1** [R] - get available resources
- **Resourceslice** [CRUD] - create a ResourceSlice
- **Resourceclaim** [CRUD] - create a ResourceClaim
- **Statu** [RU] - read status of the specified ResourceClaim

### Apis_admissionregistration (1 resources)

- **Admissionregistration** [R] - get information of a group

### Apis (1 resources)

- **Discovery** [R] - get information of a group

### Apis_apiextensions (3 resources)

- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified CustomResourceDefinition
- **Customresourcedefinition** [CRUD] - create a CustomResourceDefinition

### Apis_coordination (1 resources)

- **Coordination** [R] - get information of a group

### Apis_certificates (4 resources)

- **V1alpha1** [R] - get available resources
- **Statu** [RU] - read status of the specified PodCertificateRequest
- **Podcertificaterequest** [CRUD] - create a PodCertificateRequest
- **Clustertrustbundle** [CRUD] - create a ClusterTrustBundle

### Apis_coordination (2 resources)

- **Leasecandidate** [CRUD] - create a LeaseCandidate
- **V1alpha2** [R] - get available resources

### Apis_rbac (1 resources)

- **Rbac.authorization** [R] - get information of a group

### Apis_flowcontrol (1 resources)

- **Flowcontrol** [R] - get information of a group

### Apis_node (2 resources)

- **V1** [R] - get available resources
- **Runtimeclasse** [CRUD] - create a RuntimeClass

### Apis_rbac (5 resources)

- **V1** [R] - get available resources
- **Role** [CRUD] - create a Role
- **Clusterrole** [CRUD] - create a ClusterRole
- **Clusterrolebinding** [CRUD] - create a ClusterRoleBinding
- **Rolebinding** [CRUD] - create a RoleBinding

### Apis_coordination (2 resources)

- **V1** [R] - get available resources
- **Lease** [CRUD] - create a Lease

### Apis_storagemigration (1 resources)

- **Storagemigration** [R] - get information of a group

### Apis_authorization (1 resources)

- **Authorization** [R] - get information of a group

### Apis_autoscaling (1 resources)

- **Autoscaling** [R] - get information of a group

### Apis_admissionregistration (3 resources)

- **V1beta1** [R] - get available resources
- **Mutatingadmissionpolicie** [CRUD] - create a MutatingAdmissionPolicy
- **Mutatingadmissionpolicybinding** [CRUD] - create a MutatingAdmissionPolicyBinding

### Version (1 resources)

- **Version** [R] - get the version information for this server

### Apis_apps (8 resources)

- **V1** [R] - get available resources
- **Scale** [RU] - read scale of the specified Deployment
- **Daemonset** [CRUD] - create a DaemonSet
- **Statu** [RU] - read status of the specified Deployment
- **Statefulset** [CRUD] - create a StatefulSet
- **Replicaset** [CRUD] - create a ReplicaSet
- **Controllerrevision** [CRUD] - create a ControllerRevision
- **Deployment** [CRUD] - create a Deployment

### Apis_autoscaling (3 resources)

- **Horizontalpodautoscaler** [CRUD] - create a HorizontalPodAutoscaler
- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified HorizontalPodAutoscaler

### Apis_certificates (1 resources)

- **Certificate** [R] - get information of a group

### Api (30 resources)

- **Resourcequota** [CRUD] - create a ResourceQuota
- **Attach** [CR] - connect POST requests to attach of Pod
- **Limitrange** [CRUD] - create a LimitRange
- **Event** [CRUD] - create an Event
- **Service** [CRUD] - create a Service
- **Portforward** [CR] - connect POST requests to portforward of Pod
- **Exec** [CR] - connect POST requests to exec of Pod
- **Log** [R] - read log of the specified Pod
- **Scale** [RU] - read scale of the specified ReplicationController
- **Serviceaccount** [CRUD] - create a ServiceAccount
- **Statu** [RU] - read status of the specified Service
- **Node** [CRUD] - create a Node
- **Binding** [C] - create a Binding
- **Componentstatuse** [R] - list objects of kind ComponentStatus
- **Resize** [RU] - read resize of the specified Pod
- **Finalize** [U] - 
- **Replicationcontroller** [CRUD] - create a ReplicationController
- **Configmap** [CRUD] - create a ConfigMap
- **Ephemeralcontainer** [RU] - read ephemeralcontainers of the specified Pod
- **Podtemplate** [CRUD] - create a PodTemplate
- **Namespace** [CRUD] - create a Namespace
- **Endpoint** [CRUD] - create Endpoints
- **V1** [R] - get available resources
- **Token** [C] - create token of a ServiceAccount
- **Secret** [CRUD] - create a Secret
- **Persistentvolume** [CRUD] - create a PersistentVolume
- **Proxy** [CRUD] - connect POST requests to proxy of Node
- **Pod** [CRUD] - create a Pod
- **Persistentvolumeclaim** [CRUD] - create a PersistentVolumeClaim
- **Eviction** [C] - create eviction of a Pod

### Apis_policy (3 resources)

- **V1** [R] - get available resources
- **Poddisruptionbudget** [CRUD] - create a PodDisruptionBudget
- **Statu** [RU] - read status of the specified PodDisruptionBudget

### Apis_scheduling (2 resources)

- **Priorityclasse** [CRUD] - create a PriorityClass
- **V1** [R] - get available resources

### Apis_authentication (1 resources)

- **Authentication** [R] - get information of a group

### Apis_admissionregistration (3 resources)

- **Mutatingadmissionpolicybinding** [CRUD] - create a MutatingAdmissionPolicyBinding
- **Mutatingadmissionpolicie** [CRUD] - create a MutatingAdmissionPolicy
- **V1alpha1** [R] - get available resources

### Apis_batch (1 resources)

- **Batch** [R] - get information of a group

### Apis_authorization (5 resources)

- **Localsubjectaccessreview** [C] - create a LocalSubjectAccessReview
- **Selfsubjectaccessreview** [C] - create a SelfSubjectAccessReview
- **Selfsubjectrulesreview** [C] - create a SelfSubjectRulesReview
- **Subjectaccessreview** [C] - create a SubjectAccessReview
- **V1** [R] - get available resources

### Apis_apiextensions (1 resources)

- **Apiextension** [R] - get information of a group

### Apis_node (1 resources)

- **Node** [R] - get information of a group

### Apis_networking (7 resources)

- **Ingressclasse** [CRUD] - create an IngressClass
- **Statu** [RU] - read status of the specified Ingress
- **Ingresse** [CRUD] - create an Ingress
- **Ipaddresse** [CRUD] - create an IPAddress
- **Servicecidr** [CRUD] - create a ServiceCIDR
- **Networkpolicie** [CRUD] - create a NetworkPolicy
- **V1** [R] - get available resources

### Apis_apiregistration (3 resources)

- **Statu** [RU] - read status of the specified APIService
- **Apiservice** [CRUD] - create an APIService
- **V1** [R] - get available resources

### Apis_networking (4 resources)

- **Statu** [RU] - read status of the specified ServiceCIDR
- **Ipaddresse** [CRUD] - create an IPAddress
- **Servicecidr** [CRUD] - create a ServiceCIDR
- **V1beta1** [R] - get available resources

### Apis_certificates (4 resources)

- **Certificatesigningrequest** [CRUD] - create a CertificateSigningRequest
- **Approval** [RU] - read approval of the specified CertificateSigningRequest
- **Statu** [RU] - read status of the specified CertificateSigningRequest
- **V1** [R] - get available resources

### Apis_coordination (2 resources)

- **Leasecandidate** [CRUD] - create a LeaseCandidate
- **V1beta1** [R] - get available resources

### Apis_autoscaling (3 resources)

- **Horizontalpodautoscaler** [CRUD] - create a HorizontalPodAutoscaler
- **V2** [R] - get available resources
- **Statu** [RU] - read status of the specified HorizontalPodAutoscaler

### Apis_networking (1 resources)

- **Networking** [R] - get information of a group

### Apis_authentication (3 resources)

- **V1** [R] - get available resources
- **Selfsubjectreview** [C] - create a SelfSubjectReview
- **Tokenreview** [C] - create a TokenReview

### Apis_policy (1 resources)

- **Policy** [R] - get information of a group

### Apis_internal (3 resources)

- **V1alpha1** [R] - get available resources
- **Storageversion** [CRUD] - create a StorageVersion
- **Statu** [RU] - read status of the specified StorageVersion

### Apis_apps (1 resources)

- **App** [R] - get information of a group

### Api (1 resources)

- **Api** [R] - get available API versions

### Apis_storage (2 resources)

- **Volumeattributesclasse** [CRUD] - create a VolumeAttributesClass
- **V1beta1** [R] - get available resources

### Apis_certificates (2 resources)

- **V1beta1** [R] - get available resources
- **Clustertrustbundle** [CRUD] - create a ClusterTrustBundle

### Apis_events (1 resources)

- **Event** [R] - get information of a group

### Apis_flowcontrol (4 resources)

- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified FlowSchema
- **Flowschema** [CRUD] - create a FlowSchema
- **Prioritylevelconfiguration** [CRUD] - create a PriorityLevelConfiguration

### Apis_scheduling (1 resources)

- **Scheduling** [R] - get information of a group

### Apis (2 resources)

- **V1** [R] - get available resources
- **Endpointslice** [CRUD] - create an EndpointSlice

### Apis_resource (6 resources)

- **Deviceclasse** [CRUD] - create a DeviceClass
- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Resourceslice** [CRUD] - create a ResourceSlice
- **Statu** [RU] - read status of the specified ResourceClaim
- **Resourceclaim** [CRUD] - create a ResourceClaim
- **V1beta2** [R] - get available resources

### Apis_resource (1 resources)

- **Resource** [R] - get information of a group

### Apis_apiregistration (1 resources)

- **Apiregistration** [R] - get information of a group

### Apis_batch (4 resources)

- **Cronjob** [CRUD] - create a CronJob
- **Job** [CRUD] - create a Job
- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified CronJob

### Apis_admissionregistration (6 resources)

- **Validatingadmissionpolicybinding** [CRUD] - create a ValidatingAdmissionPolicyBinding
- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified ValidatingAdmissionPolicy
- **Validatingadmissionpolicie** [CRUD] - create a ValidatingAdmissionPolicy
- **Mutatingwebhookconfiguration** [CRUD] - create a MutatingWebhookConfiguration
- **Validatingwebhookconfiguration** [CRUD] - create a ValidatingWebhookConfiguration

### Apis_resource (6 resources)

- **Resourceslice** [CRUD] - create a ResourceSlice
- **Deviceclasse** [CRUD] - create a DeviceClass
- **Statu** [RU] - read status of the specified ResourceClaim
- **V1** [R] - get available resources
- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Resourceclaim** [CRUD] - create a ResourceClaim

### Openid (1 resources)

- **Jwk** [R] - get service account issuer OpenID JSON Web Key Set (contains public token verification keys)



## Usage

```rust
use hemmer_kubernetes_provider::KubernetesProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create unified provider
    let provider = KubernetesProvider::new().await?;

    // Access services
    let apis_internal_service = provider.apis_internal();
    let apis_events_service = provider.apis_events();
    let apis_storage_service = provider.apis_storage();
    let apis_resource_service = provider.apis_resource();
    let _service = provider.();
    let logs_service = provider.logs();
    let apis_storagemigration_service = provider.apis_storagemigration();
    let apis_storage_service = provider.apis_storage();
    let apis_service = provider.apis();
    let apis_resource_service = provider.apis_resource();
    let apis_admissionregistration_service = provider.apis_admissionregistration();
    let apis_service = provider.apis();
    let apis_apiextensions_service = provider.apis_apiextensions();
    let apis_coordination_service = provider.apis_coordination();
    let apis_certificates_service = provider.apis_certificates();
    let apis_coordination_service = provider.apis_coordination();
    let apis_rbac_service = provider.apis_rbac();
    let apis_flowcontrol_service = provider.apis_flowcontrol();
    let apis_node_service = provider.apis_node();
    let apis_rbac_service = provider.apis_rbac();
    let apis_coordination_service = provider.apis_coordination();
    let apis_storagemigration_service = provider.apis_storagemigration();
    let apis_authorization_service = provider.apis_authorization();
    let apis_autoscaling_service = provider.apis_autoscaling();
    let apis_admissionregistration_service = provider.apis_admissionregistration();
    let version_service = provider.version();
    let apis_apps_service = provider.apis_apps();
    let apis_autoscaling_service = provider.apis_autoscaling();
    let apis_certificates_service = provider.apis_certificates();
    let api_service = provider.api();
    let apis_policy_service = provider.apis_policy();
    let apis_scheduling_service = provider.apis_scheduling();
    let apis_authentication_service = provider.apis_authentication();
    let apis_admissionregistration_service = provider.apis_admissionregistration();
    let apis_batch_service = provider.apis_batch();
    let apis_authorization_service = provider.apis_authorization();
    let apis_apiextensions_service = provider.apis_apiextensions();
    let apis_node_service = provider.apis_node();
    let apis_networking_service = provider.apis_networking();
    let apis_apiregistration_service = provider.apis_apiregistration();
    let apis_networking_service = provider.apis_networking();
    let apis_certificates_service = provider.apis_certificates();
    let apis_coordination_service = provider.apis_coordination();
    let apis_autoscaling_service = provider.apis_autoscaling();
    let apis_networking_service = provider.apis_networking();
    let apis_authentication_service = provider.apis_authentication();
    let apis_policy_service = provider.apis_policy();
    let apis_internal_service = provider.apis_internal();
    let apis_apps_service = provider.apis_apps();
    let api_service = provider.api();
    let apis_storage_service = provider.apis_storage();
    let apis_certificates_service = provider.apis_certificates();
    let apis_events_service = provider.apis_events();
    let apis_flowcontrol_service = provider.apis_flowcontrol();
    let apis_scheduling_service = provider.apis_scheduling();
    let apis_service = provider.apis();
    let apis_resource_service = provider.apis_resource();
    let apis_resource_service = provider.apis_resource();
    let apis_apiregistration_service = provider.apis_apiregistration();
    let apis_batch_service = provider.apis_batch();
    let apis_admissionregistration_service = provider.apis_admissionregistration();
    let apis_resource_service = provider.apis_resource();
    let openid_service = provider.openid();

    // Use resources from any service
    let internal = apis_internal_service.internal();
    let v1 = apis_events_service.v1();
    let event = apis_events_service.event();
    let storage = apis_storage_service.storage();
    let v1alpha3 = apis_resource_service.v1alpha3();
    let devicetaintrule = apis_resource_service.devicetaintrule();
    let openid_configuration = _service.openid_configuration();
    let log = logs_service.log();
    let storageversionmigration = apis_storagemigration_service.storageversionmigration();
    let statu = apis_storagemigration_service.statu();
    let v1alpha1 = apis_storagemigration_service.v1alpha1();
    let volumeattributesclasse = apis_storage_service.volumeattributesclasse();
    let csistoragecapacitie = apis_storage_service.csistoragecapacitie();
    let csinode = apis_storage_service.csinode();
    let v1 = apis_storage_service.v1();
    let statu = apis_storage_service.statu();
    let volumeattachment = apis_storage_service.volumeattachment();
    let storageclasse = apis_storage_service.storageclasse();
    let csidriver = apis_storage_service.csidriver();
    let api = apis_service.api();
    let resourceclaimtemplate = apis_resource_service.resourceclaimtemplate();
    let deviceclasse = apis_resource_service.deviceclasse();
    let v1beta1 = apis_resource_service.v1beta1();
    let resourceslice = apis_resource_service.resourceslice();
    let resourceclaim = apis_resource_service.resourceclaim();
    let statu = apis_resource_service.statu();
    let admissionregistration = apis_admissionregistration_service.admissionregistration();
    let discovery = apis_service.discovery();
    let v1 = apis_apiextensions_service.v1();
    let statu = apis_apiextensions_service.statu();
    let customresourcedefinition = apis_apiextensions_service.customresourcedefinition();
    let coordination = apis_coordination_service.coordination();
    let v1alpha1 = apis_certificates_service.v1alpha1();
    let statu = apis_certificates_service.statu();
    let podcertificaterequest = apis_certificates_service.podcertificaterequest();
    let clustertrustbundle = apis_certificates_service.clustertrustbundle();
    let leasecandidate = apis_coordination_service.leasecandidate();
    let v1alpha2 = apis_coordination_service.v1alpha2();
    let rbac.authorization = apis_rbac_service.rbac.authorization();
    let flowcontrol = apis_flowcontrol_service.flowcontrol();
    let v1 = apis_node_service.v1();
    let runtimeclasse = apis_node_service.runtimeclasse();
    let v1 = apis_rbac_service.v1();
    let role = apis_rbac_service.role();
    let clusterrole = apis_rbac_service.clusterrole();
    let clusterrolebinding = apis_rbac_service.clusterrolebinding();
    let rolebinding = apis_rbac_service.rolebinding();
    let v1 = apis_coordination_service.v1();
    let lease = apis_coordination_service.lease();
    let storagemigration = apis_storagemigration_service.storagemigration();
    let authorization = apis_authorization_service.authorization();
    let autoscaling = apis_autoscaling_service.autoscaling();
    let v1beta1 = apis_admissionregistration_service.v1beta1();
    let mutatingadmissionpolicie = apis_admissionregistration_service.mutatingadmissionpolicie();
    let mutatingadmissionpolicybinding = apis_admissionregistration_service.mutatingadmissionpolicybinding();
    let version = version_service.version();
    let v1 = apis_apps_service.v1();
    let scale = apis_apps_service.scale();
    let daemonset = apis_apps_service.daemonset();
    let statu = apis_apps_service.statu();
    let statefulset = apis_apps_service.statefulset();
    let replicaset = apis_apps_service.replicaset();
    let controllerrevision = apis_apps_service.controllerrevision();
    let deployment = apis_apps_service.deployment();
    let horizontalpodautoscaler = apis_autoscaling_service.horizontalpodautoscaler();
    let v1 = apis_autoscaling_service.v1();
    let statu = apis_autoscaling_service.statu();
    let certificate = apis_certificates_service.certificate();
    let resourcequota = api_service.resourcequota();
    let attach = api_service.attach();
    let limitrange = api_service.limitrange();
    let event = api_service.event();
    let service = api_service.service();
    let portforward = api_service.portforward();
    let exec = api_service.exec();
    let log = api_service.log();
    let scale = api_service.scale();
    let serviceaccount = api_service.serviceaccount();
    let statu = api_service.statu();
    let node = api_service.node();
    let binding = api_service.binding();
    let componentstatuse = api_service.componentstatuse();
    let resize = api_service.resize();
    let finalize = api_service.finalize();
    let replicationcontroller = api_service.replicationcontroller();
    let configmap = api_service.configmap();
    let ephemeralcontainer = api_service.ephemeralcontainer();
    let podtemplate = api_service.podtemplate();
    let namespace = api_service.namespace();
    let endpoint = api_service.endpoint();
    let v1 = api_service.v1();
    let token = api_service.token();
    let secret = api_service.secret();
    let persistentvolume = api_service.persistentvolume();
    let proxy = api_service.proxy();
    let pod = api_service.pod();
    let persistentvolumeclaim = api_service.persistentvolumeclaim();
    let eviction = api_service.eviction();
    let v1 = apis_policy_service.v1();
    let poddisruptionbudget = apis_policy_service.poddisruptionbudget();
    let statu = apis_policy_service.statu();
    let priorityclasse = apis_scheduling_service.priorityclasse();
    let v1 = apis_scheduling_service.v1();
    let authentication = apis_authentication_service.authentication();
    let mutatingadmissionpolicybinding = apis_admissionregistration_service.mutatingadmissionpolicybinding();
    let mutatingadmissionpolicie = apis_admissionregistration_service.mutatingadmissionpolicie();
    let v1alpha1 = apis_admissionregistration_service.v1alpha1();
    let batch = apis_batch_service.batch();
    let localsubjectaccessreview = apis_authorization_service.localsubjectaccessreview();
    let selfsubjectaccessreview = apis_authorization_service.selfsubjectaccessreview();
    let selfsubjectrulesreview = apis_authorization_service.selfsubjectrulesreview();
    let subjectaccessreview = apis_authorization_service.subjectaccessreview();
    let v1 = apis_authorization_service.v1();
    let apiextension = apis_apiextensions_service.apiextension();
    let node = apis_node_service.node();
    let ingressclasse = apis_networking_service.ingressclasse();
    let statu = apis_networking_service.statu();
    let ingresse = apis_networking_service.ingresse();
    let ipaddresse = apis_networking_service.ipaddresse();
    let servicecidr = apis_networking_service.servicecidr();
    let networkpolicie = apis_networking_service.networkpolicie();
    let v1 = apis_networking_service.v1();
    let statu = apis_apiregistration_service.statu();
    let apiservice = apis_apiregistration_service.apiservice();
    let v1 = apis_apiregistration_service.v1();
    let statu = apis_networking_service.statu();
    let ipaddresse = apis_networking_service.ipaddresse();
    let servicecidr = apis_networking_service.servicecidr();
    let v1beta1 = apis_networking_service.v1beta1();
    let certificatesigningrequest = apis_certificates_service.certificatesigningrequest();
    let approval = apis_certificates_service.approval();
    let statu = apis_certificates_service.statu();
    let v1 = apis_certificates_service.v1();
    let leasecandidate = apis_coordination_service.leasecandidate();
    let v1beta1 = apis_coordination_service.v1beta1();
    let horizontalpodautoscaler = apis_autoscaling_service.horizontalpodautoscaler();
    let v2 = apis_autoscaling_service.v2();
    let statu = apis_autoscaling_service.statu();
    let networking = apis_networking_service.networking();
    let v1 = apis_authentication_service.v1();
    let selfsubjectreview = apis_authentication_service.selfsubjectreview();
    let tokenreview = apis_authentication_service.tokenreview();
    let policy = apis_policy_service.policy();
    let v1alpha1 = apis_internal_service.v1alpha1();
    let storageversion = apis_internal_service.storageversion();
    let statu = apis_internal_service.statu();
    let app = apis_apps_service.app();
    let api = api_service.api();
    let volumeattributesclasse = apis_storage_service.volumeattributesclasse();
    let v1beta1 = apis_storage_service.v1beta1();
    let v1beta1 = apis_certificates_service.v1beta1();
    let clustertrustbundle = apis_certificates_service.clustertrustbundle();
    let event = apis_events_service.event();
    let v1 = apis_flowcontrol_service.v1();
    let statu = apis_flowcontrol_service.statu();
    let flowschema = apis_flowcontrol_service.flowschema();
    let prioritylevelconfiguration = apis_flowcontrol_service.prioritylevelconfiguration();
    let scheduling = apis_scheduling_service.scheduling();
    let v1 = apis_service.v1();
    let endpointslice = apis_service.endpointslice();
    let deviceclasse = apis_resource_service.deviceclasse();
    let resourceclaimtemplate = apis_resource_service.resourceclaimtemplate();
    let resourceslice = apis_resource_service.resourceslice();
    let statu = apis_resource_service.statu();
    let resourceclaim = apis_resource_service.resourceclaim();
    let v1beta2 = apis_resource_service.v1beta2();
    let resource = apis_resource_service.resource();
    let apiregistration = apis_apiregistration_service.apiregistration();
    let cronjob = apis_batch_service.cronjob();
    let job = apis_batch_service.job();
    let v1 = apis_batch_service.v1();
    let statu = apis_batch_service.statu();
    let validatingadmissionpolicybinding = apis_admissionregistration_service.validatingadmissionpolicybinding();
    let v1 = apis_admissionregistration_service.v1();
    let statu = apis_admissionregistration_service.statu();
    let validatingadmissionpolicie = apis_admissionregistration_service.validatingadmissionpolicie();
    let mutatingwebhookconfiguration = apis_admissionregistration_service.mutatingwebhookconfiguration();
    let validatingwebhookconfiguration = apis_admissionregistration_service.validatingwebhookconfiguration();
    let resourceslice = apis_resource_service.resourceslice();
    let deviceclasse = apis_resource_service.deviceclasse();
    let statu = apis_resource_service.statu();
    let v1 = apis_resource_service.v1();
    let resourceclaimtemplate = apis_resource_service.resourceclaimtemplate();
    let resourceclaim = apis_resource_service.resourceclaim();
    let jwk = openid_service.jwk();

    Ok(())
}
```

## Service Details

### Apis_internal

**Resources:**

#### Internal

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_events

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Event

create an Event

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_storage

**Resources:**

#### Storage

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_resource

**Resources:**

#### V1alpha3

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Devicetaintrule

create a DeviceTaintRule

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### 

**Resources:**

#### Openid_configuration

get service account issuer OpenID configuration, also known as the 'OIDC discovery doc'

**Operations:**
- ✅ Read


**Fields:**



### Logs

**Resources:**

#### Log



**Operations:**
- ✅ Read


**Fields:**



### Apis_storagemigration

**Resources:**

#### Storageversionmigration

create a StorageVersionMigration

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified StorageVersionMigration

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1alpha1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_storage

**Resources:**

#### Volumeattributesclasse

create a VolumeAttributesClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Csistoragecapacitie

create a CSIStorageCapacity

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Csinode

create a CSINode

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified VolumeAttachment

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Volumeattachment

create a VolumeAttachment

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Storageclasse

create a StorageClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Csidriver

create a CSIDriver

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis

**Resources:**

#### Api

get available API versions

**Operations:**
- ✅ Read


**Fields:**



### Apis_resource

**Resources:**

#### Resourceclaimtemplate

create a ResourceClaimTemplate

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Deviceclasse

create a DeviceClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Resourceslice

create a ResourceSlice

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Resourceclaim

create a ResourceClaim

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified ResourceClaim

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_admissionregistration

**Resources:**

#### Admissionregistration

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis

**Resources:**

#### Discovery

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_apiextensions

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified CustomResourceDefinition

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Customresourcedefinition

create a CustomResourceDefinition

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_coordination

**Resources:**

#### Coordination

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_certificates

**Resources:**

#### V1alpha1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified PodCertificateRequest

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Podcertificaterequest

create a PodCertificateRequest

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Clustertrustbundle

create a ClusterTrustBundle

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_coordination

**Resources:**

#### Leasecandidate

create a LeaseCandidate

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1alpha2

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_rbac

**Resources:**

#### Rbac.authorization

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_flowcontrol

**Resources:**

#### Flowcontrol

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_node

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Runtimeclasse

create a RuntimeClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_rbac

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Role

create a Role

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Clusterrole

create a ClusterRole

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Clusterrolebinding

create a ClusterRoleBinding

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Rolebinding

create a RoleBinding

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_coordination

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Lease

create a Lease

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_storagemigration

**Resources:**

#### Storagemigration

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_authorization

**Resources:**

#### Authorization

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_autoscaling

**Resources:**

#### Autoscaling

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_admissionregistration

**Resources:**

#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Mutatingadmissionpolicie

create a MutatingAdmissionPolicy

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Mutatingadmissionpolicybinding

create a MutatingAdmissionPolicyBinding

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Version

**Resources:**

#### Version

get the version information for this server

**Operations:**
- ✅ Read


**Fields:**



### Apis_apps

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Scale

read scale of the specified Deployment

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Daemonset

create a DaemonSet

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified Deployment

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statefulset

create a StatefulSet

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Replicaset

create a ReplicaSet

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Controllerrevision

create a ControllerRevision

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Deployment

create a Deployment

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_autoscaling

**Resources:**

#### Horizontalpodautoscaler

create a HorizontalPodAutoscaler

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified HorizontalPodAutoscaler

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_certificates

**Resources:**

#### Certificate

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Api

**Resources:**

#### Resourcequota

create a ResourceQuota

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Attach

connect POST requests to attach of Pod

**Operations:**
- ✅ Create
- ✅ Read


**Fields:**


#### Limitrange

create a LimitRange

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Event

create an Event

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Service

create a Service

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Portforward

connect POST requests to portforward of Pod

**Operations:**
- ✅ Create
- ✅ Read


**Fields:**


#### Exec

connect POST requests to exec of Pod

**Operations:**
- ✅ Create
- ✅ Read


**Fields:**


#### Log

read log of the specified Pod

**Operations:**
- ✅ Read


**Fields:**


#### Scale

read scale of the specified ReplicationController

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Serviceaccount

create a ServiceAccount

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified Service

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Node

create a Node

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Binding

create a Binding

**Operations:**
- ✅ Create


**Fields:**


#### Componentstatuse

list objects of kind ComponentStatus

**Operations:**
- ✅ Read


**Fields:**


#### Resize

read resize of the specified Pod

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Finalize



**Operations:**
- ✅ Update


**Fields:**


#### Replicationcontroller

create a ReplicationController

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Configmap

create a ConfigMap

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Ephemeralcontainer

read ephemeralcontainers of the specified Pod

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Podtemplate

create a PodTemplate

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Namespace

create a Namespace

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Endpoint

create Endpoints

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Token

create token of a ServiceAccount

**Operations:**
- ✅ Create


**Fields:**


#### Secret

create a Secret

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Persistentvolume

create a PersistentVolume

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Proxy

connect POST requests to proxy of Node

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**


#### Pod

create a Pod

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Persistentvolumeclaim

create a PersistentVolumeClaim

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Eviction

create eviction of a Pod

**Operations:**
- ✅ Create


**Fields:**



### Apis_policy

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Poddisruptionbudget

create a PodDisruptionBudget

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified PodDisruptionBudget

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_scheduling

**Resources:**

#### Priorityclasse

create a PriorityClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_authentication

**Resources:**

#### Authentication

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_admissionregistration

**Resources:**

#### Mutatingadmissionpolicybinding

create a MutatingAdmissionPolicyBinding

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Mutatingadmissionpolicie

create a MutatingAdmissionPolicy

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1alpha1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_batch

**Resources:**

#### Batch

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_authorization

**Resources:**

#### Localsubjectaccessreview

create a LocalSubjectAccessReview

**Operations:**
- ✅ Create


**Fields:**


#### Selfsubjectaccessreview

create a SelfSubjectAccessReview

**Operations:**
- ✅ Create


**Fields:**


#### Selfsubjectrulesreview

create a SelfSubjectRulesReview

**Operations:**
- ✅ Create


**Fields:**


#### Subjectaccessreview

create a SubjectAccessReview

**Operations:**
- ✅ Create


**Fields:**


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_apiextensions

**Resources:**

#### Apiextension

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_node

**Resources:**

#### Node

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_networking

**Resources:**

#### Ingressclasse

create an IngressClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified Ingress

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Ingresse

create an Ingress

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Ipaddresse

create an IPAddress

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Servicecidr

create a ServiceCIDR

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Networkpolicie

create a NetworkPolicy

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_apiregistration

**Resources:**

#### Statu

read status of the specified APIService

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Apiservice

create an APIService

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_networking

**Resources:**

#### Statu

read status of the specified ServiceCIDR

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Ipaddresse

create an IPAddress

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Servicecidr

create a ServiceCIDR

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_certificates

**Resources:**

#### Certificatesigningrequest

create a CertificateSigningRequest

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Approval

read approval of the specified CertificateSigningRequest

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified CertificateSigningRequest

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_coordination

**Resources:**

#### Leasecandidate

create a LeaseCandidate

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_autoscaling

**Resources:**

#### Horizontalpodautoscaler

create a HorizontalPodAutoscaler

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V2

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified HorizontalPodAutoscaler

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_networking

**Resources:**

#### Networking

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_authentication

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Selfsubjectreview

create a SelfSubjectReview

**Operations:**
- ✅ Create


**Fields:**


#### Tokenreview

create a TokenReview

**Operations:**
- ✅ Create


**Fields:**



### Apis_policy

**Resources:**

#### Policy

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_internal

**Resources:**

#### V1alpha1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Storageversion

create a StorageVersion

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified StorageVersion

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_apps

**Resources:**

#### App

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Api

**Resources:**

#### Api

get available API versions

**Operations:**
- ✅ Read


**Fields:**



### Apis_storage

**Resources:**

#### Volumeattributesclasse

create a VolumeAttributesClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_certificates

**Resources:**

#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Clustertrustbundle

create a ClusterTrustBundle

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_events

**Resources:**

#### Event

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_flowcontrol

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified FlowSchema

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Flowschema

create a FlowSchema

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Prioritylevelconfiguration

create a PriorityLevelConfiguration

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_scheduling

**Resources:**

#### Scheduling

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Endpointslice

create an EndpointSlice

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_resource

**Resources:**

#### Deviceclasse

create a DeviceClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Resourceclaimtemplate

create a ResourceClaimTemplate

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Resourceslice

create a ResourceSlice

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified ResourceClaim

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Resourceclaim

create a ResourceClaim

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1beta2

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_resource

**Resources:**

#### Resource

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_apiregistration

**Resources:**

#### Apiregistration

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_batch

**Resources:**

#### Cronjob

create a CronJob

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Job

create a Job

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified CronJob

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_admissionregistration

**Resources:**

#### Validatingadmissionpolicybinding

create a ValidatingAdmissionPolicyBinding

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified ValidatingAdmissionPolicy

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Validatingadmissionpolicie

create a ValidatingAdmissionPolicy

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Mutatingwebhookconfiguration

create a MutatingWebhookConfiguration

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Validatingwebhookconfiguration

create a ValidatingWebhookConfiguration

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Apis_resource

**Resources:**

#### Resourceslice

create a ResourceSlice

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Deviceclasse

create a DeviceClass

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified ResourceClaim

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Resourceclaimtemplate

create a ResourceClaimTemplate

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Resourceclaim

create a ResourceClaim

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Openid

**Resources:**

#### Jwk

get service account issuer OpenID JSON Web Key Set (contains public token verification keys)

**Operations:**
- ✅ Read


**Fields:**





## Generated Code

This provider was automatically generated from SDK metadata using the Hemmer Provider Generator.

- **Generator**: hemmer-provider-generator
- **SDK**: kubernetes SDK vv1
- **Services**: 63
- **Total Resources**: 185
- **Generated**: 2025-10-29T21:09:30.870004-04:00

## License

Apache-2.0
