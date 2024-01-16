# sqlite-rs
SQLite library in pure Rust and no dependencies (Under Development)

## Getting Started via sqlite-rs CLI

```sh
cargo install sqlite-rs
sqlite-rs
```

## Run test sqlite-rs tool
```sh
git clone https://github.com/afsec/sqlite-rs
RUST_LOG="trace" cargo run -- --database-file=./data/flights-initial.db
```

## Run tests
```sh
RUST_LOG="trace" cargo test -- --nocapture --test-threads=1
```

## Run example `sqlite_info`
```sh
RUST_LOG="trace" cargo run --release --target=$(arch)-unknown-linux-musl --example sqlite_info
```

## Live sessions

- [x] *2024-01-05*: https://www.youtube.com/live/017MSZud26s?si=sfdyvMcL8JKtpNbL
## Roadmap

- [x] SQLite Header parsing
- [x] Log using RUST_LOG env var
- [x] sqlite-rs cli tool
- [ ] Pager
- [ ] Read tables
- [ ] Read table schema
- [ ] Read table rows
- [ ] TBD
