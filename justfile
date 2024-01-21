BIN := "acpi-event-manager"
RELEASE_BIN_PATH := "$(pwd)/target/release"
SYMLINK_INSTALL_PATH := "/usr/local/bin"

# default recipe to display help information
default:
  @just --list

# run main script
cli ARGS: 
    cargo run -- {{ARGS}}

# compile main script
build:
    cargo build --release
    chmod +x {{RELEASE_BIN_PATH}}/{{BIN}}

# installs the command on the system
install: build
    ln -s {{RELEASE_BIN_PATH}}/{{BIN}} {{SYMLINK_INSTALL_PATH}}/{{BIN}}
    which {{BIN}}

# run all unit test in single thread
test:
    cargo test -- test-threads=1