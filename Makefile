help:
	@fgrep -h "##" $(MAKEFILE_LIST) | sed -e 's/## //' | tail -n +2

build: ## Build release
	./build.sh

bundle:
	lipo -create -output ./bin/generate-local-changelog ./bin/arm64/generate-local-changelog ./bin/x86_64/generate-local-changelog

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
	./bin/generate-local-changelog --unreleased > CHANGELOG.md
