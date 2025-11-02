# K8s Provider for Hemmer

Auto-generated unified provider for k8s using SDK version v1.

This provider includes **26 services** with a total of **184 resources**.

## Services

### Apis_storage (11 resources)

- **Storage** [R] - get information of a group
- **Csinode** [CRUD] - create a CSINode
- **Volumeattachment** [CRUD] - create a VolumeAttachment
- **Csistoragecapacitie** [CRUD] - create a CSIStorageCapacity
- **Storageclasse** [CRUD] - create a StorageClass
- **Statu** [RU] - read status of the specified VolumeAttachment
- **V1** [R] - get available resources
- **Volumeattributesclasse** [CRUD] - create a VolumeAttributesClass
- **Csidriver** [CRUD] - create a CSIDriver
- **V1beta1** [R] - get available resources
- **Volumeattributesclasse** [CRUD] - create a VolumeAttributesClass

### Apis_rbac (6 resources)

- **Rbac.authorization** [R] - get information of a group
- **Rolebinding** [CRUD] - create a RoleBinding
- **Clusterrolebinding** [CRUD] - create a ClusterRoleBinding
- **V1** [R] - get available resources
- **Clusterrole** [CRUD] - create a ClusterRole
- **Role** [CRUD] - create a Role

### Apis_events (3 resources)

- **Event** [CRUD] - create an Event
- **V1** [R] - get available resources
- **Event** [R] - get information of a group

### Apis_flowcontrol (5 resources)

- **Flowcontrol** [R] - get information of a group
- **Prioritylevelconfiguration** [CRUD] - create a PriorityLevelConfiguration
- **Flowschema** [CRUD] - create a FlowSchema
- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified PriorityLevelConfiguration

### Version (1 resources)

- **Version** [R] - get the version information for this server

### Openid (1 resources)

- **Jwk** [R] - get service account issuer OpenID JSON Web Key Set (contains public token verification keys)

### Apis_apps (9 resources)

- **Deployment** [CRUD] - create a Deployment
- **Daemonset** [CRUD] - create a DaemonSet
- **Replicaset** [CRUD] - create a ReplicaSet
- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified ReplicaSet
- **Scale** [RU] - read scale of the specified ReplicaSet
- **Statefulset** [CRUD] - create a StatefulSet
- **Controllerrevision** [CRUD] - create a ControllerRevision
- **App** [R] - get information of a group

### Apis_scheduling (3 resources)

- **Priorityclasse** [CRUD] - create a PriorityClass
- **V1** [R] - get available resources
- **Scheduling** [R] - get information of a group

### Apis (4 resources)

- **Api** [R] - get available API versions
- **Discovery** [R] - get information of a group
- **Endpointslice** [CRUD] - create an EndpointSlice
- **V1** [R] - get available resources

### Apis_storagemigration (4 resources)

- **Storagemigration** [R] - get information of a group
- **Statu** [RU] - read status of the specified StorageVersionMigration
- **V1beta1** [R] - get available resources
- **Storageversionmigration** [CRUD] - create a StorageVersionMigration

### Apis_certificates (11 resources)

- **V1alpha1** [R] - get available resources
- **Clustertrustbundle** [CRUD] - create a ClusterTrustBundle
- **Podcertificaterequest** [CRUD] - create a PodCertificateRequest
- **Statu** [RU] - read status of the specified PodCertificateRequest
- **Certificate** [R] - get information of a group
- **Approval** [RU] - read approval of the specified CertificateSigningRequest
- **Certificatesigningrequest** [CRUD] - create a CertificateSigningRequest
- **Statu** [RU] - read status of the specified CertificateSigningRequest
- **V1** [R] - get available resources
- **Clustertrustbundle** [CRUD] - create a ClusterTrustBundle
- **V1beta1** [R] - get available resources

### Apis_batch (5 resources)

