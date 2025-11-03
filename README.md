# Kubernetes Provider for Hemmer

**Auto-generated KUBERNETES provider with 26 services and 184 resources**

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

---

## Installation

### Using Hemmer CLI (Recommended)

```bash
hemmer provider install kubernetes
```

### Manual Installation

Download the latest release for your platform from the [Releases](../../releases) page.

📖 **[Detailed installation instructions](docs/installation.md)**

---

## Quick Start

```rust
use hemmer_kubernetes_provider::KubernetesProvider;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let provider = KubernetesProvider::new().await?;

    // Access service
    let apis_apps_service = provider.apis_apps();

    // Use resources
    let daemonset = apis_apps_service.daemonset();

    Ok(())
}
```

📖 **[Getting started guide](docs/getting-started.md)**

---

## Services

This provider includes the following services:

| Service | Resources | Documentation |
|---------|-----------|---------------|
| apis_apps | 9 | [docs/services/apis_apps.md](docs/services/apis_apps.md) |
| apis_node | 3 | [docs/services/apis_node.md](docs/services/apis_node.md) |
| apis_authorization | 6 | [docs/services/apis_authorization.md](docs/services/apis_authorization.md) |
| logs | 1 | [docs/services/logs.md](docs/services/logs.md) |
| apis_flowcontrol | 5 | [docs/services/apis_flowcontrol.md](docs/services/apis_flowcontrol.md) |
| openid | 1 | [docs/services/openid.md](docs/services/openid.md) |
| apis_rbac | 6 | [docs/services/apis_rbac.md](docs/services/apis_rbac.md) |
| apis_apiextensions | 4 | [docs/services/apis_apiextensions.md](docs/services/apis_apiextensions.md) |
| version | 1 | [docs/services/version.md](docs/services/version.md) |
| apis_coordination | 7 | [docs/services/apis_coordination.md](docs/services/apis_coordination.md) |
| apis_policy | 4 | [docs/services/apis_policy.md](docs/services/apis_policy.md) |
| apis_scheduling | 3 | [docs/services/apis_scheduling.md](docs/services/apis_scheduling.md) |
| apis_networking | 12 | [docs/services/apis_networking.md](docs/services/apis_networking.md) |
| apis_storage | 11 | [docs/services/apis_storage.md](docs/services/apis_storage.md) |
| apis_apiregistration | 4 | [docs/services/apis_apiregistration.md](docs/services/apis_apiregistration.md) |
| apis_admissionregistration | 13 | [docs/services/apis_admissionregistration.md](docs/services/apis_admissionregistration.md) |
| apis_autoscaling | 7 | [docs/services/apis_autoscaling.md](docs/services/apis_autoscaling.md) |
| apis_authentication | 4 | [docs/services/apis_authentication.md](docs/services/apis_authentication.md) |
| apis_batch | 5 | [docs/services/apis_batch.md](docs/services/apis_batch.md) |
| apis_events | 3 | [docs/services/apis_events.md](docs/services/apis_events.md) |
| apis_resource | 21 | [docs/services/apis_resource.md](docs/services/apis_resource.md) |
| api | 31 | [docs/services/api.md](docs/services/api.md) |
| apis_internal | 4 | [docs/services/apis_internal.md](docs/services/apis_internal.md) |
| apis_storagemigration | 4 | [docs/services/apis_storagemigration.md](docs/services/apis_storagemigration.md) |
| apis | 4 | [docs/services/apis.md](docs/services/apis.md) |
| apis_certificates | 11 | [docs/services/apis_certificates.md](docs/services/apis_certificates.md) |

---

## Documentation

- 📖 [Installation Guide](docs/installation.md)
- 🚀 [Getting Started](docs/getting-started.md)
- 📚 [Service Documentation](docs/services/)

---

## Building from Source

```bash
git clone https://github.com/YOUR_ORG/hemmer-provider-kubernetes.git
cd hemmer-provider-kubernetes
cargo build --release
```

The binary will be at: `target/release/libhemmer_kubernetes_provider.{so,dylib,dll}`

---

## Creating a Release

This provider includes automated release workflows.

1. Update version in `Cargo.toml`
2. Commit and push changes
3. Create and push a tag: `git tag v0.2.0 && git push origin v0.2.0`
4. GitHub Actions will automatically build and publish the release

📖 See [Release Workflow](.github/workflows/release.yml) for details

---

## Generated Code

This provider was automatically generated using the Hemmer Provider Generator.

- **Generator**: [hemmer-provider-generator](https://github.com/hemmer-io/hemmer-provider-generator) v0.3.3
- **Provider**: KUBERNETES
- **SDK Version**: v1
- **Services**: 26
- **Total Resources**: 184
- **Generated**: 2025-11-03

To regenerate this provider:

```bash
hemmer-provider-generator generate-unified \
  --provider kubernetes \
  --spec-dir /path/to/kubernetes-sdk \
  --output .
```

---

## License

Apache-2.0
