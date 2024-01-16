# sqlite-rs
SQLite library in pure Rust and no dependencies (Under Development)

## Getting Started via sqlite-rs CLI

## Installation
```sh
cargo install sqlite-rs
sqlite-rs
```


## Running
```sh
sqlite-rs
```
```log
sqlite-rs v0.3.2 - 1705443253
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
sqlite-rs>
```
### Running with custom log level `TRACE`

```sh

RUST_LOG="trace" sqlite-rs
```

```log
sqlite-rs v0.3.2 - 1705443182
Enter ".help" for usage hints.
1705443185.1720595 TRACE sqlite_rs: Openning SQliteIo [:memory:]...
1705443185.1720929 TRACE sqlite_rs: SQliteIo started: [SqliteIo { mode: InMemory }].
1705443185.1721094 TRACE sqlite_rs: Connecting SqlitePager...
1705443185.1721282 TRACE sqlite_rs::pager: [0] Bytes read from [InMemory]
1705443185.1721451 TRACE sqlite_rs: SQliteIo started: [SqlitePager { io: SqliteIo { mode: InMemory }, page_size: L4096, reserved_bytes_per_page: ReservedBytesPerPage(0) }].
1705443185.1721630 TRACE sqlite_rs: Starting SqliteRuntime...
1705443185.1721826 TRACE sqlite_rs: SqliteRuntime started: [SqliteRuntime { pager: SqlitePager { io: SqliteIo { mode: InMemory }, page_size: L4096, reserved_bytes_per_page: ReservedBytesPerPage(0) }, header: SqliteHeader { magic_header_string: MagicHeaderString, page_size: L4096, file_format_version_numbers: FileFormatVersionNumbers { write_version: Legacy, read_version: Legacy }, reserved_bytes_per_page: ReservedBytesPerPage(0), payload_fractions: PayloadFractions { maximum: MaximumEmbeddedPayloadFraction(64), minimum: MinimumEmbeddedPayloadFraction(32), leaf: LeafPayloadFraction(32) }, file_change_counter: FileChangeCounter(0), db_filesize_in_pages: DatabaseFileSizeInPages(1), freelist_pages: FreeListPages { first: FreeListPagesFirstTrunkPage(0), total: FreeListPagesTotalPages(0) }, schema_cookie: SchemaCookie(0), schema_format: Format4, suggested_cache_size: SuggestedCacheSize(0), incremental_vacuum_settings: IncrementalVacuumSettings { largest_root_btree_page: LargestRootBtreePage(0), incremental_vacuum_mode: False }, database_text_encoding: Utf8, user_version: UserVersion(0), application_id: ApplicationId(0), reserved_for_expansion: ReservedForExpansion, version_valid_for: VersionValidFor(302), write_library_version: WriteLibraryVersion(302) }, btree: SqliteBtree(()) }].
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
sqlite-rs>
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
