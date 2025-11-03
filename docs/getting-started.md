# Getting Started

This guide will help you get started using the Kubernetes provider for Hemmer.

---

## Prerequisites

- Hemmer CLI installed
- Kubernetes provider installed ([Installation Guide](installation.md))
- Kubernetes config file (~/.kube/config) or cluster access

---

## Basic Usage

### 1. Initialize Provider

Create a new Rust project and add the provider dependency:

```bash
cargo new my-kubernetes-app
cd my-kubernetes-app
```

Add to `Cargo.toml`:

```toml
[dependencies]
hemmer-kubernetes-provider = "*"
hemmer-core = "*"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"
```

### 2. Basic Example

```kcl
# main.k
import kubernetes

# Create provider instance
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create a v1
v1 = provider.apis_node.V1 {
}

```

---

## Common Patterns

### Conditional Resource Creation

```kcl
# Only create resource if condition is met
if environment == "production":
    v1 = provider.apis_node.V1 {
        # configuration
    }
```

### Referencing Resource Outputs

```kcl
# Create a resource
v1 = provider.apis_node.V1 {
    # configuration
}

# Reference its outputs
output_value = v1.id
```

---

## Available Services

This provider includes 26 services:

### 1. Apis_node

**Resources**: 3

- V1 [R]
- Runtimeclasse [CRUD]
- Node [R]

📖 [Full apis_node documentation](services/apis_node.md)

### 2. Apis_apps

**Resources**: 9

- Controllerrevision [CRUD]
- Statu [RU]
- Scale [RU]
- Deployment [CRUD]
- Replicaset [CRUD]
- Daemonset [CRUD]
- V1 [R]
- Statefulset [CRUD]
- App [R]

📖 [Full apis_apps documentation](services/apis_apps.md)

### 3. Api

**Resources**: 31

- Binding [C]
- Resourcequota [CRUD]
- Replicationcontroller [CRUD]
- Token [C]
- Proxy [CRUD]
- Exec [CR]
- Namespace [CRUD]
- Resize [RU]
- Log [R]
- Secret [CRUD]
- Componentstatuse [R]
- V1 [R]
- Persistentvolumeclaim [CRUD]
- Attach [CR]
- Portforward [CR]
- Event [CRUD]
- Podtemplate [CRUD]
- Ephemeralcontainer [RU]
- Service [CRUD]
- Endpoint [CRUD]
- Persistentvolume [CRUD]
- Statu [RU]
- Serviceaccount [CRUD]
- Configmap [CRUD]
- Finalize [U]
- Eviction [C]
- Node [CRUD]
- Scale [RU]
- Limitrange [CRUD]
- Pod [CRUD]
- Api [R]

📖 [Full api documentation](services/api.md)

### 4. Apis_scheduling

**Resources**: 3

- Priorityclasse [CRUD]
- V1 [R]
- Scheduling [R]

📖 [Full apis_scheduling documentation](services/apis_scheduling.md)

### 5. Apis_resource

**Resources**: 21

- Devicetaintrule [CRUD]
- V1alpha3 [R]
- Resourceclaimtemplate [CRUD]
- V1beta1 [R]
- Resourceclaim [CRUD]
- Statu [RU]
- Resourceslice [CRUD]
- Deviceclasse [CRUD]
- Resourceclaimtemplate [CRUD]
- Resourceslice [CRUD]
- Statu [RU]
- Resourceclaim [CRUD]
- Deviceclasse [CRUD]
- V1beta2 [R]
- Resource [R]
- Deviceclasse [CRUD]
- V1 [R]
- Resourceclaim [CRUD]
- Resourceslice [CRUD]
- Statu [RU]
- Resourceclaimtemplate [CRUD]

📖 [Full apis_resource documentation](services/apis_resource.md)

### 6. Apis_flowcontrol

**Resources**: 5

- Flowcontrol [R]
- Prioritylevelconfiguration [CRUD]
- Flowschema [CRUD]
- V1 [R]
- Statu [RU]

📖 [Full apis_flowcontrol documentation](services/apis_flowcontrol.md)

### 7. Apis_coordination

**Resources**: 7

- Coordination [R]
- Leasecandidate [CRUD]
- V1alpha2 [R]
- Lease [CRUD]
- V1 [R]
- V1beta1 [R]
- Leasecandidate [CRUD]

📖 [Full apis_coordination documentation](services/apis_coordination.md)

### 8. Apis_storagemigration

**Resources**: 4

- Storagemigration [R]
- V1beta1 [R]
- Statu [RU]
- Storageversionmigration [CRUD]

📖 [Full apis_storagemigration documentation](services/apis_storagemigration.md)

### 9. Apis

**Resources**: 4

- Api [R]
- Discovery [R]
- Endpointslice [CRUD]
- V1 [R]

📖 [Full apis documentation](services/apis.md)

### 10. Apis_events

**Resources**: 3

- Event [CRUD]
- V1 [R]
- Event [R]

📖 [Full apis_events documentation](services/apis_events.md)

### 11. Apis_apiregistration

**Resources**: 4

- Statu [RU]
- Apiservice [CRUD]
- V1 [R]
- Apiregistration [R]

📖 [Full apis_apiregistration documentation](services/apis_apiregistration.md)

### 12. Apis_batch

**Resources**: 5

- Batch [R]
- Statu [RU]
- Cronjob [CRUD]
- Job [CRUD]
- V1 [R]

📖 [Full apis_batch documentation](services/apis_batch.md)

### 13. Apis_internal

