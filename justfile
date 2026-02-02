alias r := run


# Build RUSTPORT
build:
    cargo build --release

# Run RUSTPORT with given CMD
run CMD="" FLAG="" VAL="":
    cargo run {{CMD}} {{FLAG}} {{VAL}}

# Clean project
clean:
    cargo clean

# Build and install binary
install: build
    @cp ./target/release/rustport ~/.cargo/bin/rustport
    @echo "RUSTPORT has been installed."
    @echo "Make sure to have ~/.cargo/bin in your PATH in order to use RUSTPORT"