- **Batch** [R] - get information of a group
- **Job** [CRUD] - create a Job
- **Statu** [RU] - read status of the specified Job
- **Cronjob** [CRUD] - create a CronJob
- **V1** [R] - get available resources

### Apis_node (3 resources)

- **V1** [R] - get available resources
- **Runtimeclasse** [CRUD] - create a RuntimeClass
- **Node** [R] - get information of a group

### Apis_resource (21 resources)

- **V1alpha3** [R] - get available resources
- **Devicetaintrule** [CRUD] - create a DeviceTaintRule
- **V1beta1** [R] - get available resources
- **Statu** [RU] - read status of the specified ResourceClaim
- **Resourceslice** [CRUD] - create a ResourceSlice
- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Resourceclaim** [CRUD] - create a ResourceClaim
- **Deviceclasse** [CRUD] - create a DeviceClass
- **Resourceslice** [CRUD] - create a ResourceSlice
- **V1beta2** [R] - get available resources
- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Deviceclasse** [CRUD] - create a DeviceClass
- **Statu** [RU] - read status of the specified ResourceClaim
- **Resourceclaim** [CRUD] - create a ResourceClaim
- **Resource** [R] - get information of a group
- **Resourceslice** [CRUD] - create a ResourceSlice
- **Statu** [RU] - read status of the specified ResourceClaim
- **V1** [R] - get available resources
- **Resourceclaimtemplate** [CRUD] - create a ResourceClaimTemplate
- **Deviceclasse** [CRUD] - create a DeviceClass
- **Resourceclaim** [CRUD] - create a ResourceClaim

### Apis_networking (12 resources)

- **V1** [R] - get available resources
- **Ingressclasse** [CRUD] - create an IngressClass
- **Ingresse** [CRUD] - create an Ingress
- **Ipaddresse** [CRUD] - create an IPAddress
- **Servicecidr** [CRUD] - create a ServiceCIDR
- **Statu** [RU] - read status of the specified Ingress
- **Networkpolicie** [CRUD] - create a NetworkPolicy
- **Servicecidr** [CRUD] - create a ServiceCIDR
- **Ipaddresse** [CRUD] - create an IPAddress
- **V1beta1** [R] - get available resources
- **Statu** [RU] - read status of the specified ServiceCIDR
- **Networking** [R] - get information of a group

### Apis_apiregistration (4 resources)

- **Apiservice** [CRUD] - create an APIService
- **Statu** [RU] - read status of the specified APIService
- **V1** [R] - get available resources
- **Apiregistration** [R] - get information of a group

### Apis_internal (4 resources)

- **Internal** [R] - get information of a group
- **Statu** [RU] - read status of the specified StorageVersion
- **V1alpha1** [R] - get available resources
- **Storageversion** [CRUD] - create a StorageVersion

### Logs (1 resources)

- **Log** [R] - 

### Apis_policy (4 resources)

- **V1** [R] - get available resources
- **Poddisruptionbudget** [CRUD] - create a PodDisruptionBudget
- **Statu** [RU] - read status of the specified PodDisruptionBudget
- **Policy** [R] - get information of a group

### Apis_authorization (6 resources)

- **Authorization** [R] - get information of a group
- **Subjectaccessreview** [C] - create a SubjectAccessReview
- **Selfsubjectaccessreview** [C] - create a SelfSubjectAccessReview
- **Localsubjectaccessreview** [C] - create a LocalSubjectAccessReview
- **V1** [R] - get available resources
- **Selfsubjectrulesreview** [C] - create a SelfSubjectRulesReview

### Apis_apiextensions (4 resources)

- **V1** [R] - get available resources
- **Statu** [RU] - read status of the specified CustomResourceDefinition
- **Customresourcedefinition** [CRUD] - create a CustomResourceDefinition
- **Apiextension** [R] - get information of a group

### Apis_autoscaling (7 resources)

