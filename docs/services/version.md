# Version Service



**Resources**: 1

---

## Overview

The version service provides access to 1 resource type:

- [Version](#version) [R]

---

## Resources


### Version

get the version information for this server

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

# Access version outputs
version_id = version.id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import kubernetes

provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create multiple version resources
version_0 = provider.version.Version {
}
version_1 = provider.version.Version {
}
version_2 = provider.version.Version {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    version = provider.version.Version {
    }
```

---

## Related Documentation

- [Kubernetes Version Documentation](https://kubernetes.io/docs/reference/kubernetes-api/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
