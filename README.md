# sqlite-rs
SQLite library in pure Rust and no dependencies (Under Development)


## Example `sqlite_info`
```sh
nix-shell
cargo run --release --target=$(arch)-unknown-linux-musl --example sqlite_info
```



## Run tests (no_std)

```sh
nix-shell
cargo test --no-default-features --tests -- --nocapture
```
## Roadmap

- [x] SQLite Header parsing
- [ ] Pager
- [ ] Read tables
- [ ] Read table schema
- [ ] Read table rows
- [ ] TBD