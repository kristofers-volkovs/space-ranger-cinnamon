
run:
	cargo run

checks:
	cargo run -p ci -- lints

fmt:
	cargo fmt --all
