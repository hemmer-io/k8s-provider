# Kubernetes Provider for Hemmer

Auto-generated unified provider for kubernetes using SDK version v1.

This provider includes **63 services** with a total of **185 resources**.

## Services

### Apis__internal (1 resources)

- **Internal.apiserver.k8s.io**R - get information of a group

### Apis__events (2 resources)

- **V1**R - get available resources
- **Event** [CRUD] - create an Event

### Apis__storage (1 resources)

- **Storage.k8s.io**R - get information of a group

### Apis__resource (2 resources)

- **Devicetaintrule** [CRUD] - create a DeviceTaintRule
- **V1alpha3**R - get available resources

###  (1 resources)

- **Openid-configuration**R - get service account issuer OpenID configuration, also known as the 'OIDC discovery doc'

### Logs (1 resources)

- **Log**R - 

### Apis__storagemigration (3 resources)

- **Storageversionmigration** [CRUD] - create a StorageVersionMigration
- **Statu**RU - read status of the specified StorageVersionMigration
- **V1alpha1**R - get available resources

### Apis__storage (8 resources)

- **Storageclasse** [CRUD] - create a StorageClass
- **V1**R - get available resources
- **Statu**RU - read status of the specified VolumeAttachment
- **Csidriver** [CRUD] - create a CSIDriver
- **Csistoragecapacitie** [CRUD] - create a CSIStorageCapacity
- **Volumeattributesclasse** [CRUD] - create a VolumeAttributesClass
- **Csinode** [CRUD] - create a CSINode
- **Volumeattachment** [CRUD] - create a VolumeAttachment

### Apis (1 resources)

- **Api**R - get available API versions

### Apis__resource (6 resources)

- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Resourceslice** [CRUD] - create a ResourceSlice
- **Deviceclasse** [CRUD] - create a DeviceClass
- **Statu**RU - read status of the specified ResourceClaim
- **Resourceclaim** [CRUD] - create a ResourceClaim
- **V1beta1**R - get available resources

### Apis__admissionregistration (1 resources)

- **Admissionregistration.k8s.io**R - get information of a group

### Apis__discovery (1 resources)

- **Discovery.k8s.io**R - get information of a group

### Apis__apiextensions (3 resources)

- **Customresourcedefinition** [CRUD] - create a CustomResourceDefinition
- **Statu**RU - read status of the specified CustomResourceDefinition
- **V1**R - get available resources

### Apis__coordination (1 resources)

- **Coordination.k8s.io**R - get information of a group

### Apis__certificates (4 resources)

- **V1alpha1**R - get available resources
- **Statu**RU - read status of the specified PodCertificateRequest
- **Podcertificaterequest** [CRUD] - create a PodCertificateRequest
- **Clustertrustbundle** [CRUD] - create a ClusterTrustBundle

### Apis__coordination (2 resources)

- **V1alpha2**R - get available resources
- **Leasecandidate** [CRUD] - create a LeaseCandidate

### Apis__rbac (1 resources)

- **Rbac.authorization.k8s.io**R - get information of a group

### Apis__flowcontrol (1 resources)

- **Flowcontrol.apiserver.k8s.io**R - get information of a group

### Apis__node (2 resources)

- **Runtimeclasse** [CRUD] - create a RuntimeClass
- **V1**R - get available resources

### Apis__rbac (5 resources)

- **Role** [CRUD] - create a Role
- **Clusterrolebinding** [CRUD] - create a ClusterRoleBinding
- **Rolebinding** [CRUD] - create a RoleBinding
- **V1**R - get available resources
- **Clusterrole** [CRUD] - create a ClusterRole

### Apis__coordination (2 resources)

- **Lease** [CRUD] - create a Lease
- **V1**R - get available resources

### Apis__storagemigration (1 resources)

- **Storagemigration.k8s.io**R - get information of a group

### Apis__authorization (1 resources)

- **Authorization.k8s.io**R - get information of a group

### Autoscaling (1 resources)

- **Autoscaling**R - get information of a group

### Apis__admissionregistration (3 resources)

