#!/bin/sh

cargo install cargo-watch
cargo install cargo-udeps
cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres

asdf reshim rust