- **Autoscaling** [R] - get information of a group
- **Statu** [RU] - read status of the specified HorizontalPodAutoscaler
- **Horizontalpodautoscaler** [CRUD] - create a HorizontalPodAutoscaler
- **V1** [R] - get available resources
- **V2** [R] - get available resources
- **Horizontalpodautoscaler** [CRUD] - create a HorizontalPodAutoscaler
- **Statu** [RU] - read status of the specified HorizontalPodAutoscaler

### Api (31 resources)

- **Resize** [RU] - read resize of the specified Pod
- **Statu** [RU] - read status of the specified PersistentVolumeClaim
- **Attach** [CR] - connect POST requests to attach of Pod
- **Podtemplate** [CRUD] - create a PodTemplate
- **Eviction** [C] - create eviction of a Pod
- **Exec** [CR] - connect POST requests to exec of Pod
- **Ephemeralcontainer** [RU] - read ephemeralcontainers of the specified Pod
- **Service** [CRUD] - create a Service
- **Proxy** [CRUD] - connect POST requests to proxy of Node
- **Node** [CRUD] - create a Node
- **Log** [R] - read log of the specified Pod
- **Serviceaccount** [CRUD] - create a ServiceAccount
- **Pod** [CRUD] - create a Pod
- **Binding** [C] - create binding of a Pod
- **Persistentvolume** [CRUD] - create a PersistentVolume
- **Limitrange** [CRUD] - create a LimitRange
- **Configmap** [CRUD] - create a ConfigMap
- **Event** [CRUD] - create an Event
- **Finalize** [U] - 
- **Persistentvolumeclaim** [CRUD] - create a PersistentVolumeClaim
- **Secret** [CRUD] - create a Secret
- **Portforward** [CR] - connect POST requests to portforward of Pod
- **Componentstatuse** [R] - read the specified ComponentStatus
- **Scale** [RU] - read scale of the specified ReplicationController
- **Resourcequota** [CRUD] - create a ResourceQuota
- **Namespace** [CRUD] - create a Namespace
- **Replicationcontroller** [CRUD] - create a ReplicationController
- **Token** [C] - create token of a ServiceAccount
- **V1** [R] - get available resources
- **Endpoint** [CRUD] - create Endpoints
- **Api** [R] - get available API versions

### Apis_authentication (4 resources)

- **Authentication** [R] - get information of a group
- **V1** [R] - get available resources
- **Selfsubjectreview** [C] - create a SelfSubjectReview
- **Tokenreview** [C] - create a TokenReview

### Apis_admissionregistration (13 resources)

- **Admissionregistration** [R] - get information of a group
- **Mutatingadmissionpolicie** [CRUD] - create a MutatingAdmissionPolicy
- **V1beta1** [R] - get available resources
- **Mutatingadmissionpolicybinding** [CRUD] - create a MutatingAdmissionPolicyBinding
- **Mutatingadmissionpolicie** [CRUD] - create a MutatingAdmissionPolicy
- **Mutatingadmissionpolicybinding** [CRUD] - create a MutatingAdmissionPolicyBinding
- **V1alpha1** [R] - get available resources
- **Validatingwebhookconfiguration** [CRUD] - create a ValidatingWebhookConfiguration
- **Validatingadmissionpolicie** [CRUD] - create a ValidatingAdmissionPolicy
- **Statu** [RU] - read status of the specified ValidatingAdmissionPolicy
- **V1** [R] - get available resources
- **Validatingadmissionpolicybinding** [CRUD] - create a ValidatingAdmissionPolicyBinding
- **Mutatingwebhookconfiguration** [CRUD] - create a MutatingWebhookConfiguration

### Apis_coordination (7 resources)

- **Coordination** [R] - get information of a group
- **Leasecandidate** [CRUD] - create a LeaseCandidate
- **V1alpha2** [R] - get available resources
- **Lease** [CRUD] - create a Lease
- **V1** [R] - get available resources
- **Leasecandidate** [CRUD] - create a LeaseCandidate
- **V1beta1** [R] - get available resources



## Installation

