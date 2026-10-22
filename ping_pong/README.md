# Ping-pong

Responds to `GET /pingpong` with `pong N`, where N counts the requests received. The counter is kept in memory and resets when the app restarts.

Written in Rust (axum).

## Configuration

- `PORT`: port to listen on (default `3000`)

## Run locally

```bash
cargo run
```

Open http://localhost:3000/pingpong

## Build and run with Docker

```bash
docker build -t lastpatriot/ping-pong:1.9 .
docker run --rm -p 3000:3000 lastpatriot/ping-pong:1.9
```

## Deploy to Kubernetes

Apply the deployment and ClusterIP Service:

```bash
kubectl apply -f manifests/
```

The app shares the Log output Ingress, which routes `/pingpong` here. Apply it from the Log output folder:

```bash
kubectl apply -f ../log_output/manifests/
```

## Access the app

Open http://localhost:8081/pingpong

Image: `lastpatriot/ping-pong:1.9`