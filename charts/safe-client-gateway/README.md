# Safe Client Gateway Chart

This chart packages the Safe Client Gateway resources. The chart assumes that there is already an existing Redis instance available and connection attribute should be passed in the values of the Helm Chart

## Parameters

### Common parameters

| Name               | Description                                        | Value |
| ------------------ | -------------------------------------------------- | ----- |
| `nameOverride`     | String to partially override common.names.fullname | `""`  |
| `fullnameOverride` | String to fully override common.names.fullname     | `""`  |

### Installation Parameters

| Name                       | Description                                                      | Value                               |
| -------------------------- | ---------------------------------------------------------------- | ----------------------------------- |
| `replicas`                 | Replicas for deployment                                          | `1`                                 |
| `strategy`                 | Strategy for deployment                                          | `Recreate`                          |
| `commonLabels`             | Labels to add to all related objects                             | `{}`                                |
| `commonAnnotations`        | Annotations to to all related objects                            | `{}`                                |
| `ingress.ingressClassName` | Name of the ingress class name to be used                        | `""`                                |
| `ingress.hostname`         | Default host for the ingress record                              | `safe-client-gateway.cluster.local` |
| `ingress.annotations`      | Annotations to be added to ingress resources                     | `{}`                                |
| `nodeSelector`             | Object containing node selection constraint to deployment        | `{}`                                |
| `resources`                | Resource specification to deployment                             | `{}`                                |
| `tolerations`              | Tolerations specifications to deployment                         | `[]`                                |
| `affinity`                 | Affinity specifications to deployment                            | `{}`                                |
| `image.registry`           | Docker registry to deployment                                    | `registry.hub.docker.com`           |
| `image.repository`         | Docker image repository to deployment                            | `safeglobal/safe-client-gateway`    |
| `image.tag`                | Docker image tag to deployment                                   | `""`                                |
| `image.pullPolicy`         | Pull policy to deployment as deinfed in                          | `IfNotPresent`                      |
| `service.type`             | service type                                                     | `ClusterIP`                         |
| `service.ports.number`     | service port number                                              | `80`                                |
| `service.ports.name`       | service port name                                                | `api`                               |
| `service.sessionAffinity`  | Control where client requests go, to the same pod or round-robin | `None`                              |

### Config Service Parameters

| Name                                     | Description                                                                                                                        | Value                             |
| ---------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| `config.configServiceEndpoint`           | The Base URL of the Safe Config Service                                                                                            | `https://safe.cluster.local/cfg/` |
| `config.secretKey`                       | Rocket Secret Key.                                                                                                                 | `""`                              |
| `config.webToken`                        | Client Gateway Web Token                                                                                                           | `""`                              |
| `config.transactionServiceToken`         | Client Gateway Web Token                                                                                                           | `""`                              |
| `config.secretReferenceKey`              | Reference to an existing secret containing the following entries: ROCKET_SECRET_KEY, WEBHOOK_TOKEN, TRANSACTION_SERVICE_AUTH_TOKEN | `""`                              |
| `config.extraEnvVars`                    | Add additional extra environment vairables to the configMap                                                                        | `{}`                              |
| `config.logLevel`                        | Allowed hosts                                                                                                                      | `normal`                          |
| `config.redis.secretReferenceKey`        | Reference to an existing secret containing the following entries: REDIS_URI                                                        | `""`                              |
| `config.redis.password`                  | Redis user's password                                                                                                              | `""`                              |
| `config.redis.host`                      | Redis server host                                                                                                                  | `""`                              |
| `config.redis.port`                      | Redis server port                                                                                                                  | `6379`                            |
| `config.redisMainnet.secretReferenceKey` | Reference to an existing secret containing the following entries: REDIS_URI_MAINNET                                                | `""`                              |
| `config.redisMainnet.password`           | Redis user's password                                                                                                              | `""`                              |
| `config.redisMainnet.host`               | Redis server host                                                                                                                  | `""`                              |
| `config.redisMainnet.port`               | Redis server port                                                                                                                  | `6379`                            |
