# Todo app

Web server for the course project. Serves a simple HTML page at `/` and prints `Server started in port NNNN` on startup.

Written in Rust (axum).

## Configuration

- `PORT`: port to listen on (default `3000`)

## Run locally

```bash
PORT=8080 cargo run
```

Open http://localhost:8080

## Build and run with Docker

```bash
docker build -t lastpatriot/todo-app:1.5 .
docker run --rm -e PORT=8080 -p 8080:8080 lastpatriot/todo-app:1.5
```

## Deploy to Kubernetes

```bash
kubectl apply -f manifests/deployment.yaml
kubectl logs deployment/todo-app
```

## Access the app

```bash
kubectl get pods
kubectl port-forward todo-app-xxxxxxxx-xxxx 3003:3000
```

Open http://localhost:3003

Image: `lastpatriot/todo-app:1.5`