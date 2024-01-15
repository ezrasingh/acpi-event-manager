RELEASE_BIN_PATH := "$(pwd)/target/release/acpi-keyboard-backlight"
SYMLINK_INSTALL_PATH := "/usr/local/bin/acpi-keyboard-backlight"

# default recipe to display help information
default:
  @just --list

# run main script
cli ARGS: 
    cargo run -- {{ARGS}}

# compile main script
build:
    cargo build --release
    chmod +x {{RELEASE_BIN_PATH}}

# installs the command on the system
install: build
    rm -f {{SYMLINK_INSTALL_PATH}}
    ln -s {{RELEASE_BIN_PATH}} {{SYMLINK_INSTALL_PATH}}
    which acpi-keyboard-backlight