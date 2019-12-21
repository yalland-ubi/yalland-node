dev:
	cargo run -- --dev

compile:
	./scripts/build.sh

purge:
	./target/release/yalland-node purge-chain --dev