### Using Hemmer CLI (Recommended)

Install the provider using the Hemmer CLI:

```bash
hemmer provider install k8s
```

This will automatically download the correct pre-built binary for your platform from the latest GitHub release.

### Manual Installation

1. Download the appropriate binary for your platform from the [Releases](../../releases) page:
   - **macOS (Intel)**: `hemmer-provider-k8s-darwin-amd64`
   - **macOS (Apple Silicon)**: `hemmer-provider-k8s-darwin-arm64`
   - **Linux (x86_64)**: `hemmer-provider-k8s-linux-amd64`
   - **Linux (ARM64)**: `hemmer-provider-k8s-linux-arm64`
   - **Windows (x86_64)**: `hemmer-provider-k8s-windows-amd64.exe`

2. Verify the checksum (optional but recommended):
   ```bash
   sha256sum -c checksums.txt
   ```

3. Move the binary to your Hemmer providers directory:
   ```bash
   # macOS/Linux
   mv hemmer-provider-k8s-* ~/.hemmer/providers/

   # Windows
   move hemmer-provider-k8s-*.exe %USERPROFILE%\.hemmer\providers\
   ```

## Usage

```rust
use hemmer_k8s_provider::K8sProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create unified provider
    let provider = K8sProvider::new().await?;

    // Access services
    let apis_storage_service = provider.apis_storage();
    let apis_rbac_service = provider.apis_rbac();
    let apis_events_service = provider.apis_events();
    let apis_flowcontrol_service = provider.apis_flowcontrol();
    let version_service = provider.version();
    let openid_service = provider.openid();
    let apis_apps_service = provider.apis_apps();
    let apis_scheduling_service = provider.apis_scheduling();
    let apis_service = provider.apis();
    let apis_storagemigration_service = provider.apis_storagemigration();
    let apis_certificates_service = provider.apis_certificates();
    let apis_batch_service = provider.apis_batch();
    let apis_node_service = provider.apis_node();
    let apis_resource_service = provider.apis_resource();
    let apis_networking_service = provider.apis_networking();
    let apis_apiregistration_service = provider.apis_apiregistration();
    let apis_internal_service = provider.apis_internal();
    let logs_service = provider.logs();
    let apis_policy_service = provider.apis_policy();
    let apis_authorization_service = provider.apis_authorization();
    let apis_apiextensions_service = provider.apis_apiextensions();
    let apis_autoscaling_service = provider.apis_autoscaling();
    let api_service = provider.api();
    let apis_authentication_service = provider.apis_authentication();
    let apis_admissionregistration_service = provider.apis_admissionregistration();
    let apis_coordination_service = provider.apis_coordination();

    // Use resources from any service
    let storage = apis_storage_service.storage();
    let csinode = apis_storage_service.csinode();
    let volumeattachment = apis_storage_service.volumeattachment();
    let csistoragecapacitie = apis_storage_service.csistoragecapacitie();
    let storageclasse = apis_storage_service.storageclasse();
    let statu = apis_storage_service.statu();
    let v1 = apis_storage_service.v1();
    let volumeattributesclasse = apis_storage_service.volumeattributesclasse();
    let csidriver = apis_storage_service.csidriver();
    let v1beta1 = apis_storage_service.v1beta1();
    let volumeattributesclasse = apis_storage_service.volumeattributesclasse();
    let rbac.authorization = apis_rbac_service.rbac.authorization();
    let rolebinding = apis_rbac_service.rolebinding();
    let clusterrolebinding = apis_rbac_service.clusterrolebinding();
    let v1 = apis_rbac_service.v1();
    let clusterrole = apis_rbac_service.clusterrole();
    let role = apis_rbac_service.role();
    let event = apis_events_service.event();
    let v1 = apis_events_service.v1();
    let event = apis_events_service.event();
    let flowcontrol = apis_flowcontrol_service.flowcontrol();
    let prioritylevelconfiguration = apis_flowcontrol_service.prioritylevelconfiguration();
    let flowschema = apis_flowcontrol_service.flowschema();
    let v1 = apis_flowcontrol_service.v1();
    let statu = apis_flowcontrol_service.statu();
    let version = version_service.version();
    let jwk = openid_service.jwk();
    let deployment = apis_apps_service.deployment();
    let daemonset = apis_apps_service.daemonset();
    let replicaset = apis_apps_service.replicaset();
    let v1 = apis_apps_service.v1();
    let statu = apis_apps_service.statu();
    let scale = apis_apps_service.scale();
    let statefulset = apis_apps_service.statefulset();
    let controllerrevision = apis_apps_service.controllerrevision();
    let app = apis_apps_service.app();
    let priorityclasse = apis_scheduling_service.priorityclasse();
    let v1 = apis_scheduling_service.v1();
    let scheduling = apis_scheduling_service.scheduling();
    let api = apis_service.api();
    let discovery = apis_service.discovery();
    let endpointslice = apis_service.endpointslice();
    let v1 = apis_service.v1();
    let storagemigration = apis_storagemigration_service.storagemigration();
    let statu = apis_storagemigration_service.statu();
    let v1beta1 = apis_storagemigration_service.v1beta1();
    let storageversionmigration = apis_storagemigration_service.storageversionmigration();
    let v1alpha1 = apis_certificates_service.v1alpha1();
    let clustertrustbundle = apis_certificates_service.clustertrustbundle();
    let podcertificaterequest = apis_certificates_service.podcertificaterequest();
    let statu = apis_certificates_service.statu();
    let certificate = apis_certificates_service.certificate();
    let approval = apis_certificates_service.approval();
    let certificatesigningrequest = apis_certificates_service.certificatesigningrequest();
    let statu = apis_certificates_service.statu();
    let v1 = apis_certificates_service.v1();
    let clustertrustbundle = apis_certificates_service.clustertrustbundle();
    let v1beta1 = apis_certificates_service.v1beta1();
    let batch = apis_batch_service.batch();
    let job = apis_batch_service.job();
    let statu = apis_batch_service.statu();
    let cronjob = apis_batch_service.cronjob();
    let v1 = apis_batch_service.v1();
    let v1 = apis_node_service.v1();
    let runtimeclasse = apis_node_service.runtimeclasse();
    let node = apis_node_service.node();
    let v1alpha3 = apis_resource_service.v1alpha3();
    let devicetaintrule = apis_resource_service.devicetaintrule();
    let v1beta1 = apis_resource_service.v1beta1();
    let statu = apis_resource_service.statu();
    let resourceslice = apis_resource_service.resourceslice();
    let resourceclaimtemplate = apis_resource_service.resourceclaimtemplate();
    let resourceclaim = apis_resource_service.resourceclaim();
    let deviceclasse = apis_resource_service.deviceclasse();
    let resourceslice = apis_resource_service.resourceslice();
    let v1beta2 = apis_resource_service.v1beta2();
    let resourceclaimtemplate = apis_resource_service.resourceclaimtemplate();
    let deviceclasse = apis_resource_service.deviceclasse();
    let statu = apis_resource_service.statu();
    let resourceclaim = apis_resource_service.resourceclaim();
    let resource = apis_resource_service.resource();
    let resourceslice = apis_resource_service.resourceslice();
    let statu = apis_resource_service.statu();
    let v1 = apis_resource_service.v1();
    let resourceclaimtemplate = apis_resource_service.resourceclaimtemplate();
    let deviceclasse = apis_resource_service.deviceclasse();
    let resourceclaim = apis_resource_service.resourceclaim();
    let v1 = apis_networking_service.v1();
    let ingressclasse = apis_networking_service.ingressclasse();
    let ingresse = apis_networking_service.ingresse();
    let ipaddresse = apis_networking_service.ipaddresse();
    let servicecidr = apis_networking_service.servicecidr();
    let statu = apis_networking_service.statu();
    let networkpolicie = apis_networking_service.networkpolicie();
    let servicecidr = apis_networking_service.servicecidr();
    let ipaddresse = apis_networking_service.ipaddresse();
    let v1beta1 = apis_networking_service.v1beta1();
    let statu = apis_networking_service.statu();
    let networking = apis_networking_service.networking();
    let apiservice = apis_apiregistration_service.apiservice();
    let statu = apis_apiregistration_service.statu();
    let v1 = apis_apiregistration_service.v1();
    let apiregistration = apis_apiregistration_service.apiregistration();
    let internal = apis_internal_service.internal();
    let statu = apis_internal_service.statu();
    let v1alpha1 = apis_internal_service.v1alpha1();
    let storageversion = apis_internal_service.storageversion();
    let log = logs_service.log();
    let v1 = apis_policy_service.v1();
    let poddisruptionbudget = apis_policy_service.poddisruptionbudget();
    let statu = apis_policy_service.statu();
    let policy = apis_policy_service.policy();
    let authorization = apis_authorization_service.authorization();
    let subjectaccessreview = apis_authorization_service.subjectaccessreview();
    let selfsubjectaccessreview = apis_authorization_service.selfsubjectaccessreview();
    let localsubjectaccessreview = apis_authorization_service.localsubjectaccessreview();
    let v1 = apis_authorization_service.v1();
    let selfsubjectrulesreview = apis_authorization_service.selfsubjectrulesreview();
    let v1 = apis_apiextensions_service.v1();
    let statu = apis_apiextensions_service.statu();
    let customresourcedefinition = apis_apiextensions_service.customresourcedefinition();
    let apiextension = apis_apiextensions_service.apiextension();
    let autoscaling = apis_autoscaling_service.autoscaling();
    let statu = apis_autoscaling_service.statu();
    let horizontalpodautoscaler = apis_autoscaling_service.horizontalpodautoscaler();
    let v1 = apis_autoscaling_service.v1();
    let v2 = apis_autoscaling_service.v2();
    let horizontalpodautoscaler = apis_autoscaling_service.horizontalpodautoscaler();
    let statu = apis_autoscaling_service.statu();
    let resize = api_service.resize();
    let statu = api_service.statu();
    let attach = api_service.attach();
    let podtemplate = api_service.podtemplate();
    let eviction = api_service.eviction();
    let exec = api_service.exec();
    let ephemeralcontainer = api_service.ephemeralcontainer();
    let service = api_service.service();
    let proxy = api_service.proxy();
    let node = api_service.node();
    let log = api_service.log();
    let serviceaccount = api_service.serviceaccount();
    let pod = api_service.pod();
    let binding = api_service.binding();
    let persistentvolume = api_service.persistentvolume();
    let limitrange = api_service.limitrange();
    let configmap = api_service.configmap();
    let event = api_service.event();
    let finalize = api_service.finalize();
    let persistentvolumeclaim = api_service.persistentvolumeclaim();
    let secret = api_service.secret();
    let portforward = api_service.portforward();
    let componentstatuse = api_service.componentstatuse();
    let scale = api_service.scale();
    let resourcequota = api_service.resourcequota();
    let namespace = api_service.namespace();
    let replicationcontroller = api_service.replicationcontroller();
    let token = api_service.token();
    let v1 = api_service.v1();
    let endpoint = api_service.endpoint();
    let api = api_service.api();
    let authentication = apis_authentication_service.authentication();
    let v1 = apis_authentication_service.v1();
    let selfsubjectreview = apis_authentication_service.selfsubjectreview();
    let tokenreview = apis_authentication_service.tokenreview();
    let admissionregistration = apis_admissionregistration_service.admissionregistration();
    let mutatingadmissionpolicie = apis_admissionregistration_service.mutatingadmissionpolicie();
    let v1beta1 = apis_admissionregistration_service.v1beta1();
    let mutatingadmissionpolicybinding = apis_admissionregistration_service.mutatingadmissionpolicybinding();
    let mutatingadmissionpolicie = apis_admissionregistration_service.mutatingadmissionpolicie();
    let mutatingadmissionpolicybinding = apis_admissionregistration_service.mutatingadmissionpolicybinding();
    let v1alpha1 = apis_admissionregistration_service.v1alpha1();
    let validatingwebhookconfiguration = apis_admissionregistration_service.validatingwebhookconfiguration();
    let validatingadmissionpolicie = apis_admissionregistration_service.validatingadmissionpolicie();
    let statu = apis_admissionregistration_service.statu();
    let v1 = apis_admissionregistration_service.v1();
    let validatingadmissionpolicybinding = apis_admissionregistration_service.validatingadmissionpolicybinding();
    let mutatingwebhookconfiguration = apis_admissionregistration_service.mutatingwebhookconfiguration();
    let coordination = apis_coordination_service.coordination();
    let leasecandidate = apis_coordination_service.leasecandidate();
    let v1alpha2 = apis_coordination_service.v1alpha2();
    let lease = apis_coordination_service.lease();
    let v1 = apis_coordination_service.v1();
    let leasecandidate = apis_coordination_service.leasecandidate();
    let v1beta1 = apis_coordination_service.v1beta1();

    Ok(())
}
```

