# Bambushain Helm Chart

This directory contains a Helm chart for deploying the Bambushain application to Kubernetes. The chart was converted from the original Kustomize templates.

## Structure

The Helm chart is organized as follows:

```
bambushain/
├── Chart.yaml           # Chart metadata
├── values.yaml          # Default values
├── templates/           # Template files
│   ├── _helpers.tpl     # Helper functions
│   ├── bamboo-config.yaml
│   ├── bamboo-deployment.yaml
│   ├── bamboo-ingress.yaml
│   ├── bamboo-service.yaml
│   ├── migrate-job.yaml
│   ├── nats-deployment.yaml
│   ├── nats-service.yaml
│   ├── postgres-deployment.yaml
│   ├── postgres-pvc.yaml
│   ├── postgres-service.yaml
│   └── sync-database-job.yaml
```

## Usage

### Installation

To install the chart with the release name `bambushain`:

```bash
helm install bambushain ./bambushain
```

### Configuration

The following table lists the configurable parameters of the Bambushain chart and their default values.

| Parameter | Description | Default |
|-----------|-------------|---------|
| `namespace` | Namespace to deploy to | `bamboo-review` |
| `branch` | Branch name for review environments | `""` |
| `image.registry` | Image registry | `registry.ulbricht.casa` |
| `image.tag` | Image tag | `4.0.0-alpha` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `bamboo.replicas` | Number of bamboo replicas | `1` |
| `bamboo.config.databaseUrl` | Database URL | `postgres://bamboo:bamboo@postgres:5432/bamboo` |
| `bamboo.config.mailer.*` | Mailer configuration | See values.yaml |
| `bamboo.config.s3.*` | S3 configuration | See values.yaml |
| `nats.replicas` | Number of NATS replicas | `1` |
| `postgres.replicas` | Number of PostgreSQL replicas | `1` |
| `postgres.persistence.size` | Size of PostgreSQL data volume | `1Gi` |
| `postgres.sync.enabled` | Enable database synchronization | `true` |
| `migrate.enabled` | Enable database migration | `true` |

### Customization

To customize the deployment, create a values file with your changes:

```yaml
# my-values.yaml
branch: feature-branch
image:
  tag: latest
```

Then install or upgrade the chart with your values:

```bash
helm install -f my-values.yaml bambushain ./bambushain
# or
helm upgrade -f my-values.yaml bambushain ./bambushain
```

## Conversion from Kustomize

This Helm chart was converted from the original Kustomize templates. The conversion process involved:

1. Creating the basic Helm chart structure
2. Converting Kustomize resources to Helm templates
3. Extracting configurable values to values.yaml
4. Adding conditionals for optional components
5. Creating helper functions for common template patterns

The chart maintains the same functionality as the original Kustomize templates, but with the added flexibility and templating capabilities of Helm.