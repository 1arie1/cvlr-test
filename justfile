cvlr-test-build-sbf:
	just -f cvlr-test/justfile build-sbf

cvlr-solana-test-build-sbf:
	just -f cvlr-solana-test/justfile build-sbf



build-sbf: cvlr-test-build-sbf cvlr-solana-test-build-sbf

clean: 
	cargo clean --manifest-path=cvlr-test/Cargo.toml
	cargo clean --manifest-path=cvlr-solana-test/Cargo.toml

cvlr-update:
	cargo update -p cvlr
	cargo update -p cvlr-solana
	
