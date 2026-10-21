# Log output

Generates a random UUID on startup, keeps it in memory, and prints it with a UTC timestamp every 5 seconds.

Example output:

```
2026-10-21T14:15:17.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43
2026-10-21T14:15:22.705Z: 8523ecb1-c716-4cb6-a044-b9e83bb98e43
```

Written in Rust.

## Run locally

```bash
cargo run
```

## Build and run with Docker

```bash
docker build -t lastpatriot/random-string:1.1 .
docker run --rm lastpatriot/random-string:1.1
```

## Deploy to Kubernetes

```bash
kubectl create deployment log-output --image=lastpatriot/random-string:1.1
kubectl logs -f deployment/log-output
```

Image: `lastpatriot/random-string:1.1`