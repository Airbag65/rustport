alias r := run
alias i := install


# Build PASSPORT
build:
    cargo build --release

# Run passport with given CMD
run CMD="" FLAG="" VAL="":
    cargo run {{CMD}} {{FLAG}} {{VAL}}

# Clean project
clean:
    cargo clean

# Build and install binary
install: clean build
    @git clone https://github.com/cktan/tomlc17.git
    @make -C ./tomlc17
    @cc -Wall -Wextra -o passport-update rustport-update.c ./tomlc17/src/libtomlc17.a
    @cp ./target/release/rp ~/.cargo/bin/pp
    @cp ./target/release/rp ~/.cargo/bin/passport
    @cp ./passport-update ~/.cargo/bin/passport-update
    @mkdir -p ~/.passport
    @mkdir -p ~/.portsuite
    @touch ~/.portsuite/authentication.json
    @touch ~/.passport/publicKey.pem
    @echo "" > ~/.passport/config.toml
    @echo "[global]" >> ~/.passport/config.toml
    @echo "source_path = \"$(pwd)\"" >> ~/.passport/config.toml
    @echo "ip_addr = \"127.0.0.1\"" >> ~/.passport/config.toml
    @echo '{"auth_token":"","name":"","surname":"","email":""}' > ~/.portsuite/authentication.json
    @cp ./assets/passport_title.txt ~/.passport/passport_title.txt
    @echo
    @echo "passport ( Binary: [passport | pp] ) has been installed."
    @echo "Make sure to have ~/.cargo/bin in your PATH in order to use passport"

# Update passport

update: build
    @cp ./target/release/rp ~/.cargo/bin/pp
    @cp ./target/release/rp ~/.cargo/bin/passport
    @echo
    @echo "passport ( Binary: [passport| pp] ) has been updated."
