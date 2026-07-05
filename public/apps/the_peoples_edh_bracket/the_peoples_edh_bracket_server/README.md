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

### Environment

Create a `.env` based off of [`.env.example`](./.env.example) (right next to it). The server loads these environment variables at runtime during developement - and more importantly - *at compile time during development*. This is because specifically, `DATABASE_URL` tells sqlx which database to inspect to validate the inline sql at compile time.

### Migrations

Run from the project root (where the `migrations/` folder lives):

```bash
sqlx migrate run
```

### Syncing cards to the server

In order to get every magic card synced into the server, you must run the application with the `sync-cards` options.


```bash
cargo r sync-cards
```

This will query https://scryfall.com/ for every magic card from the bulk endpoint, and then seed those cards into your local server (there's a lot of Magic: The Gathering cards, so it takes a about 10 0 15 seconds).

### Running the Server

```bash
cargo r server
```

### (Optional) Seeding debug users

To emulate an active community in your development environment, another utility script exists which is the `seed` option.

```bash
cargo r seed
```

This will seed the database with many debug users, and then simulate them rating a set of 10 cards.

## Deployment

To prepare for deployment, you need to prepare `sqlx` since during CI it won't have acess to a database.

```bash
cargo sqlx prepare
```
