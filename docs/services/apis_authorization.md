# Apis_authorization Service



**Resources**: 6

---

## Overview

The apis_authorization service provides access to 6 resource types:

- [Authorization](#authorization) [R]
- [Selfsubjectaccessreview](#selfsubjectaccessreview) [C]
- [Localsubjectaccessreview](#localsubjectaccessreview) [C]
- [V1](#v1) [R]
- [Selfsubjectrulesreview](#selfsubjectrulesreview) [C]
- [Subjectaccessreview](#subjectaccessreview) [C]

---

## Resources


### Authorization

get information of a group

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Access authorization outputs
authorization_id = authorization.id
```

---


### Selfsubjectaccessreview

create a SelfSubjectAccessReview

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create selfsubjectaccessreview
selfsubjectaccessreview = provider.apis_authorization.Selfsubjectaccessreview {
}

```

---


### Localsubjectaccessreview

create a LocalSubjectAccessReview

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create localsubjectaccessreview
localsubjectaccessreview = provider.apis_authorization.Localsubjectaccessreview {
}

```

---


### V1

get available resources

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Access v1 outputs
v1_id = v1.id
```

---


### Selfsubjectrulesreview

create a SelfSubjectRulesReview

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create selfsubjectrulesreview
selfsubjectrulesreview = provider.apis_authorization.Selfsubjectrulesreview {
}

```

---


### Subjectaccessreview

create a SubjectAccessReview

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create subjectaccessreview
subjectaccessreview = provider.apis_authorization.Subjectaccessreview {
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import kubernetes

provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create multiple authorization resources
authorization_0 = provider.apis_authorization.Authorization {
}
authorization_1 = provider.apis_authorization.Authorization {
}
authorization_2 = provider.apis_authorization.Authorization {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    authorization = provider.apis_authorization.Authorization {
    }
```

---

## Related Documentation

- [Kubernetes Apis_authorization Documentation](https://kubernetes.io/docs/reference/kubernetes-api/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
