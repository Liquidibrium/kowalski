
build:
    cargo build

release: 
    carge build --release
    
update:
    rustup update
    rustup component add rustfmt
    rustup component add rust-src
    rustup component add rust-analyzer
    rustup component add clippy

    # CLI For migration
    cargo install sqlx-cli
    
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
    
    
add_migration *name:
    sqlx migrate add -r {{ name }}
    
run_migration:
    sqlx migrate run
    
revert_migration:
    sqlx migrate revert