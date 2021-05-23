# DIA

## Set up environment

### Prerequisites

- Rust installed
- Cargo installed
- PostgreSQL database with credentials
- Redis (password required for remote instances)

### Run containers **(optional)**

If there is no database set up already, developing locally or running tests.

1. Set credentials and settings for redis and postrges in `docker-compose.yml`
2. `docker-compose up -d`
3. Update `.env` and `config.toml` later if the credentials changed

### Create `.env`

For Sqlx CLI -actions and compile time query testing. Shoud look like:

```
DATABASE_URL=postgresql://.../...
```

### Create `config.toml`

This is the main application configuration, and the only one used in production. Adjust fields accordindly

```toml
bind_to = "127.0.0.1:8080"
allow_registerations = true

[pg]
max_connections = 10
url = "postgresql://dia:dia@postgres:5432/dia"

[rd]
url = "redis://redis"

```

### Install sqlx-cli, migrations

Migrations have to be ran before building or testing, for compile-time checks.

1. `cargo install sqlx-cli`
2. `sqlx migrate run`

---

## Run locally (development)

The application is not ran in a container, since build and dependency caching is not supported (easily).

1. Set up environment as above (docker recommended)
2. Build and run the application `cargo run`

## Testing

Tests will wipe some database tables, so do not run with a database instance with important data.

1. Set up environment as above (docker recommended)
2. `cargo test`
