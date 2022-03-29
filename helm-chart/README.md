
# safe-client-gateway Helm chart

Helm chart for safe-client-gateway, deployed to GCP [Artifact Repository](https://console.cloud.google.com/artifacts/docker/clabs-gnosis-safe/us-central1/clabs-gnosis-safe/safe-client-gateway?project=clabs-gnosis-safe&supportedpurview=project) as OCI Helm chart.

## Artifact Repository Deployment

This is currently manual. Steps:

1. If you modify the helm chart, plase chege the `version` in Chart.yaml file

2. Package the chart, that generates a `.tgz` file:
```bash
$ helm package helm-chart
```

3. Login to Artifact Registry (with gcloud in project)
```bash
$ gcloud auth configure-docker us-central1-docker.pkg.dev
```

4. Push the chart `.tgz` file
```bash
$ helm push safe-client-gateway-0.1.0.tgz oci://us-central1-docker.pkg.dev/clabs-gnosis-safe/clabs-gnosis-safe
```

## Values Reference

TODO - [helm-docs](https://github.com/norwoodj/helm-docs) can be useful.
