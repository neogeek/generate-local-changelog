help:
	@fgrep -h "##" $(MAKEFILE_LIST) | sed -e 's/## //' | tail -n +2

build: ## Build release
	./build.sh

format: ## Format code
	cargo fmt

lint: ## Run lint
	cargo clippy

test: ## Run tests
	cargo test

docs: ## Generate and open docs
	cargo doc
	open target/doc/generate_local_changelog/index.html

changelog: ## Generate changelog
	./bin/arm64/generate-local-changelog --unreleased > CHANGELOG.md
