
build:
    cargo build

update:
    rustup update
    rustup component add rustfmt
    rustup component add rust-src
    rustup component add rust-analyzer
    rustup component add clippy

install:
    cargo install --path .

cli *args: install
    kowalski analysis {{ args}}

format:
    cargo clippy --fix
    cargo fmt --all


check:
    cargo check
#    rust-analyzer diagnostics .

start:
    cargo run --bin server