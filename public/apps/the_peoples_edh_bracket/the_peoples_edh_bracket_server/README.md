# the_peoples_edh_bracket_server

Rust backend for The People's EDH Bracket, built with Axum, SQLx, and Tokio.

## Local Development

### Prerequisites

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- Rust toolchain (`rustup`)
- [`sqlx-cli`](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli): `cargo install sqlx-cli`

### Services

The local dev environment is managed with Docker Compose from the project root.

```bash
cd ..
docker compose up -d
```

This starts:

| Service  | URL                   | Credentials                      |
| -------- | --------------------- | -------------------------------- |
| Postgres | `localhost:5432`      | `admin` / `root`, db: `db`       |
| pgAdmin  | http://localhost:5050 | No login — opens in desktop mode |

pgAdmin is pre-configured with a **Local Postgres** server entry. When connecting for the first time, enter `root` as the password.

To stop and remove all containers:

```bash
docker compose down
```

### Migrations

Run from the project root (where the `migrations/` folder lives):

```bash
sqlx migrate run --database-url postgres://admin:root@localhost:5432/db
```

### Running the Server

```bash
cargo run -p the_peoples_edh_bracket_server
```
