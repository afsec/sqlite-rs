# sqlite-rs
SQLite library in pure Rust and no dependencies (Under Development)

## Run tests
```sh
nix-shell
cargo test -- --nocapture --test-threads=1
```

## Run example `sqlite_info`
```sh
nix-shell
cargo run --release --target=$(arch)-unknown-linux-musl --example sqlite_info
```

## Live sessions

- [x] *2024-01-05*: https://www.youtube.com/live/017MSZud26s?si=sfdyvMcL8JKtpNbL
## Roadmap

- [x] SQLite Header parsing
- [ ] Pager
- [ ] Read tables
- [ ] Read table schema
- [ ] Read table rows
- [ ] TBD
