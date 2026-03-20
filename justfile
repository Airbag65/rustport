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
install: clean build
    @git clone https://github.com/cktan/tomlc17.git
    @make -C ./tomlc17
    @cc -Wall -Wextra -o rustport-update rustport-update.c ./tomlc17/src/libtomlc17.a
    @cp ./target/release/rp ~/.cargo/bin/rp
    @cp ./target/release/rp ~/.cargo/bin/rustport
    @cp ./rustport-update ~/.cargo/bin/rustport-update
    @mkdir -p ~/.passport
    @touch ~/.passport/authentication.json
    @touch ~/.passport/publicKey.pem
    @echo "" > ~/.passport/config.toml
    @echo "[global]" >> ~/.passport/config.toml
    @echo "source_path = \"$(pwd)\"" >> ~/.passport/config.toml
    @echo "ip_addr = \"127.0.0.1\"" >> ~/.passport/config.toml
    @echo '{"auth_token":"","name":"","surname":"","email":""}' > ~/.passport/authentication.json
    @cp ./assets/rustport_title.txt ~/.passport/rustport_title.txt
    @echo
    @echo "RUSTPORT ( Binary: [rustport | rp] ) has been installed."
    @echo "Make sure to have ~/.cargo/bin in your PATH in order to use RUSTPORT"

# Update rustport
update: build
    @cp ./target/release/rp ~/.cargo/bin/rp
    @cp ./target/release/rp ~/.cargo/bin/rustport
    @echo
    @echo "RUSTPORT ( Binary: [rustport | rp] ) has been updated."
