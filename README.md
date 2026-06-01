# Boiled Crab — Clean Architecture Axum API

Professional README with clear setup, migration, build, and deployment instructions.

## Summary

Boiled Crab is a minimal REST API scaffold demonstrating Clean/Onion Architecture using Axum, SeaORM, and JWT-based authentication. This repository is focused on a secure, testable structure suitable as a starting point for production services.

Supported features
- Clean architecture layering (presentation → application → infrastructure → domain)
- JWT authentication with configurable algorithm and expiration
- SeaORM-based repository adapter (MySQL)
- Input validation using `validator`
- CORS and basic rate-limiting middleware

## Prerequisites

Install required tools:

```bash
rustup update stable
cargo --version
mysql 8.0+
docker (optional, for containerized deployments)
```

## Configuration

Copy the example env and edit values for your environment:

```bash
cp .env.example .env
# Edit .env with production values (DB, JWT_SECRET, JWT_ALGORITHM, APP_HOST, APP_PORT)
```

Important production variables:
- `APP_ENV=production`
- `DATABASE_URL` or `DB_*` variables
- `JWT_SECRET` (must be set and sufficiently long)
- `JWT_ALGORITHM` (e.g. `HS256`, `RS256`)
- `APP_ALLOWED_WEB` (CORS allowlist) — ensure valid origins

## Migrations

This project includes SQL migrations under `migration/`.

Manual (MySQL):

```bash
# create database
mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS boiled_crab CHARACTER SET utf8mb4;"

# apply migration (example)
mysql -u root -p boiled_crab < migration/sql/001_create_users_table.sql
```

If you adopt a migration tool (SeaORM migrator, sqlx-cli, or Flyway), prefer using a single `DATABASE_URL` env var and run migrations in CI/CD before deployment.

## Run (Development)

```bash
cargo run
# Server defaults to http://127.0.0.1:3000
```

Run with a specific environment file:

```bash
APP_ENV=development cargo run
```

## Build (Production)

```bash
cargo build --release
# Run the optimized binary
./target/release/boiled_crab
```

## Docker (recommended for production)

Create a `Dockerfile` (example):

```dockerfile
FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/boiled_crab /usr/local/bin/boiled_crab
ENV RUST_LOG=info
EXPOSE 3000
CMD ["/usr/local/bin/boiled_crab"]
```

Build and run:

```bash
docker build -t boiled_crab:latest .
docker run --env-file .env -p 3000:3000 boiled_crab:latest
```

## Deployment Notes

- Use a process supervisor (systemd) or orchestrator (Kubernetes) to run the binary.
- Prefer TLS termination at the edge (load balancer / reverse proxy) or provide certs and enable TLS in the service.
- Inject secrets with a secret manager (Vault, AWS Secrets Manager, Kubernetes Secrets) — do not store production secrets in `.env`.
- Run database migrations as a separate CI/CD step before rolling updates.
- Use rolling updates and health checks to avoid downtime.

## Healthchecks & Monitoring

- Provide `/health` endpoint (already present) for liveness and readiness probes.
- Export metrics (Prometheus) and traces (OpenTelemetry) in production for observability.

## Testing

Run unit and integration tests locally:

```bash
cargo test
```

For integration tests that need a DB, use a dedicated test database or a test container (Docker Compose).

## Security Checklist (Before Production)

- Ensure `JWT_SECRET` and `JWT_ALGORITHM` are configured and validated at startup.
- Use native DB UUID types and validate persisted IDs.
- Replace any remaining `unwrap()` calls.
- Harden CORS and validate `APP_ALLOWED_WEB`.
- Use rate limiting and account lockouts for authentication endpoints.
- Enable structured logging and avoid exposing internal errors to clients.

## Contributing

Please follow the contribution guide: create branches, add tests, run `cargo fmt` and `cargo clippy`, and open a PR.

## License

MIT. See the `LICENSE` file for details.

---

If you'd like, I can also add a `Dockerfile`, a `Makefile` for common tasks, and a CI workflow for build/tests/migrations. Which should I add next?

