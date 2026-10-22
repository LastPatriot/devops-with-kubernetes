# Log output

Generates a random UUID on startup and keeps it in memory.

- Prints the UUID with a UTC timestamp every 5 seconds
- Serves the current timestamp and the UUID over HTTP at `/`

Example:

```
2026-10-22T13:15:17.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43
```

Written in Rust (axum).

## Configuration

- `PORT`: port to listen on (default `3000`)

## Run locally

```bash
cargo run
```

Open http://localhost:3000

## Build and run with Docker

```bash
docker build -t lastpatriot/random-string:1.7 .
docker run --rm -p 3000:3000 lastpatriot/random-string:1.7
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

Image: `lastpatriot/random-string:1.7`