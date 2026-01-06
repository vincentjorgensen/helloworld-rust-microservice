 Helloworld

<!-- TOC start (generated with https://github.com/derlin/bitdowntoc) -->

- [Environment variables](#environment-variables)
- [Endpoints](#endpoints)
   * [/](#)
   * [/version](#version)
   * [/healthz](#healthz)
- [Examples](#examples)
   * [Rust](#rust)
   * [Docker Compose](#docker-compose)
   * [Kubernetes](#kubernetes)

<!-- TOC end -->

A Helloworld microservice implemented in rust using express that is best
suited for testing network topologies.

Current version: `0.1.2`

dockerhub: `vincentjorgensen/rust-helloworld:0.1.2` x86_64 and arm64

<!-- TOC --><a name="environment-variables"></a>
## Environment variables

|     Variable           |  Description            | Default                 |
| ---------------------- | ----------------------- | ----------------------- |
| SERVER\_PORT           | HTTP Listen Port        | 80                      |
| SERVER\_SSL\_PORT      | HTTPS Listen Port       | None                    |
| SSL\_KEY               | Path to SSL Key         | None                    |
| SSL\_CERT              | Path to SSL Cert        | None                    |
| SERVICE\_VERSION       | Arbitary string         | 0                       |
| HOSTNAME               | Arbitary string         | localhost               |
| REGION                 | Arbitary string         | local                   |
| ZONE                   | Arbitary string         | local-0                 |

<!-- TOC --><a name="endpoints"></a>
## Endpoints

<!-- TOC --><a name=""></a>
### /

Always returns `Hello World!\n`

<!-- TOC --><a name="version"></a>
### /version

Useful if Helloworld has multiple backends. `version` can be tailored using
environment variables to indicate which backend is being hit.

Returns `version: $SERVICE_VERSION, zone: $ZONE, region: $REGION, instance: $HOSTNAME, protocol: (http|tls)\n`

<!-- TOC --><a name="healthz"></a>
### /healthz

Useful for healthcheck infrastructure

Returns `{"state": "READY"}`

<!-- TOC --><a name="examples"></a>
## Examples

<!-- TOC --><a name="rust"></a>
### Rust

If `rust` is installed, you can run it on the command line like so:
```
SSL_KEY=./key.pem SSL_CERT=./cert.pem SERVER_SSL_PORT=8443 SERVER_PORT=8080 cargo run
```

You can use the following to generate a self-signed cert.
```
openssl req -x509 -sha256 -nodes -days 365 -newkey rsa:2048 -subj '/O=any domain/CN=*' -keyout key.pem -out cert.pem
```

The service can then reached on `localhost:8080` and `localhost:8443`.

<!-- TOC --><a name="docker-compose"></a>
### Docker Compose

When used in combination with [Docker Mac Net
Connect](https://github.com/chipmk/docker-mac-net-connect), Listens on
`192.168.96.17:8080` and `192.168.96.17:8443` 

```
networks:
  default:
    driver: bridge
    external: true
    name: k3d-cluster-network
services:
  helloworld:
    container_name: helloworld
    image: vincentjorgensen/rust-helloworld:0.1.3
    networks:
      default:
        ipv4_address: 192.168.96.17
    ports:
    - "0.0.0.0:50375:8080/tcp"
    - "0.0.0.0:50376:8443/tcp"
    restart: unless-stopped
    environment:
      SERVICE_VERSION: d100
      ZONE: local-1
      REGION: local
      SERVER_PORT: 8080
      SERVER_SSL_PORT: 8443
      SSL_KEY: /ssl/root.key
      SSL_CERT: /ssl/root.crt
    healthcheck:
      interval: 30s
      retries: 3
      start_period: 15s
      start_interval: 5s
      test: ["CMD", "curl", "-s", "http://localhost:8080/healthz", "-o", "/dev/null"]
      timeout: 3s
    volumes:
    - /etc/localtime:/etc/localtime:ro
    - /PATH/TO/example/ssl:/ssl:ro
```

<!-- TOC --><a name="kubernetes"></a>
### Kubernetes

Can be used to to see how the backend distributes traffic across the topology.
Single cluster example below.

```
---
apiVersion: v1
kind: Service
metadata:
  name: helloworld
  namespace: helloworld
  labels:
    app: helloworld
    service: helloworld
spec:
  ports:
  - name: http
    port: 8001
    targetPort: 8080 ### http # named port does not work for ambient mc mesh.internal
  selector:
    app: helloworld
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: helloworld
  namespace: helloworld
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: helloworld-us-west-1a
  namespace: helloworld
  labels:
    app: helloworld
    version: v1
spec:
  replicas: 1
  selector:
    matchLabels:
      app: helloworld
      version: us-west-1a
  template:
    metadata:
      labels:
        app: helloworld
        version: us-west-1a
    spec:
      serviceAccountName: helloworld
      affinity:
        nodeAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 1
            preference:
              matchExpressions:
              - key: topology.kubernetes.io/zone
                operator: In
                values:
                - us-west-1a
      containers:
      - name: helloworld
        env:
        - name: SERVICE_VERSION
          value: v1
        - name: ZONE
          value: us-west-1a
        - name: REGION
          value: us-west-1
        - name: SERVER_PORT
          value: '8080'
        image: vincentjorgensen/rust-helloworld:0.1.0
        imagePullPolicy: IfNotPresent
        ports:
        - containerPort: 8080
          name: http
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: helloworld-us-west-1b
  namespace: helloworld
  labels:
    app: helloworld
    version: v1
spec:
  replicas: 1
  selector:
    matchLabels:
      app: helloworld
      version: us-west-1b
  template:
    metadata:
      labels:
        app: helloworld
        version: us-west-1b
    spec:
      serviceAccountName: helloworld
      affinity:
        nodeAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 1
            preference:
              matchExpressions:
              - key: topology.kubernetes.io/zone
                operator: In
                values:
                - us-west-1b
      containers:
      - name: helloworld
        env:
        - name: SERVICE_VERSION
          value: v1
        - name: ZONE
          value: us-west-1b
        - name: REGION
          value: us-west-1
        - name: SERVER_PORT
          value: '8080'
        image: vincentjorgensen/rust-helloworld:0.1.0
        imagePullPolicy: IfNotPresent
        ports:
        - containerPort: 8080
          name: http
...
```

