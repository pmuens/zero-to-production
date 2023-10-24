# Zero to Production

Code written while reading the book ["Zero to Production"](https://www.zero2prod.com) by Luca Palmieri.

## Setup

1. `git clone git@github.com:pmuens/zero-to-production.git`
2. `asdf install`
3. `direnv allow`
4. `./scripts/setup.sh`
5. Run `./scripts/docker_dev.sh` in terminal #1
6. Run `./scripts/init_db.sh` in terminal #2
7. Run `./scripts/dev.sh` in terminal #2

## Useful Commands

```sh
asdf install
asdf reshim rust
direnv allow

cargo init
cargo new <name> [--lib]

cargo build [--bin <binary>]
cargo clean
cargo test
cargo bench

cargo update
cargo add <name>
cargo search <name>

cargo run [--bin <binary>] [-- arg]

cargo doc [--open]
cargo fix
cargo fmt
cargo clippy
cargo check

cargo install <name>
cargo uninstall <name>

sqlx database create
sqlx migrate add <name>
sqlx migrate run
```

## Useful Resources

- [Lib.rs](https://lib.rs)
- [Rust Language Cheat Sheet](https://cheats.rs)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook)
- [Comprehensive Rust](https://google.github.io/comprehensive-rust)
- [The Rust Programming Language](https://doc.rust-lang.org/book)
- [rust-lang/rustlings](https://github.com/rust-lang/rustlings)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns)
- [Rust for Professionals](https://overexact.com/rust-for-professionals)
- [The Rustonomicon](https://doc.rust-lang.org/nightly/nomicon/)
- [The Rust Reference](https://doc.rust-lang.org/stable/reference)
- [The Little Book of Rust Books](https://lborb.github.io/book)
- [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
