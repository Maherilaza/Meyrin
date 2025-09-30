# Meyrin

Meyrin is a lightweight example application that demonstrates how to build a static-file web server with [Actix Web](https://actix.rs/). It starts a single Actix instance, serves the contents of the [`static/`](static/) directory, and falls back to a custom 404 page, making it a handy starting point for experiments or small internal tools.

## Table of contents
- [Features](#features)
- [Quick start](#quick-start)
  - [Running locally](#running-locally)
  - [Command-line options](#command-line-options)
- [Project layout](#project-layout)
- [Development workflow](#development-workflow)
  - [Prerequisites](#prerequisites)
  - [Common tasks](#common-tasks)
- [Serving your own content](#serving-your-own-content)
- [Additional resources](#additional-resources)

## Features
- Thin Actix Web wrapper with a ready-to-use `HttpServer` configuration.
- Static asset handling with sensible defaults for `/` and `/index` routes.
- Custom 404 handler powered by the bundled HTML template in [`.config/404.html`](.config/404.html).
- Structured logging configured through `env_logger`.
- Minimal CLI interface for selecting host and port.

## Quick start

### Running locally
1. **Clone the repository**
   ```bash
   git clone https://github.com/joaoviictorti/Meyrin.git
   cd Meyrin
   ```
2. **Launch the server**
   ```bash
   cargo run -- --ip 127.0.0.1 --port 8080
   ```
3. **Open the demo site** by navigating to [http://127.0.0.1:8080](http://127.0.0.1:8080). You should see the HTML page served from [`static/index.html`](static/index.html).

### Command-line options
The binary exposes a small CLI to control runtime behaviour:

```text
meyrin [OPTIONS]

Options:
  -p, --port <PORT>    TCP port to bind the server to (default: 8080)
  -i, --ip <IP>        IP address/interface to listen on (default: 127.0.0.1)
  -V, --version        Print version information
  -h, --help           Print help
```

If no flags are supplied, Meyrin will bind to `127.0.0.1:8080`.

## Project layout

```text
.
├── src/                # Rust sources for the Actix server
├── static/             # Default HTML, CSS, and JS assets served at runtime
├── .config/404.html    # Template used by the custom 404 handler
├── Cargo.toml          # Crate metadata and dependencies
├── Makefile.toml       # cargo-make tasks for building with Zig
└── Readme.md           # Project documentation (this file)
```

## Development workflow

### Prerequisites

Install the standard Rust toolchain plus the optional helpers used by the automation scripts:

```bash
cargo install cargo-make
cargo install --locked cargo-zigbuild
```

> **Note:** These dependencies are only required if you plan to use the Zig-based optimized builds described below. Running `cargo run` or `cargo check` works with the standard toolchain.

### Common tasks

| Task | Command | Description |
| ---- | ------- | ----------- |
| Run the development server | `cargo run` | Starts Meyrin with default options for local testing. |
| Lint and format | `cargo fmt` | Formats the Rust sources using `rustfmt`. |
| Check for compilation errors | `cargo check` | Quickly verifies the crate compiles without producing a binary. |
| Release build with Zig | `cargo make optimize` | Produces an optimized binary through `cargo-zigbuild`.

## Serving your own content

The default routes point to files inside [`static/`](static/). Replace `index.html`, `style.css`, or `script.js` with your own assets, or add new files alongside them. Because the server uses Actix's `Files::new` service, any additional assets in that directory are automatically available at runtime.

To customize the not-found experience, edit the HTML snippet in [`.config/404.html`](.config/404.html). The [`activate_404_handler!`](src/actix.rs) macro wires this template into the Actix configuration.

## Additional resources

For more advanced use cases—such as authentication, websockets, or deployment topologies—refer directly to the official [Actix Web documentation](https://actix.rs/docs/).

