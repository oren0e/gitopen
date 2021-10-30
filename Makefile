install:
	cargo build --release
	chmod +x ./target/release/gitopen
	cp ./target/release/gitopen /usr/local/bin/

test:
	cargo test --verbose

lint:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -D warnings