- **V1beta1**R - get available resources
- **Mutatingadmissionpolicybinding** [CRUD] - create a MutatingAdmissionPolicyBinding
- **Mutatingadmissionpolicie** [CRUD] - create a MutatingAdmissionPolicy

### Version (1 resources)

- **Version**R - get the version information for this server

### AppsV1 (8 resources)

- **Replicaset** [CRUD] - create a ReplicaSet
- **Statefulset** [CRUD] - create a StatefulSet
- **Statu**RU - read status of the specified StatefulSet
- **V1**R - get available resources
- **Scale**RU - read scale of the specified StatefulSet
- **Daemonset** [CRUD] - create a DaemonSet
- **Deployment** [CRUD] - create a Deployment
- **Controllerrevision** [CRUD] - create a ControllerRevision

### AutoscalingV1 (3 resources)

- **V1**R - get available resources
- **Statu**RU - read status of the specified HorizontalPodAutoscaler
- **Horizontalpodautoscaler** [CRUD] - create a HorizontalPodAutoscaler

### Apis__certificates (1 resources)

- **Certificates.k8s.io**R - get information of a group

### CoreV1 (30 resources)

- **Endpoint** [CRUD] - create Endpoints
- **Namespace** [CRUD] - create a Namespace
- **Scale**RU - read scale of the specified ReplicationController
- **Eviction** [C - create eviction of a Pod
- **Ephemeralcontainer**RU - read ephemeralcontainers of the specified Pod
- **Statu**RU - read status of the specified Service
- **Binding** [C - create a Binding
- **Secret** [CRUD] - create a Secret
- **Podtemplate** [CRUD] - create a PodTemplate
- **Resize**RU - read resize of the specified Pod
- **Proxy** [CRUD] - connect POST requests to proxy of Pod
- **Configmap** [CRUD] - create a ConfigMap
- **Pod** [CRUD] - create a Pod
- **Limitrange** [CRUD] - create a LimitRange
- **Exec** [CR - connect POST requests to exec of Pod
- **Componentstatuse**R - read the specified ComponentStatus
- **Portforward** [CR - connect POST requests to portforward of Pod
- **V1**R - get available resources
- **Node** [CRUD] - create a Node
- **Event** [CRUD] - create an Event
- **Resourcequota** [CRUD] - create a ResourceQuota
- **Log**R - read log of the specified Pod
- **Finalize**U - 
- **Replicationcontroller** [CRUD] - create a ReplicationController
- **Serviceaccount** [CRUD] - create a ServiceAccount
- **Token** [C - create token of a ServiceAccount
- **Persistentvolumeclaim** [CRUD] - create a PersistentVolumeClaim
- **Attach** [CR - connect POST requests to attach of Pod
- **Persistentvolume** [CRUD] - create a PersistentVolume
- **Service** [CRUD] - create a Service

### PolicyV1 (3 resources)

- **Statu**RU - read status of the specified PodDisruptionBudget
- **Poddisruptionbudget** [CRUD] - create a PodDisruptionBudget
- **V1**R - get available resources

### Apis__scheduling (2 resources)

- **Priorityclasse** [CRUD] - create a PriorityClass
- **V1**R - get available resources

### Apis__authentication (1 resources)

- **Authentication.k8s.io**R - get information of a group

### Apis__admissionregistration (3 resources)

- **Mutatingadmissionpolicie** [CRUD] - create a MutatingAdmissionPolicy
- **V1alpha1**R - get available resources
- **Mutatingadmissionpolicybinding** [CRUD] - create a MutatingAdmissionPolicyBinding

### Batch (1 resources)

- **Batch**R - get information of a group

### Apis__authorization (5 resources)

- **Localsubjectaccessreview** [C - create a LocalSubjectAccessReview
- **Selfsubjectaccessreview** [C - create a SelfSubjectAccessReview
- **Selfsubjectrulesreview** [C - create a SelfSubjectRulesReview
- **V1**R - get available resources
- **Subjectaccessreview** [C - create a SubjectAccessReview

### Apis__apiextensions (1 resources)

- **Apiextensions.k8s.io**R - get information of a group

### Apis__node (1 resources)

- **Node.k8s.io**R - get information of a group

### Apis__networking (7 resources)

- **Networkpolicie** [CRUD] - create a NetworkPolicy
- **Servicecidr** [CRUD] - create a ServiceCIDR
- **Ingressclasse** [CRUD] - create an IngressClass
- **Statu**RU - read status of the specified Ingress
- **V1**R - get available resources
- **Ingresse** [CRUD] - create an Ingress
- **Ipaddresse** [CRUD] - create an IPAddress

### Apis__apiregistration (3 resources)

- **Apiservice** [CRUD] - create an APIService
- **Statu**RU - read status of the specified APIService
- **V1**R - get available resources

### Apis__networking (4 resources)

- **Ipaddresse** [CRUD] - create an IPAddress
- **Statu**RU - read status of the specified ServiceCIDR
- **Servicecidr** [CRUD] - create a ServiceCIDR
- **V1beta1**R - get available resources

### Apis__certificates (4 resources)

- **Approval**RU - read approval of the specified CertificateSigningRequest
- **Statu**RU - read status of the specified CertificateSigningRequest
- **V1**R - get available resources
- **Certificatesigningrequest** [CRUD] - create a CertificateSigningRequest

### Apis__coordination (2 resources)

- **Leasecandidate** [CRUD] - create a LeaseCandidate
- **V1beta1**R - get available resources

### AutoscalingV2 (3 resources)

- **Horizontalpodautoscaler** [CRUD] - create a HorizontalPodAutoscaler
- **V2**R - get available resources
- **Statu**RU - read status of the specified HorizontalPodAutoscaler

### Apis__networking (1 resources)

- **Networking.k8s.io**R - get information of a group

### Apis__authentication (3 resources)

- **Tokenreview** [C - create a TokenReview
- **V1**R - get available resources
- **Selfsubjectreview** [C - create a SelfSubjectReview

### Policy (1 resources)

- **Policy**R - get information of a group

### Apis__internal (3 resources)

- **Statu**RU - read status of the specified StorageVersion
- **Storageversion** [CRUD] - create a StorageVersion
- **V1alpha1**R - get available resources

### Apps (1 resources)

- **App**R - get information of a group

### Core (1 resources)

- **Api**R - get available API versions

### Apis__storage (2 resources)

- **V1beta1**R - get available resources
- **Volumeattributesclasse** [CRUD] - create a VolumeAttributesClass

### Apis__certificates (2 resources)

- **V1beta1**R - get available resources
- **Clustertrustbundle** [CRUD] - create a ClusterTrustBundle

### Apis__events (1 resources)

- **Events.k8s.io**R - get information of a group

### Apis__flowcontrol (4 resources)

- **Prioritylevelconfiguration** [CRUD] - create a PriorityLevelConfiguration
- **Statu**RU - read status of the specified PriorityLevelConfiguration
- **Flowschema** [CRUD] - create a FlowSchema
- **V1**R - get available resources

### Apis__scheduling (1 resources)

- **Scheduling.k8s.io**R - get information of a group

### Apis__discovery (2 resources)

- **Endpointslice** [CRUD] - create an EndpointSlice
- **V1**R - get available resources

### Apis__resource (6 resources)

- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Statu**RU - read status of the specified ResourceClaim
- **V1beta2**R - get available resources
- **Resourceslice** [CRUD] - create a ResourceSlice
- **Resourceclaim** [CRUD] - create a ResourceClaim
- **Deviceclasse** [CRUD] - create a DeviceClass

### Apis__resource (1 resources)

- **Resource.k8s.io**R - get information of a group

### Apis__apiregistration (1 resources)

- **Apiregistration.k8s.io**R - get information of a group

### BatchV1 (4 resources)

- **Cronjob** [CRUD] - create a CronJob
- **V1**R - get available resources
- **Statu**RU - read status of the specified CronJob
- **Job** [CRUD] - create a Job

### Apis__admissionregistration (6 resources)

- **Mutatingwebhookconfiguration** [CRUD] - create a MutatingWebhookConfiguration
- **Validatingadmissionpolicie** [CRUD] - create a ValidatingAdmissionPolicy
- **Statu**RU - read status of the specified ValidatingAdmissionPolicy
- **Validatingwebhookconfiguration** [CRUD] - create a ValidatingWebhookConfiguration
- **V1**R - get available resources
- **Validatingadmissionpolicybinding** [CRUD] - create a ValidatingAdmissionPolicyBinding

### Apis__resource (6 resources)

- **Resourceslice** [CRUD] - create a ResourceSlice
- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Statu**RU - read status of the specified ResourceClaim
- **Deviceclasse** [CRUD] - create a DeviceClass
- **Resourceclaim** [CRUD] - create a ResourceClaim
- **V1**R - get available resources

### OpenidV1Jwks (1 resources)

- **Jwk**R - get service account issuer OpenID JSON Web Key Set (contains public token verification keys)



## Usage

```rust
use hemmer_kubernetes_provider::KubernetesProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create unified provider
    let provider = KubernetesProvider::new().await?;

    // Access services
    let > internal_service = provider.internal();
    let > events_service = provider.events();
    let > storage_service = provider.storage();
    let > resource_service = provider.resource();
    let _service = provider.();
    let > logs_service = provider.logs();
    let > storagemigration_service = provider.storagemigration();
    let > storage_service = provider.storage();
    let > apis_service = provider.apis();
    let > resource_service = provider.resource();
    let > admissionregistration_service = provider.admissionregistration();
    let > discovery_service = provider.discovery();
    let > apiextensions_service = provider.apiextensions();
    let > coordination_service = provider.coordination();
    let > certificates_service = provider.certificates();
    let > coordination_service = provider.coordination();
    let > rbac_service = provider.rbac();
    let > flowcontrol_service = provider.flowcontrol();
    let > node_service = provider.node();
    let > rbac_service = provider.rbac();
    let > coordination_service = provider.coordination();
    let > storagemigration_service = provider.storagemigration();
    let > authorization_service = provider.authorization();
    let > autoscaling_service = provider.autoscaling();
    let > admissionregistration_service = provider.admissionregistration();
    let > version_service = provider.version();
    let > apps_v1_service = provider.apps_v1();
    let > autoscaling_v1_service = provider.autoscaling_v1();
    let > certificates_service = provider.certificates();
    let > core_v1_service = provider.core_v1();
    let > policy_v1_service = provider.policy_v1();
    let > scheduling_service = provider.scheduling();
    let > authentication_service = provider.authentication();
    let > admissionregistration_service = provider.admissionregistration();
    let > batch_service = provider.batch();
    let > authorization_service = provider.authorization();
    let > apiextensions_service = provider.apiextensions();
    let > node_service = provider.node();
    let > networking_service = provider.networking();
    let > apiregistration_service = provider.apiregistration();
    let > networking_service = provider.networking();
    let > certificates_service = provider.certificates();
    let > coordination_service = provider.coordination();
    let > autoscaling_v2_service = provider.autoscaling_v2();
    let > networking_service = provider.networking();
    let > authentication_service = provider.authentication();
    let > policy_service = provider.policy();
    let > internal_service = provider.internal();
    let > apps_service = provider.apps();
    let core_service = provider.core();
    let > storage_service = provider.storage();
    let > certificates_service = provider.certificates();
    let > events_service = provider.events();
    let > flowcontrol_service = provider.flowcontrol();
    let > scheduling_service = provider.scheduling();
    let > discovery_service = provider.discovery();
    let > resource_service = provider.resource();
    let > resource_service = provider.resource();
    let > apiregistration_service = provider.apiregistration();
    let > batch_v1_service = provider.batch_v1();
    let > admissionregistration_service = provider.admissionregistration();
    let > resource_service = provider.resource();
    let > openid_v1_jwks_service = provider.openid_v1_jwks();

    // Use resources from any service
    let internal.apiserver.k8s.io = > internal_service.internal.apiserver.k8s.io();
    let v1 = > events_service.v1();
    let event = > events_service.event();
    let storage.k8s.io = > storage_service.storage.k8s.io();
    let devicetaintrule = > resource_service.devicetaintrule();
    let v1alpha3 = > resource_service.v1alpha3();
    let openid-configuration = _service.openid-configuration();
    let log = > logs_service.log();
    let storageversionmigration = > storagemigration_service.storageversionmigration();
    let statu = > storagemigration_service.statu();
    let v1alpha1 = > storagemigration_service.v1alpha1();
    let storageclasse = > storage_service.storageclasse();
    let v1 = > storage_service.v1();
    let statu = > storage_service.statu();
    let csidriver = > storage_service.csidriver();
    let csistoragecapacitie = > storage_service.csistoragecapacitie();
    let volumeattributesclasse = > storage_service.volumeattributesclasse();
    let csinode = > storage_service.csinode();
    let volumeattachment = > storage_service.volumeattachment();
    let api = > apis_service.api();
    let resourceclaimtemplate = > resource_service.resourceclaimtemplate();
    let resourceslice = > resource_service.resourceslice();
    let deviceclasse = > resource_service.deviceclasse();
    let statu = > resource_service.statu();
    let resourceclaim = > resource_service.resourceclaim();
    let v1beta1 = > resource_service.v1beta1();
    let admissionregistration.k8s.io = > admissionregistration_service.admissionregistration.k8s.io();
    let discovery.k8s.io = > discovery_service.discovery.k8s.io();
    let customresourcedefinition = > apiextensions_service.customresourcedefinition();
    let statu = > apiextensions_service.statu();
    let v1 = > apiextensions_service.v1();
    let coordination.k8s.io = > coordination_service.coordination.k8s.io();
    let v1alpha1 = > certificates_service.v1alpha1();
    let statu = > certificates_service.statu();
    let podcertificaterequest = > certificates_service.podcertificaterequest();
    let clustertrustbundle = > certificates_service.clustertrustbundle();
    let v1alpha2 = > coordination_service.v1alpha2();
    let leasecandidate = > coordination_service.leasecandidate();
    let rbac.authorization.k8s.io = > rbac_service.rbac.authorization.k8s.io();
    let flowcontrol.apiserver.k8s.io = > flowcontrol_service.flowcontrol.apiserver.k8s.io();
    let runtimeclasse = > node_service.runtimeclasse();
    let v1 = > node_service.v1();
    let role = > rbac_service.role();
    let clusterrolebinding = > rbac_service.clusterrolebinding();
    let rolebinding = > rbac_service.rolebinding();
    let v1 = > rbac_service.v1();
    let clusterrole = > rbac_service.clusterrole();
    let lease = > coordination_service.lease();
    let v1 = > coordination_service.v1();
    let storagemigration.k8s.io = > storagemigration_service.storagemigration.k8s.io();
    let authorization.k8s.io = > authorization_service.authorization.k8s.io();
    let autoscaling = > autoscaling_service.autoscaling();
    let v1beta1 = > admissionregistration_service.v1beta1();
    let mutatingadmissionpolicybinding = > admissionregistration_service.mutatingadmissionpolicybinding();
    let mutatingadmissionpolicie = > admissionregistration_service.mutatingadmissionpolicie();
    let version = > version_service.version();
    let replicaset = > apps_v1_service.replicaset();
    let statefulset = > apps_v1_service.statefulset();
    let statu = > apps_v1_service.statu();
    let v1 = > apps_v1_service.v1();
    let scale = > apps_v1_service.scale();
    let daemonset = > apps_v1_service.daemonset();
    let deployment = > apps_v1_service.deployment();
    let controllerrevision = > apps_v1_service.controllerrevision();
    let v1 = > autoscaling_v1_service.v1();
    let statu = > autoscaling_v1_service.statu();
    let horizontalpodautoscaler = > autoscaling_v1_service.horizontalpodautoscaler();
    let certificates.k8s.io = > certificates_service.certificates.k8s.io();
    let endpoint = > core_v1_service.endpoint();
    let namespace = > core_v1_service.namespace();
    let scale = > core_v1_service.scale();
    let eviction = > core_v1_service.eviction();
    let ephemeralcontainer = > core_v1_service.ephemeralcontainer();
    let statu = > core_v1_service.statu();
    let binding = > core_v1_service.binding();
    let secret = > core_v1_service.secret();
    let podtemplate = > core_v1_service.podtemplate();
    let resize = > core_v1_service.resize();
    let proxy = > core_v1_service.proxy();
    let configmap = > core_v1_service.configmap();
    let pod = > core_v1_service.pod();
    let limitrange = > core_v1_service.limitrange();
    let exec = > core_v1_service.exec();
    let componentstatuse = > core_v1_service.componentstatuse();
    let portforward = > core_v1_service.portforward();
    let v1 = > core_v1_service.v1();
    let node = > core_v1_service.node();
    let event = > core_v1_service.event();
    let resourcequota = > core_v1_service.resourcequota();
    let log = > core_v1_service.log();
    let finalize = > core_v1_service.finalize();
    let replicationcontroller = > core_v1_service.replicationcontroller();
    let serviceaccount = > core_v1_service.serviceaccount();
    let token = > core_v1_service.token();
    let persistentvolumeclaim = > core_v1_service.persistentvolumeclaim();
    let attach = > core_v1_service.attach();
    let persistentvolume = > core_v1_service.persistentvolume();
    let service = > core_v1_service.service();
    let statu = > policy_v1_service.statu();
    let poddisruptionbudget = > policy_v1_service.poddisruptionbudget();
    let v1 = > policy_v1_service.v1();
    let priorityclasse = > scheduling_service.priorityclasse();
    let v1 = > scheduling_service.v1();
    let authentication.k8s.io = > authentication_service.authentication.k8s.io();
    let mutatingadmissionpolicie = > admissionregistration_service.mutatingadmissionpolicie();
    let v1alpha1 = > admissionregistration_service.v1alpha1();
    let mutatingadmissionpolicybinding = > admissionregistration_service.mutatingadmissionpolicybinding();
    let batch = > batch_service.batch();
    let localsubjectaccessreview = > authorization_service.localsubjectaccessreview();
    let selfsubjectaccessreview = > authorization_service.selfsubjectaccessreview();
    let selfsubjectrulesreview = > authorization_service.selfsubjectrulesreview();
    let v1 = > authorization_service.v1();
    let subjectaccessreview = > authorization_service.subjectaccessreview();
    let apiextensions.k8s.io = > apiextensions_service.apiextensions.k8s.io();
    let node.k8s.io = > node_service.node.k8s.io();
    let networkpolicie = > networking_service.networkpolicie();
    let servicecidr = > networking_service.servicecidr();
    let ingressclasse = > networking_service.ingressclasse();
    let statu = > networking_service.statu();
    let v1 = > networking_service.v1();
    let ingresse = > networking_service.ingresse();
    let ipaddresse = > networking_service.ipaddresse();
    let apiservice = > apiregistration_service.apiservice();
    let statu = > apiregistration_service.statu();
    let v1 = > apiregistration_service.v1();
    let ipaddresse = > networking_service.ipaddresse();
    let statu = > networking_service.statu();
    let servicecidr = > networking_service.servicecidr();
    let v1beta1 = > networking_service.v1beta1();
    let approval = > certificates_service.approval();
    let statu = > certificates_service.statu();
    let v1 = > certificates_service.v1();
    let certificatesigningrequest = > certificates_service.certificatesigningrequest();
    let leasecandidate = > coordination_service.leasecandidate();
    let v1beta1 = > coordination_service.v1beta1();
    let horizontalpodautoscaler = > autoscaling_v2_service.horizontalpodautoscaler();
    let v2 = > autoscaling_v2_service.v2();
    let statu = > autoscaling_v2_service.statu();
    let networking.k8s.io = > networking_service.networking.k8s.io();
    let tokenreview = > authentication_service.tokenreview();
    let v1 = > authentication_service.v1();
    let selfsubjectreview = > authentication_service.selfsubjectreview();
    let policy = > policy_service.policy();
    let statu = > internal_service.statu();
    let storageversion = > internal_service.storageversion();
    let v1alpha1 = > internal_service.v1alpha1();
    let app = > apps_service.app();
    let api = core_service.api();
    let v1beta1 = > storage_service.v1beta1();
    let volumeattributesclasse = > storage_service.volumeattributesclasse();
    let v1beta1 = > certificates_service.v1beta1();
    let clustertrustbundle = > certificates_service.clustertrustbundle();
    let events.k8s.io = > events_service.events.k8s.io();
    let prioritylevelconfiguration = > flowcontrol_service.prioritylevelconfiguration();
    let statu = > flowcontrol_service.statu();
    let flowschema = > flowcontrol_service.flowschema();
    let v1 = > flowcontrol_service.v1();
    let scheduling.k8s.io = > scheduling_service.scheduling.k8s.io();
    let endpointslice = > discovery_service.endpointslice();
    let v1 = > discovery_service.v1();
    let resourceclaimtemplate = > resource_service.resourceclaimtemplate();
    let statu = > resource_service.statu();
    let v1beta2 = > resource_service.v1beta2();
    let resourceslice = > resource_service.resourceslice();
    let resourceclaim = > resource_service.resourceclaim();
    let deviceclasse = > resource_service.deviceclasse();
    let resource.k8s.io = > resource_service.resource.k8s.io();
    let apiregistration.k8s.io = > apiregistration_service.apiregistration.k8s.io();
    let cronjob = > batch_v1_service.cronjob();
    let v1 = > batch_v1_service.v1();
    let statu = > batch_v1_service.statu();
    let job = > batch_v1_service.job();
    let mutatingwebhookconfiguration = > admissionregistration_service.mutatingwebhookconfiguration();
    let validatingadmissionpolicie = > admissionregistration_service.validatingadmissionpolicie();
    let statu = > admissionregistration_service.statu();
    let validatingwebhookconfiguration = > admissionregistration_service.validatingwebhookconfiguration();
    let v1 = > admissionregistration_service.v1();
    let validatingadmissionpolicybinding = > admissionregistration_service.validatingadmissionpolicybinding();
    let resourceslice = > resource_service.resourceslice();
    let resourceclaimtemplate = > resource_service.resourceclaimtemplate();
    let statu = > resource_service.statu();
    let deviceclasse = > resource_service.deviceclasse();
    let resourceclaim = > resource_service.resourceclaim();
    let v1 = > resource_service.v1();
    let jwk = > openid_v1_jwks_service.jwk();

    Ok(())
}
```

## Service Details

### Apis__internal

**Resources:**

#### Internal.apiserver.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__events

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



### Apis__storage

**Resources:**

#### Storage.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__resource

**Resources:**

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


#### V1alpha3

get available resources

**Operations:**

- ✅ Read



**Fields:**



### 

**Resources:**

#### Openid-configuration

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



### Apis__storagemigration

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



### Apis__storage

**Resources:**

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



### Apis

**Resources:**

#### Api

get available API versions

**Operations:**

- ✅ Read



**Fields:**



### Apis__resource

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


#### V1beta1

get available resources

**Operations:**

- ✅ Read



**Fields:**



### Apis__admissionregistration

**Resources:**

#### Admissionregistration.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__discovery

**Resources:**

#### Discovery.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__apiextensions

**Resources:**

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


#### Statu

read status of the specified CustomResourceDefinition

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



### Apis__coordination

**Resources:**

#### Coordination.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__certificates

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



### Apis__coordination

**Resources:**

#### V1alpha2

get available resources

**Operations:**

- ✅ Read



**Fields:**


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



### Apis__rbac

**Resources:**

#### Rbac.authorization.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__flowcontrol

**Resources:**

#### Flowcontrol.apiserver.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__node

**Resources:**

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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**



### Apis__rbac

**Resources:**

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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**


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



### Apis__coordination

**Resources:**

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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**



### Apis__storagemigration

**Resources:**

#### Storagemigration.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__authorization

**Resources:**

#### Authorization.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Autoscaling

**Resources:**

#### Autoscaling

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__admissionregistration

**Resources:**

#### V1beta1

get available resources

**Operations:**

- ✅ Read



**Fields:**


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



### Version

**Resources:**

#### Version

get the version information for this server

**Operations:**

- ✅ Read



**Fields:**



### AppsV1

**Resources:**

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


#### Statu

read status of the specified StatefulSet

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


#### Scale

read scale of the specified StatefulSet

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



### AutoscalingV1

**Resources:**

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



### Apis__certificates

**Resources:**

#### Certificates.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### CoreV1

**Resources:**

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


#### Scale

read scale of the specified ReplicationController

**Operations:**

- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Eviction

create eviction of a Pod

**Operations:**
- ✅ Create




**Fields:**


#### Ephemeralcontainer

read ephemeralcontainers of the specified Pod

**Operations:**

- ✅ Read
- ✅ Update


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


#### Binding

create a Binding

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


#### Resize

read resize of the specified Pod

**Operations:**

- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Proxy

connect POST requests to proxy of Pod

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete

**Fields:**


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


#### Exec

connect POST requests to exec of Pod

**Operations:**
- ✅ Create
- ✅ Read



**Fields:**


#### Componentstatuse

read the specified ComponentStatus

**Operations:**

- ✅ Read



**Fields:**


#### Portforward

connect POST requests to portforward of Pod

**Operations:**
- ✅ Create
- ✅ Read



**Fields:**


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**


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


#### Log

read log of the specified Pod

**Operations:**

- ✅ Read



**Fields:**


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


#### Token

create token of a ServiceAccount

**Operations:**
- ✅ Create




**Fields:**


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


#### Attach

connect POST requests to attach of Pod

**Operations:**
- ✅ Create
- ✅ Read



**Fields:**


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



### PolicyV1

**Resources:**

#### Statu

read status of the specified PodDisruptionBudget

**Operations:**

- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**



### Apis__scheduling

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



### Apis__authentication

**Resources:**

#### Authentication.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__admissionregistration

**Resources:**

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



### Batch

**Resources:**

#### Batch

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__authorization

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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**


#### Subjectaccessreview

create a SubjectAccessReview

**Operations:**
- ✅ Create




**Fields:**



### Apis__apiextensions

**Resources:**

#### Apiextensions.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__node

**Resources:**

#### Node.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__networking

**Resources:**

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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**


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



### Apis__apiregistration

**Resources:**

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


#### Statu

read status of the specified APIService

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



### Apis__networking

**Resources:**

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


#### Statu

read status of the specified ServiceCIDR

**Operations:**

- ✅ Read
- ✅ Update


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



### Apis__certificates

**Resources:**

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



### Apis__coordination

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



### AutoscalingV2

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



### Apis__networking

**Resources:**

#### Networking.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__authentication

**Resources:**

#### Tokenreview

create a TokenReview

**Operations:**
- ✅ Create




**Fields:**


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



### Policy

**Resources:**

#### Policy

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__internal

**Resources:**

#### Statu

read status of the specified StorageVersion

**Operations:**

- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


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


#### V1alpha1

get available resources

**Operations:**

- ✅ Read



**Fields:**



### Apps

**Resources:**

#### App

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Core

**Resources:**

#### Api

get available API versions

**Operations:**

- ✅ Read



**Fields:**



### Apis__storage

**Resources:**

#### V1beta1

get available resources

**Operations:**

- ✅ Read



**Fields:**


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



### Apis__certificates

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



### Apis__events

**Resources:**

#### Events.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__flowcontrol

**Resources:**

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


#### Statu

read status of the specified PriorityLevelConfiguration

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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**



### Apis__scheduling

**Resources:**

#### Scheduling.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__discovery

**Resources:**

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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**



### Apis__resource

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


#### Statu

read status of the specified ResourceClaim

**Operations:**

- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1beta2

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



### Apis__resource

**Resources:**

#### Resource.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### Apis__apiregistration

**Resources:**

#### Apiregistration.k8s.io

get information of a group

**Operations:**

- ✅ Read



**Fields:**



### BatchV1

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



### Apis__admissionregistration

**Resources:**

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


#### Statu

read status of the specified ValidatingAdmissionPolicy

**Operations:**

- ✅ Read
- ✅ Update


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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**


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



### Apis__resource

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


#### Statu

read status of the specified ResourceClaim

**Operations:**

- ✅ Read
- ✅ Update


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


#### V1

get available resources

**Operations:**

- ✅ Read



**Fields:**



### OpenidV1Jwks

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
- **Generated**: 2025-10-29T19:20:32.192678-04:00

## License

Apache-2.0
