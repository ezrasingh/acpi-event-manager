# Tooling

- [`Clap`](https://docs.rs/clap/4.4.17/clap/) - Command Line Argument Parser for Rust
- [`Serde`](https://docs.rs/serde/1.0.195/serde/) - Serialization framework for Rust
- [`Toml`](https://docs.rs/toml/0.8.8/toml/) - A serde-compatible TOML-parsing library

# Project Structure

```shell
src
├── acpi.rs # ACPI Facade interface
├── backlight.rs # Backlight controller
├── config.rs # Config parser
└── main.rs # CLI entrypoint
```

Info on unit testing can be found in the [README](README.md#unit-testing-)
