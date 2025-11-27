# ctrader-openapi-rust

[![quack-builder on crates.io](https://img.shields.io/crates/v/quack-builder.svg)](https://crates.io/crates/quack-builder)
[![quacky on crates.io](https://img.shields.io/crates/v/quacky.svg)](https://crates.io/crates/quacky)

Rust implementation for Spotware cTrader OpenAPI. This repository contains the crate and codegen helpers used to generate a typed Rust client from the cTrader OpenAPI protobuf files.

This project uses quack-builder to generate Rust code from .proto files and quacky for shared runtime utilities (traits, helpers, and implementations) referenced by the generated API.

Table of contents
- What this repository is
- Features
- Requirements
- Usage
  - Generating the API
  - Using the generated client
- Examples
- Development
- Contributing
- License

What this repository is
----------------------
A Rust crate that provides a typed, async-first client for Spotware's cTrader OpenAPI. The API surface is generated from .proto definitions using quack-builder; runtime helpers and trait implementations come from the quacky crate.

Features
--------
- Generated strongly-typed Rust API from .proto files
- Uses quacky for runtime traits and helper implementations
- Async-first design (tokio compatible)
- Ergonomic configuration and error types

Requirements
------------
- Rust toolchain (stable) — minimum version: as specified in Cargo.toml
- protoc / protoc-compatible toolchain if you regenerate the .proto bindings locally
- Optional: tokio runtime to run async examples

Usage
-----
This repository contains generated sources and the crate you can depend on directly from GitHub, or you can regenerate the bindings locally using quack-builder.

Generating the API
------------------
To regenerate Rust code from the proto files, use quack-builder. See the quack-builder documentation for details:

- quack-builder: https://crates.io/crates/quack-builder
- quacky: https://crates.io/crates/quacky

A typical workflow:
1. Install protoc (the protocol buffers compiler) for your platform.
2. Add quack-builder as a dev-dependency or use it in a build script.

Example build.rs (very small sketch — adapt to your repository layout):

```rust
// build.rs
fn main() {
    // The exact API depends on quack-builder; consult its docs.
    quack_builder::Builder::new()
        .proto_dir("proto")
        .out_dir("src/generated")
        .compile()
        .expect("Failed to compile protos");
}
```

Using the generated client
--------------------------
You can use the crate directly from GitHub while the package remains unpublished:

In your Cargo.toml:

```toml
[dependencies]
ctrader-openapi-rust = { git = "https://github.com/raul-gherman/ctrader-openapi-rust.git", branch = "main" }
quacky = "*" # or a specific version from crates.io
```

Example (illustrative — adjust types to match the actual generated API):

```rust
use ctrader_openapi_rust::ClientConfig;
use quacky::runtime::HttpTransport;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = ClientConfig::builder()
        .base_url("https://demo.ctraderapi.com")
        .api_key(std::env::var("CTRADER_API_KEY").ok())
        .build();

    let client = ctrader_openapi_rust::Client::new(cfg, HttpTransport::default()).await?;

    let instruments = client.instruments().list().await?;
    println!("Instruments: {:#?}", instruments);

    Ok(())
}
```

Replace the API calls above with the actual methods produced by the generator.

Examples
--------
- examples/basic_client.rs — basic client setup and authentication
- examples/ws_stream.rs — subscribe to market updates (if WebSocket support is included)
- examples/place_order.rs — example for placing orders

Development
-----------
Run tests and linters locally:

```bash
cargo test
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

If you change the proto files, regenerate the generated sources either by running the build script or invoking quack-builder directly.

Contributing
------------
Contributions welcome. Typical workflow:
1. Fork the repository
2. Create a feature branch
3. Add tests and documentation
4. Open a pull request

Please keep API compatibility in mind and update examples and README when adding or changing features.

License
-------
This repository is licensed under the MIT License. See the LICENSE file for details.

Maintainer / Contact
--------------------
- GitHub: https://github.com/raul-gherman
- Issues: https://github.com/raul-gherman/ctrader-openapi-rust/issues
