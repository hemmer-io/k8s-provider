# Openid Service



**Resources**: 1

---

## Overview

The openid service provides access to 1 resource type:

- [Jwk](#jwk) [R]

---

## Resources


### Jwk

get service account issuer OpenID JSON Web Key Set (contains public token verification keys)

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

# Access jwk outputs
jwk_id = jwk.id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import kubernetes

provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create multiple jwk resources
jwk_0 = provider.openid.Jwk {
}
jwk_1 = provider.openid.Jwk {
}
jwk_2 = provider.openid.Jwk {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    jwk = provider.openid.Jwk {
    }
```

---

## Related Documentation

- [Kubernetes Openid Documentation](https://kubernetes.io/docs/reference/kubernetes-api/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
