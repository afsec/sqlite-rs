# sqlite-rs
SQLite library in pure Rust with `no_std` support and no dependencies (Under Development)


## Example `sqlite_info`
```sh
nix-shell
cargo run --release --target=$(arch -m)-unknown-linux-musl --example sqlite_info
```



## Tests (no_std)

```sh
cargo test --no-default-features --tests -- --nocapture
```