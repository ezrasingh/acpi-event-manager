# ACPI Event Manager

![example branch parameter](https://github.com/ezrasingh/acpi-event-manager/actions/workflows/release.yml/badge.svg?branch=main)

> ACPI Event Manager for brightness control

This script is for Ubuntu developers like me who made the mistake of buying an HP 🙃

## Getting Started 🚀

- [Install](https://www.rust-lang.org/tools/install) `cargo`
- [Install](https://github.com/casey/just?tab=readme-ov-file#packages) `just` (`cargo install just`)

This project uses a `justfile`, to see availble commands run `just`.

To run the CLI use `cargo run -- <args>`, for example:

```shell
cargo run -- --help

Usage: acpi-keyboard-backlight [OPTIONS]

Options:
      --configure                  Installs acpi events and reloads acpi daemon
  -a, --action <ACTION>            Action from acpi event
  -c, --config-file <CONFIG_FILE>  Absolute path to config file [default: config.toml]
  -d, --debug                      Log config details
  -h, --help                       Print help
  -V, --version                    Print version
```

To compile and install the script run `just install`, this will link the `acpi-keyboard-backlight` CLI binary to your `/usr/local/bin` directory. Or if you dont want to install onto your system you can try `just cli <args>`.

## Configuration 🔧

All settings are stored in [`config.toml`](config.toml) at the project root.

| Config Flag                   | Description                                             |
| ----------------------------- | ------------------------------------------------------- |
| `acpi_device`                 | directory name of acpi device in `/sys/class/backlight` |
| `xrandr_display`              | display name in xrandr                                  |
| `brightness_increment`        | how much to change brightness on up/down                |
| `acpi_events.brightness_up`   | ACPI event code for brightness up                       |
| `acpi_events.brightness_down` | ACPI event code for brightness down                     |

For help setting up your [`config.toml`](config.toml), see the [HELPME.md](HELPME.md) doc.

## Unit Testing 🧪

To prevent test from actually modifying system files, I replicated the filesystem needed to run unit test in the `fixtures/` directory. There is also a mock `config.toml` for validating the config parser.

In order to run the entire test suite you must use a single thread. This is becuase `Rust` will by default run tests in parallel, which creates a race-condition for files in the `fixtures/` directory.

```shell
# use
cargo test -- --test-threads=1

# or
just test
```