## Service Details

### Apis_storage

**Resources:**

#### Storage

get information of a group

**Operations:**
- ✅ Read


**Fields:**


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


#### Statu

read status of the specified VolumeAttachment

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



### Apis_rbac

**Resources:**

#### Rbac.authorization

get information of a group

**Operations:**
- ✅ Read


**Fields:**


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



### Apis_events

**Resources:**

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


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Event

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


#### Statu

read status of the specified PriorityLevelConfiguration

**Operations:**
- ✅ Read
- ✅ Update


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



### Openid

**Resources:**

#### Jwk

get service account issuer OpenID JSON Web Key Set (contains public token verification keys)

**Operations:**
- ✅ Read


**Fields:**



### Apis_apps

**Resources:**

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


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified ReplicaSet

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Scale

read scale of the specified ReplicaSet

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


#### App

get information of a group

**Operations:**
- ✅ Read


**Fields:**



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


#### Scheduling

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis

**Resources:**

#### Api

get available API versions

**Operations:**
- ✅ Read


**Fields:**


#### Discovery

get information of a group

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


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**



### Apis_storagemigration

**Resources:**

#### Storagemigration

get information of a group

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified StorageVersionMigration

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**


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



### Apis_certificates

**Resources:**

#### V1alpha1

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


#### Statu

