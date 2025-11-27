# ctrader-openapi-rust

[![quack-builder on crates.io](https://img.shields.io/crates/v/quack-builder.svg)](https://crates.io/crates/quack-builder)
[![quacky on crates.io](https://img.shields.io/crates/v/quacky.svg)](https://crates.io/crates/quacky)

Rust implementation for Spotware cTrader OpenAPI. This repository contains the typed Rust client API ghenerated from the cTrader OpenAPI protobuf files.

This project uses quack-builder to generate Rust code from .proto files and quacky for utilities (traits, helpers, and implementations) referenced by the generated API.

Features
--------
- Generated strongly-typed Rust API from .proto files
- Uses quacky for runtime traits and helper implementations
- Ergonomic configuration and error types

Requirements
------------
- Rust toolchain (stable preferred)

Generating the API
------------------
To regenerate Rust code from the proto files, use quack-builder. See the quack-builder documentation for details:

- quack-builder: https://crates.io/crates/quack-builder
- quacky: https://crates.io/crates/quacky

License
-------
This repository is licensed under the MIT License. See the LICENSE file for details.

Maintainer / Contact
--------------------
- GitHub: https://github.com/raul-gherman
- Issues: https://github.com/raul-gherman/ctrader-openapi-rust/issues
