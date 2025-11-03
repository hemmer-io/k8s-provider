# Apis_apiregistration Service



**Resources**: 4

---

## Overview

The apis_apiregistration service provides access to 4 resource types:

- [Statu](#statu) [RU]
- [Apiservice](#apiservice) [CRUD]
- [V1](#v1) [R]
- [Apiregistration](#apiregistration) [R]

---

## Resources


### Statu

read status of the specified APIService

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | String |  | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |
| `field_manager` | String |  | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |
| `field_validation` | String |  | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |



#### Usage Example

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Access statu outputs
statu_id = statu.id
```

---


### Apiservice

create an APIService

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dry_run` | String |  | When present, indicates that modifications should not be persisted. An invalid or unrecognized dryRun directive will result in an error response and no further processing of the request. Valid values are: - All: all dry run stages will be processed |
| `field_manager` | String |  | fieldManager is a name associated with the actor or entity that is making these changes. The value must be less than or 128 characters long, and only contain printable characters, as defined by https://golang.org/pkg/unicode/#IsPrint. |
| `field_validation` | String |  | fieldValidation instructs the server on how to handle objects in the request (POST/PUT/PATCH) containing unknown or duplicate fields. Valid values are: - Ignore: This will ignore any unknown fields that are silently dropped from the object, and will ignore all but the last duplicate field that the decoder encounters. This is the default behavior prior to v1.23. - Warn: This will send a warning via the standard warning response header for each unknown field that is dropped from the object, and for each duplicate field that is encountered. The request will still succeed if there are no other errors, and will only persist the last of any duplicate fields. This is the default in v1.23+ - Strict: This will fail the request with a BadRequest error if any unknown fields would be dropped from the object, or if any duplicate fields are present. The error returned from the server will contain all unknown and duplicate fields encountered. |



#### Usage Example

```kcl
# main.k
import kubernetes

# Initialize provider
provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create apiservice
apiservice = provider.apis_apiregistration.Apiservice {
}

# Access apiservice outputs
apiservice_id = apiservice.id
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


### Apiregistration

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

# Access apiregistration outputs
apiregistration_id = apiregistration.id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import kubernetes

provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create multiple statu resources
statu_0 = provider.apis_apiregistration.Statu {
}
statu_1 = provider.apis_apiregistration.Statu {
}
statu_2 = provider.apis_apiregistration.Statu {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    statu = provider.apis_apiregistration.Statu {
    }
```

---

## Related Documentation

- [Kubernetes Apis_apiregistration Documentation](https://kubernetes.io/docs/reference/kubernetes-api/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
