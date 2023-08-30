watch_no_logs:
	RUST_LOG="critical,actix_server=info" cargo watch -x run
watch:
	cargo watch -x run
build:
	cargo build
run: 
	cargo run
build_release:
	cargo build --release
run_release: 
	./target/release/life
run_release_no_logs:
	RUST_LOG="critical,actix_server=info" ./target/release/life
test:
	cargo test
remove_unused_deps:
	cargo +nightly udeps --all-targets
remove_unused_deps_release:
	cargo +nightly udeps
lint_fix:
	cargo clippy --fix --allow-dirty --allow-staged
install:
	rustup update
	rustup component add clippy
	cargo install cargo-watch
	cargo install cargo-udeps
build_docker:
	docker build -t life .
build_musl:
	cargo build --target x86_64-unknown-linux-musl
build_musl_release:
	cargo build --release --target x86_64-unknown-linux-musl
run_musl:
	./target/x86_64-unknown-linux-musl/debug/life
run_musl_release:
	./target/x86_64-unknown-linux-musl/release/life