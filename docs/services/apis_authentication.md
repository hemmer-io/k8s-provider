# Apis_authentication Service



**Resources**: 4

---

## Overview

The apis_authentication service provides access to 4 resource types:

- [Authentication](#authentication) [R]
- [Tokenreview](#tokenreview) [C]
- [V1](#v1) [R]
- [Selfsubjectreview](#selfsubjectreview) [C]

---

## Resources


### Authentication

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

# Access authentication outputs
authentication_id = authentication.id
```

---


### Tokenreview

create a TokenReview

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

# Create tokenreview
tokenreview = provider.apis_authentication.Tokenreview {
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


### Selfsubjectreview

create a SelfSubjectReview

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

# Create selfsubjectreview
selfsubjectreview = provider.apis_authentication.Selfsubjectreview {
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

# Create multiple authentication resources
authentication_0 = provider.apis_authentication.Authentication {
}
authentication_1 = provider.apis_authentication.Authentication {
}
authentication_2 = provider.apis_authentication.Authentication {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    authentication = provider.apis_authentication.Authentication {
    }
```

---

## Related Documentation

- [Kubernetes Apis_authentication Documentation](https://kubernetes.io/docs/reference/kubernetes-api/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
