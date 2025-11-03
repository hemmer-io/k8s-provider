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

# Create a daemonset
daemonset = provider.apis_apps.Daemonset {
}

```

---

## Common Patterns

### Conditional Resource Creation

```kcl
# Only create resource if condition is met
if environment == "production":
    daemonset = provider.apis_apps.Daemonset {
        # configuration
    }
```

### Referencing Resource Outputs

```kcl
# Create a resource
daemonset = provider.apis_apps.Daemonset {
    # configuration
}

# Reference its outputs
output_value = daemonset.id
```

---

## Available Services

This provider includes 26 services:

### 1. Apis_apps

**Resources**: 9

- Daemonset [CRUD]
- Deployment [CRUD]
- Statefulset [CRUD]
- Scale [RU]
- Controllerrevision [CRUD]
- V1 [R]
- Replicaset [CRUD]
- Statu [RU]
- App [R]

📖 [Full apis_apps documentation](services/apis_apps.md)

### 2. Apis_node

**Resources**: 3

- V1 [R]
- Runtimeclasse [CRUD]
- Node [R]

📖 [Full apis_node documentation](services/apis_node.md)

### 3. Apis_authorization

**Resources**: 6

- Authorization [R]
- Selfsubjectaccessreview [C]
- Localsubjectaccessreview [C]
- V1 [R]
- Selfsubjectrulesreview [C]
- Subjectaccessreview [C]

📖 [Full apis_authorization documentation](services/apis_authorization.md)

### 4. Logs

**Resources**: 1

- Log [R]

📖 [Full logs documentation](services/logs.md)

### 5. Apis_flowcontrol

**Resources**: 5

- Flowcontrol [R]
- Flowschema [CRUD]
- V1 [R]
- Prioritylevelconfiguration [CRUD]
- Statu [RU]

📖 [Full apis_flowcontrol documentation](services/apis_flowcontrol.md)

### 6. Openid

**Resources**: 1

- Jwk [R]

📖 [Full openid documentation](services/openid.md)

### 7. Apis_rbac

**Resources**: 6

- Rbac.authorization [R]
- Clusterrole [CRUD]
- Clusterrolebinding [CRUD]
- V1 [R]
- Role [CRUD]
- Rolebinding [CRUD]

📖 [Full apis_rbac documentation](services/apis_rbac.md)

### 8. Apis_apiextensions

**Resources**: 4

- Statu [RU]
- V1 [R]
- Customresourcedefinition [CRUD]
- Apiextension [R]

📖 [Full apis_apiextensions documentation](services/apis_apiextensions.md)

### 9. Version

**Resources**: 1

- Version [R]

📖 [Full version documentation](services/version.md)

### 10. Apis_coordination

**Resources**: 7

- Coordination [R]
- V1alpha2 [R]
- Leasecandidate [CRUD]
- Lease [CRUD]
- V1 [R]
- V1beta1 [R]
- Leasecandidate [CRUD]

📖 [Full apis_coordination documentation](services/apis_coordination.md)

### 11. Apis_policy

**Resources**: 4

- V1 [R]
- Statu [RU]
- Poddisruptionbudget [CRUD]
- Policy [R]

📖 [Full apis_policy documentation](services/apis_policy.md)

### 12. Apis_scheduling

**Resources**: 3

- Priorityclasse [CRUD]
- V1 [R]
- Scheduling [R]

📖 [Full apis_scheduling documentation](services/apis_scheduling.md)

### 13. Apis_networking

**Resources**: 12

- Statu [RU]
- Servicecidr [CRUD]
- Ingresse [CRUD]
- Networkpolicie [CRUD]
- Ingressclasse [CRUD]
- V1 [R]
- Ipaddresse [CRUD]
- Servicecidr [CRUD]
- Statu [RU]
- Ipaddresse [CRUD]
- V1beta1 [R]
- Networking [R]

📖 [Full apis_networking documentation](services/apis_networking.md)

### 14. Apis_storage

**Resources**: 11

- Storage [R]
- Statu [RU]
- Csidriver [CRUD]
- Csinode [CRUD]
- V1 [R]
- Volumeattachment [CRUD]
- Csistoragecapacitie [CRUD]
- Volumeattributesclasse [CRUD]
- Storageclasse [CRUD]
- V1beta1 [R]
- Volumeattributesclasse [CRUD]

📖 [Full apis_storage documentation](services/apis_storage.md)

### 15. Apis_apiregistration

**Resources**: 4

- Statu [RU]
- V1 [R]
- Apiservice [CRUD]
- Apiregistration [R]

📖 [Full apis_apiregistration documentation](services/apis_apiregistration.md)

### 16. Apis_admissionregistration

**Resources**: 13

- Admissionregistration [R]
- V1beta1 [R]
- Mutatingadmissionpolicie [CRUD]
- Mutatingadmissionpolicybinding [CRUD]
- Mutatingadmissionpolicie [CRUD]
- Mutatingadmissionpolicybinding [CRUD]
- V1alpha1 [R]
- Validatingadmissionpolicybinding [CRUD]
- Statu [RU]
- Mutatingwebhookconfiguration [CRUD]
- V1 [R]
- Validatingwebhookconfiguration [CRUD]
- Validatingadmissionpolicie [CRUD]

📖 [Full apis_admissionregistration documentation](services/apis_admissionregistration.md)

### 17. Apis_autoscaling

**Resources**: 7

- Autoscaling [R]
- Horizontalpodautoscaler [CRUD]
- V1 [R]
- Statu [RU]
- Horizontalpodautoscaler [CRUD]
- V2 [R]
- Statu [RU]

📖 [Full apis_autoscaling documentation](services/apis_autoscaling.md)

### 18. Apis_authentication

**Resources**: 4

- Authentication [R]
- Selfsubjectreview [C]
- Tokenreview [C]
- V1 [R]

📖 [Full apis_authentication documentation](services/apis_authentication.md)

### 19. Apis_batch

**Resources**: 5

- Batch [R]
- Cronjob [CRUD]
- Statu [RU]
- Job [CRUD]
- V1 [R]

📖 [Full apis_batch documentation](services/apis_batch.md)

### 20. Apis_events

**Resources**: 3

- V1 [R]
- Event [CRUD]
- Event [R]

📖 [Full apis_events documentation](services/apis_events.md)

### 21. Apis_resource

**Resources**: 21

- V1alpha3 [R]
- Devicetaintrule [CRUD]
- Deviceclasse [CRUD]
- Statu [RU]
- Resourceclaim [CRUD]
- V1beta1 [R]
- Resourceslice [CRUD]
- Resourceclaimtemplate [CRUD]
- Resourceclaimtemplate [CRUD]
- Resourceclaim [CRUD]
- Deviceclasse [CRUD]
- V1beta2 [R]
- Resourceslice [CRUD]
- Statu [RU]
- Resource [R]
- Deviceclasse [CRUD]
- V1 [R]
- Resourceslice [CRUD]
- Resourceclaimtemplate [CRUD]
- Statu [RU]
- Resourceclaim [CRUD]

📖 [Full apis_resource documentation](services/apis_resource.md)

### 22. Api

**Resources**: 31

- Resize [RU]
- Pod [CRUD]
- Replicationcontroller [CRUD]
- Persistentvolumeclaim [CRUD]
- Event [CRUD]
- Attach [CR]
- Finalize [U]
- Service [CRUD]
- Componentstatuse [R]
- Log [R]
- Binding [C]
- Serviceaccount [CRUD]
- Resourcequota [CRUD]
- Secret [CRUD]
- Portforward [CR]
- V1 [R]
- Eviction [C]
- Token [C]
- Configmap [CRUD]
- Proxy [CRUD]
- Scale [RU]
- Endpoint [CRUD]
- Namespace [CRUD]
- Statu [RU]
- Persistentvolume [CRUD]
- Exec [CR]
- Limitrange [CRUD]
- Ephemeralcontainer [RU]
- Podtemplate [CRUD]
- Node [CRUD]
- Api [R]

📖 [Full api documentation](services/api.md)

### 23. Apis_internal

**Resources**: 4

- Internal [R]
- Storageversion [CRUD]
- Statu [RU]
- V1alpha1 [R]

📖 [Full apis_internal documentation](services/apis_internal.md)

### 24. Apis_storagemigration

**Resources**: 4

- Storagemigration [R]
- Storageversionmigration [CRUD]
- V1beta1 [R]
- Statu [RU]

📖 [Full apis_storagemigration documentation](services/apis_storagemigration.md)

### 25. Apis

**Resources**: 4

- Api [R]
- Discovery [R]
- V1 [R]
- Endpointslice [CRUD]

📖 [Full apis documentation](services/apis.md)

### 26. Apis_certificates

**Resources**: 11

- Statu [RU]
- Clustertrustbundle [CRUD]
- Podcertificaterequest [CRUD]
- V1alpha1 [R]
- Certificate [R]
- Approval [RU]
- Statu [RU]
- V1 [R]
- Certificatesigningrequest [CRUD]
- V1beta1 [R]
- Clustertrustbundle [CRUD]

📖 [Full apis_certificates documentation](services/apis_certificates.md)


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

# Create daemonset
daemonset = provider.apis_apps.Daemonset {
}

# Use resource outputs
daemonset_id = daemonset.id

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
