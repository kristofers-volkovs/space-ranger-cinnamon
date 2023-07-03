
run:
	cargo run

fmt:
	cargo clippy -- -W clippy::correctness -D warnings && \
	cargo fmt
