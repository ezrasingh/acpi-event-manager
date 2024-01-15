# ACPI Back-Light Config Manager

> This script is for Ubuntu developers like me who made the mistake of buying an HP 🙃

## Getting Started

- [Install](<(https://www.rust-lang.org/tools/install)>) `cargo` .
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

To compile and install the script run `just install`, this will link the `acpi-keyboard-backlight` CLI binary to `/usr/local/bin` directory. Or if you dont want to install onto your system you can try `just cli <args>`.

For help setting up your [`config.toml`](config.toml), continue reading.

### Listing ACPI Devices

To view availble ACPI devices check the following directory `/sys/class/backlight`. For me, it looked like this:

```shell
❯ tre /sys/class/backlight
/sys/class/backlight
└── acpi_video0
```

So in my case I would set `acpi_device = "acpi_video0"` in my [`config.toml`](config.toml)

### Listing Xrandr Displays

By default `xrandr` shows availble displays. For me, it looked like this:

```shell
❯ xrandr
Screen 0: minimum 8 x 8, current 1920 x 1080, maximum 32767 x 32767
HDMI-0 disconnected # ...
eDP-1-0 connected primary # <- we only need this part
   1920x1080    144.00*+  60.02
   1680x1050    144.00
   1280x1024    144.00
# ...
```

So in my case I would set `xrandr_display = "eDP-1-0"`

### Determine ACPI Event Codes

If unsure check by running `sudo acpi_listen` and pressing the corresponding button on your keyboard.

The corresponding event should log onto your shell. You can expect to see something like this (except the comments I added, those are only there to indicate my physical actions):

```shell
❯ sudo acpi_listen
# presses brightess down button
video/brightnessdown BRTDN 00000087 00000000

# presses brightness up button
video/brightnessup BRTUP 00000086 00000000
```

## Configuration

| Config Flag                   | Description                                             |
| ----------------------------- | ------------------------------------------------------- |
| `acpi_device`                 | directory name of acpi device in `/sys/class/backlight` |
| `xrandr_display`              | display name in xrandr                                  |
| `brightness_increment`        | how much to change brightness on up/down                |
| `acpi_events.brightness_up`   | ACPI event code for brightness up                       |
| `acpi_events.brightness_down` | ACPI event code for brightness down                     |
