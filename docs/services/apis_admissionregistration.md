# Apis_admissionregistration Service



**Resources**: 13

---

## Overview

The apis_admissionregistration service provides access to 13 resource types:

- [Admissionregistration](#admissionregistration) [R]
- [Mutatingadmissionpolicybinding](#mutatingadmissionpolicybinding) [CRUD]
- [Mutatingadmissionpolicie](#mutatingadmissionpolicie) [CRUD]
- [V1beta1](#v1beta1) [R]
- [Mutatingadmissionpolicie](#mutatingadmissionpolicie) [CRUD]
- [Mutatingadmissionpolicybinding](#mutatingadmissionpolicybinding) [CRUD]
- [V1alpha1](#v1alpha1) [R]
- [Validatingadmissionpolicie](#validatingadmissionpolicie) [CRUD]
- [Mutatingwebhookconfiguration](#mutatingwebhookconfiguration) [CRUD]
- [Validatingadmissionpolicybinding](#validatingadmissionpolicybinding) [CRUD]
- [V1](#v1) [R]
- [Statu](#statu) [RU]
- [Validatingwebhookconfiguration](#validatingwebhookconfiguration) [CRUD]

---

## Resources


### Admissionregistration

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

# Access admissionregistration outputs
admissionregistration_id = admissionregistration.id
```

---


### Mutatingadmissionpolicybinding

create a MutatingAdmissionPolicyBinding

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

# Create mutatingadmissionpolicybinding
mutatingadmissionpolicybinding = provider.apis_admissionregistration.Mutatingadmissionpolicybinding {
}

# Access mutatingadmissionpolicybinding outputs
mutatingadmissionpolicybinding_id = mutatingadmissionpolicybinding.id
```

---


### Mutatingadmissionpolicie

create a MutatingAdmissionPolicy

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

# Create mutatingadmissionpolicie
mutatingadmissionpolicie = provider.apis_admissionregistration.Mutatingadmissionpolicie {
}

# Access mutatingadmissionpolicie outputs
mutatingadmissionpolicie_id = mutatingadmissionpolicie.id
```

---


### V1beta1

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

# Access v1beta1 outputs
v1beta1_id = v1beta1.id
```

---


### Mutatingadmissionpolicie

create a MutatingAdmissionPolicy

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

# Create mutatingadmissionpolicie
mutatingadmissionpolicie = provider.apis_admissionregistration.Mutatingadmissionpolicie {
}

# Access mutatingadmissionpolicie outputs
mutatingadmissionpolicie_id = mutatingadmissionpolicie.id
```

---


### Mutatingadmissionpolicybinding

create a MutatingAdmissionPolicyBinding

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

# Create mutatingadmissionpolicybinding
mutatingadmissionpolicybinding = provider.apis_admissionregistration.Mutatingadmissionpolicybinding {
}

# Access mutatingadmissionpolicybinding outputs
mutatingadmissionpolicybinding_id = mutatingadmissionpolicybinding.id
```

---


### V1alpha1

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

# Access v1alpha1 outputs
v1alpha1_id = v1alpha1.id
```

---


### Validatingadmissionpolicie

create a ValidatingAdmissionPolicy

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

# Create validatingadmissionpolicie
validatingadmissionpolicie = provider.apis_admissionregistration.Validatingadmissionpolicie {
}

# Access validatingadmissionpolicie outputs
validatingadmissionpolicie_id = validatingadmissionpolicie.id
```

---


### Mutatingwebhookconfiguration

create a MutatingWebhookConfiguration

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

# Create mutatingwebhookconfiguration
mutatingwebhookconfiguration = provider.apis_admissionregistration.Mutatingwebhookconfiguration {
}

# Access mutatingwebhookconfiguration outputs
mutatingwebhookconfiguration_id = mutatingwebhookconfiguration.id
```

---


### Validatingadmissionpolicybinding

create a ValidatingAdmissionPolicyBinding

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

# Create validatingadmissionpolicybinding
validatingadmissionpolicybinding = provider.apis_admissionregistration.Validatingadmissionpolicybinding {
}

# Access validatingadmissionpolicybinding outputs
validatingadmissionpolicybinding_id = validatingadmissionpolicybinding.id
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


### Statu

read status of the specified ValidatingAdmissionPolicy

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


### Validatingwebhookconfiguration

create a ValidatingWebhookConfiguration

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

# Create validatingwebhookconfiguration
validatingwebhookconfiguration = provider.apis_admissionregistration.Validatingwebhookconfiguration {
}

# Access validatingwebhookconfiguration outputs
validatingwebhookconfiguration_id = validatingwebhookconfiguration.id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import kubernetes

provider = kubernetes.KubernetesProvider {
    kubeconfig = "~/.kube/config"
}

# Create multiple admissionregistration resources
admissionregistration_0 = provider.apis_admissionregistration.Admissionregistration {
}
admissionregistration_1 = provider.apis_admissionregistration.Admissionregistration {
}
admissionregistration_2 = provider.apis_admissionregistration.Admissionregistration {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    admissionregistration = provider.apis_admissionregistration.Admissionregistration {
    }
```

---

## Related Documentation

- [Kubernetes Apis_admissionregistration Documentation](https://kubernetes.io/docs/reference/kubernetes-api/)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
