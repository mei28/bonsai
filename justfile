default:
    @just --list

build:
    cargo build

check:
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo test

fmt:
    cargo fmt

lint:
    cargo clippy -- -D warnings

test *args:
    cargo test {{args}}

run *args:
    cargo run -- {{args}}

bn *args:
    cargo run --bin bn -- {{args}}

install:
    cargo install --path .