read status of the specified PodCertificateRequest

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Certificate

get information of a group

**Operations:**
- ✅ Read


**Fields:**


#### Approval

read approval of the specified CertificateSigningRequest

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


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


#### V1beta1

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


#### Statu

read status of the specified Job

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


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


#### Node

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


#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified ResourceClaim

**Operations:**
- ✅ Read
- ✅ Update


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


#### V1beta2

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


#### Resource

get information of a group

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



### Apis_networking

**Resources:**

#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


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


#### Statu

read status of the specified Ingress

**Operations:**
- ✅ Read
- ✅ Update


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


#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified ServiceCIDR

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Networking

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_apiregistration

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


#### Apiregistration

get information of a group

**Operations:**
- ✅ Read


**Fields:**



### Apis_internal

**Resources:**

#### Internal

get information of a group

**Operations:**
- ✅ Read


**Fields:**


#### Statu

read status of the specified StorageVersion

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



### Logs

**Resources:**

#### Log



**Operations:**
- ✅ Read


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


#### Policy

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


#### Subjectaccessreview

create a SubjectAccessReview

**Operations:**
- ✅ Create


**Fields:**


#### Selfsubjectaccessreview

create a SelfSubjectAccessReview

**Operations:**
- ✅ Create


**Fields:**


