# controlmylights_server

Backend server for Control My Lights. It exposes an HTTP + WebSocket API for
controlling LED strips and serves the compiled web client, which is embedded
into the binary at build time.

This crate is not published to crates.io (`publish = false`).

## Running

Configuration is read from the environment:

| Variable       | Default          | Description                        |
| -------------- | ---------------- | ---------------------------------- |
| `BIND_ADDRESS` | `127.0.0.1:3000` | Address the server listens on.     |
| `STAGE`        | `dev` / `prod`   | Selects the runtime configuration. |

In release builds `build.rs` runs `pnpm build` in `ControlMyLightsClient` and
embeds the resulting assets, so the client must be installable there.