**Resources**: 4

- Internal [R]
- Statu [RU]
- V1alpha1 [R]
- Storageversion [CRUD]

📖 [Full apis_internal documentation](services/apis_internal.md)

### 14. Apis_storage

**Resources**: 11

- Storage [R]
- Csistoragecapacitie [CRUD]
- Statu [RU]
- Storageclasse [CRUD]
- Volumeattachment [CRUD]
- Csinode [CRUD]
- V1 [R]
- Volumeattributesclasse [CRUD]
- Csidriver [CRUD]
- Volumeattributesclasse [CRUD]
- V1beta1 [R]

📖 [Full apis_storage documentation](services/apis_storage.md)

### 15. Apis_apiextensions

**Resources**: 4

- V1 [R]
- Statu [RU]
- Customresourcedefinition [CRUD]
- Apiextension [R]

📖 [Full apis_apiextensions documentation](services/apis_apiextensions.md)

### 16. Apis_admissionregistration

**Resources**: 13

- Admissionregistration [R]
- Mutatingadmissionpolicybinding [CRUD]
- Mutatingadmissionpolicie [CRUD]
- V1beta1 [R]
- Mutatingadmissionpolicie [CRUD]
- Mutatingadmissionpolicybinding [CRUD]
- V1alpha1 [R]
- Validatingadmissionpolicie [CRUD]
- Mutatingwebhookconfiguration [CRUD]
- Validatingadmissionpolicybinding [CRUD]
- V1 [R]
- Statu [RU]
- Validatingwebhookconfiguration [CRUD]

📖 [Full apis_admissionregistration documentation](services/apis_admissionregistration.md)

### 17. Apis_authorization

**Resources**: 6

- Authorization [R]
- Selfsubjectaccessreview [C]
- Selfsubjectrulesreview [C]
- Subjectaccessreview [C]
- Localsubjectaccessreview [C]
- V1 [R]

📖 [Full apis_authorization documentation](services/apis_authorization.md)

### 18. Apis_networking

**Resources**: 12

- V1 [R]
- Servicecidr [CRUD]
- Ingresse [CRUD]
- Statu [RU]
- Ipaddresse [CRUD]
- Networkpolicie [CRUD]
- Ingressclasse [CRUD]
- Ipaddresse [CRUD]
- V1beta1 [R]
- Statu [RU]
- Servicecidr [CRUD]
- Networking [R]

📖 [Full apis_networking documentation](services/apis_networking.md)

### 19. Apis_certificates

**Resources**: 11

- Statu [RU]
- V1alpha1 [R]
- Podcertificaterequest [CRUD]
- Clustertrustbundle [CRUD]
- Certificate [R]
- Approval [RU]
- Certificatesigningrequest [CRUD]
- V1 [R]
- Statu [RU]
- Clustertrustbundle [CRUD]
- V1beta1 [R]

📖 [Full apis_certificates documentation](services/apis_certificates.md)

### 20. Apis_rbac

**Resources**: 6

- Rbac.authorization [R]
- Rolebinding [CRUD]
- Clusterrolebinding [CRUD]
- Clusterrole [CRUD]
- Role [CRUD]
- V1 [R]

📖 [Full apis_rbac documentation](services/apis_rbac.md)

### 21. Apis_autoscaling

**Resources**: 7

- Autoscaling [R]
- Horizontalpodautoscaler [CRUD]
- V1 [R]
- Statu [RU]
- V2 [R]
- Statu [RU]
- Horizontalpodautoscaler [CRUD]

📖 [Full apis_autoscaling documentation](services/apis_autoscaling.md)

### 22. Version

**Resources**: 1

- Version [R]

📖 [Full version documentation](services/version.md)

### 23. Apis_policy

**Resources**: 4

- Statu [RU]
- Poddisruptionbudget [CRUD]
- V1 [R]
- Policy [R]

📖 [Full apis_policy documentation](services/apis_policy.md)

### 24. Logs

**Resources**: 1

- Log [R]

📖 [Full logs documentation](services/logs.md)

### 25. Apis_authentication

**Resources**: 4

- Authentication [R]
- Tokenreview [C]
- V1 [R]
- Selfsubjectreview [C]

📖 [Full apis_authentication documentation](services/apis_authentication.md)

### 26. Openid

**Resources**: 1

- Jwk [R]

📖 [Full openid documentation](services/openid.md)


---

## Example: Complete Workflow

Here's a complete example showing a typical workflow:

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
    context = "my-cluster"
}

# Create v1
v1 = provider.apis_node.V1 {
}

# Use resource outputs
v1_id = v1.id

```

---

## Configuration

### Environment Variables

The provider uses your kubeconfig file by default:

```bash
export KUBECONFIG=~/.kube/config
```

Or configure inline:

```bash
export KUBE_HOST=https://kubernetes.example.com
export KUBE_TOKEN=your_service_account_token
```

### KCL Configuration

```kcl
# Configure provider in your KCL code
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
    context = "production-cluster"
    # Or use in-cluster configuration
}
```

---

## Next Steps

- 📚 [Service Documentation](services/) - Detailed docs for each service
- 📖 [Installation Guide](installation.md) - Installation options
- ⬅️ [Back to README](../README.md)

---

## Need Help?

- 📖 Check service-specific documentation in `docs/services/`
- 🐛 [Report issues](https://github.com/YOUR_ORG/hemmer-provider-kubernetes/issues)
- 💬 [Join discussions](https://github.com/YOUR_ORG/hemmer-provider-kubernetes/discussions)