#### Localsubjectaccessreview

create a LocalSubjectAccessReview

**Operations:**
- ✅ Create


**Fields:**


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### Selfsubjectrulesreview

create a SelfSubjectRulesReview

**Operations:**
- ✅ Create


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


#### Apiextension

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


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


#### V2

get available resources

**Operations:**
- ✅ Read


**Fields:**


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


#### Statu

read status of the specified HorizontalPodAutoscaler

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.



### Api

**Resources:**

#### Resize

read resize of the specified Pod

**Operations:**
- ✅ Read
- ✅ Update


**Fields:**
- `dry_run` (String) - When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed
- `field_manager` (String) - fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint.
- `field_validation` (String) - fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered.


#### Statu

read status of the specified PersistentVolumeClaim

**Operations:**
- ✅ Read
- ✅ Update


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


#### Eviction

create eviction of a Pod

**Operations:**
- ✅ Create


**Fields:**


#### Exec

connect POST requests to exec of Pod

**Operations:**
- ✅ Create
- ✅ Read


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


#### Proxy

connect POST requests to proxy of Node

**Operations:**
- ✅ Create
- ✅ Read
- ✅ Update
- ✅ Delete


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


#### Log

read log of the specified Pod

**Operations:**
- ✅ Read


**Fields:**


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


