# Todo app

Web server for the course project. Prints `Server started in port NNNN` on startup.

Written in Rust (axum).

## Configuration

- `PORT`: port to listen on (default `3000`)

## Run locally

```bash
   PORT=8080 cargo run
```

## Build and run with Docker

```bash
   docker build -t lastpatriot/todo-app:1.2 .
   docker run --rm -e PORT=8080 -p 8080:8080 lastpatriot/todo-app:1.2
```

## Deploy to Kubernetes

```bash
   kubectl create deployment todo-app --image=lastpatriot/todo-app:1.2
   kubectl logs deployment/todo-app
```

Image: `lastpatriot/todo-app:1.2`