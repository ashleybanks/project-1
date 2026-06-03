.PHONY: build build-edge build-service dev-edge dev-service dev-infra dev check fmt clippy clean

# Build all workspaces
build: build-edge build-service

build-edge:
	cd edge-worker && worker-build --release

build-service:
	cargo build -p analytics-service --release

# Development
dev-edge:
	cd edge-worker && wrangler dev

dev-service:
	cargo run -p analytics-service

dev-web:
	cd web && npm run dev

dev-infra:
	docker compose -f infra/docker-compose.yml up -d

stop-infra:
	docker compose -f infra/docker-compose.yml down

# Code quality
check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings

# Clean
clean:
	cargo clean
	cd web && rm -rf .next node_modules
