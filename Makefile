
usage: 
	@echo "make test"

test:
	@RUST_LOG=trace cargo test ok_on_show_tables -- --nocapture
