dev:
	cargo run -- --dev

compile:
	./scripts/build.sh

purge:
	./target/release/yalland-node purge-chain --dev

validator-1:
	cd scripts && ./run1.sh

validator-2:
	cd scripts && ./run2.sh

validator-3:
	cd scripts && ./run3.sh
