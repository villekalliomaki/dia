## DIA

### Run locally (development)

The application is not ran in a container, since build and dependency caching is not supported (easily).

1. Set up `config.toml` (use `ci-config.toml` as a base)
2. Run Postgres and Redis with `docker-compose up -d`
3. Build and run the application `cargo run`
