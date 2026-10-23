# Log output

Two containers in one pod, sharing an `emptyDir` volume:

- **writer**: generates a random UUID on startup and appends a line with a UTC timestamp and the UUID to `log.txt` every 5 seconds
- **reader**: serves the contents of `log.txt` over HTTP at `/`

Example:

```
2026-10-23T15:15:17.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43
2026-10-23T15:15:22.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43
```

Written in Rust (reader uses axum).

## Configuration

- `LOG_OUTPUT`: folder containing `log.txt` (default `/usr/src/app/files`), used by both containers
- `PORT`: reader's port (default `3000`)

## Run locally

In two terminals, with the same `LOG_OUTPUT`:

```bash
cd writer
LOG_OUTPUT=/path/to/files cargo run
```

```bash
cd reader
LOG_OUTPUT=/path/to/files cargo run
```

Open http://localhost:3000

## Build images

```bash
docker build -t lastpatriot/log-writer:1.10 writer
docker build -t lastpatriot/log-reader:1.10 reader
```

## Deploy to Kubernetes

The cluster needs port 8081 mapped to the load balancer:

```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

Apply the deployment, ClusterIP Service and Ingress:

```bash
kubectl apply -f manifests/
```

## Access the app

Open http://localhost:8081

Images: `lastpatriot/log-writer:1.10`, `lastpatriot/log-reader:1.10`