#### Binding

create binding of a Pod

**Operations:**
- ✅ Create


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


#### Finalize



**Operations:**
- ✅ Update


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


#### Portforward

connect POST requests to portforward of Pod

**Operations:**
- ✅ Create
- ✅ Read


**Fields:**


#### Componentstatuse

read the specified ComponentStatus

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


#### Token

create token of a ServiceAccount

**Operations:**
- ✅ Create


**Fields:**


#### V1

get available resources

**Operations:**
- ✅ Read


**Fields:**


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


#### Api

get available API versions

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



### Apis_admissionregistration

**Resources:**

#### Admissionregistration

get information of a group

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


#### V1alpha1

get available resources

**Operations:**
- ✅ Read


**Fields:**


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



### Apis_coordination

**Resources:**

#### Coordination

get information of a group

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


#### V1alpha2

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


#### V1

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


#### V1beta1

get available resources

**Operations:**
- ✅ Read


**Fields:**





## Building from Source

If you prefer to build the provider from source:

```bash
# Clone the repository
git clone https://github.com/YOUR_ORG/hemmer-provider-k8s.git
cd hemmer-provider-k8s

# Build the provider
cargo build --release

# The binary will be at: target/release/libhemmer_k8s_provider.{so,dylib,dll}
```

## Creating a Release

This provider includes automated release workflows. To create a new release:

1. **Update the version** in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"  # Update this
   ```

2. **Commit your changes**:
   ```bash
   git add Cargo.toml
   git commit -m "chore: Bump version to 0.2.0"
   git push
   ```

3. **Create and push a tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. **Automated build process**:
   - GitHub Actions will automatically build binaries for all platforms
   - Checksums will be generated
   - A GitHub release will be created with all artifacts

5. **Release assets** will include:
   - `hemmer-provider-k8s-darwin-amd64`
   - `hemmer-provider-k8s-darwin-arm64`
   - `hemmer-provider-k8s-linux-amd64`
   - `hemmer-provider-k8s-linux-arm64`
   - `hemmer-provider-k8s-windows-amd64.exe`
   - `checksums.txt`
   - `provider.k`

The release workflow is defined in `.github/workflows/release.yml`.

## Generated Code

This provider was automatically generated from SDK metadata using the Hemmer Provider Generator.

- **Generator**: hemmer-provider-generator
- **SDK**: k8s SDK vv1
- **Services**: 26
- **Total Resources**: 184

## Contributing

To regenerate this provider with updated SDK specifications:

```bash
hemmer-provider-generator generate-unified \
  --provider k8s \
  --spec-dir /path/to/k8s-sdk \
  --output .
```

## License

Apache-2.0
