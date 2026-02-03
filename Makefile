RUST_VERSION := $(shell grep 'rust-version = ' Cargo.toml | head -1 | sed 's/.*rust-version = "\(.*\)"/\1/')

.PHONY: install-rust
install-rust: ## Install Rust toolchain with required components
	rustup toolchain install $(RUST_VERSION)
	rustup component add rustfmt clippy --toolchain $(RUST_VERSION)

.PHONY: check
check: ## Run cargo check
	cargo +$(RUST_VERSION) check --all-features

.PHONY: fmt
fmt: ## Format all code
	cargo +$(RUST_VERSION) fmt

.PHONY: fmt-check
fmt-check: ## Check code formatting
	cargo +$(RUST_VERSION) fmt --check

.PHONY: clippy
clippy: ## Run clippy lints
	cargo +$(RUST_VERSION) clippy --all-targets --all-features -- -D warnings -W clippy::pedantic

.PHONY: test
test: ## Run all tests
	cargo +$(RUST_VERSION) test --all-features

.PHONY: test-doc
test-doc: ## Run documentation tests
	cargo +$(RUST_VERSION) test --doc --all-features

.PHONY: examples
examples: ## Run examples
	cargo +$(RUST_VERSION) run --example table
	@echo "Running crabular-cli with CSV..."
	cargo +$(RUST_VERSION) run -p crabular-cli --release -- -i examples/data.csv
	@echo "Running crabular-cli with TSV..."
	cargo +$(RUST_VERSION) run -p crabular-cli --release -- -i examples/data.tsv --format tsv
	@echo "Running crabular-cli with JSON..."
	cargo +$(RUST_VERSION) run -p crabular-cli --release -- -i examples/data.json --format json
	@echo "Running crabular-cli with JSONL..."
	cargo +$(RUST_VERSION) run -p crabular-cli --release -- -i examples/data.jsonl --format jsonl

.PHONY: ci
ci: fmt-check clippy test test-doc examples ## Run all CI checks

.PHONY: clean
clean: ## Clean build artifacts
	cargo +$(RUST_VERSION) clean
