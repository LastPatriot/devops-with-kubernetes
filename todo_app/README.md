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

The cluster needs port 8082 mapped to NodePort 30080:

```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

Apply the manifests:

```bash
kubectl apply -f manifests/
```

## Access the app

The app is exposed with a NodePort Service on port 30080.

Open http://localhost:8082

Image: `lastpatriot/todo-app:1.5`