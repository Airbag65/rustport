alias r := run
alias i := install


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
    @cp ./target/release/rp ~/.cargo/bin/rp
    @cp ./target/release/rp ~/.cargo/bin/rustport
    @cp ./assets/rustport_title.txt ~/.passport/rustport_title.txt
    @echo
    @echo "RUSTPORT ( Binary: [rustport | rp] ) has been installed."
    @echo "Make sure to have ~/.cargo/bin in your PATH in order to use RUSTPORT"
