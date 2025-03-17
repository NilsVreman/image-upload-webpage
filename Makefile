# Directories
CLIENT_DIR = client
SERVER_DIR = server

.PHONY: help lint-client lint-server lint build-client build-server build-server-prod move-assets run-server run-server-prod build run

help:
	@echo "Makefile targets:"
	@echo "  lint-client       - Lint the Vue (TypeScript/Vite) project in $(CLIENT_DIR)"
	@echo "  lint-server       - Lint the Rust/Axum project in $(SERVER_DIR)"
	@echo "  build-client      - Build the client (npm build in $(CLIENT_DIR))"
	@echo "  build-server      - Build the server (cargo build in $(SERVER_DIR))"
	@echo "  build-server-prod - Build the server in production mode (cargo build --release in $(SERVER_DIR))"
	@echo "  move-assets       - Move assets from $(CLIENT_DIR)/dist to $(SERVER_DIR)/assets"
	@echo "  run-server        - Run the server (cargo run in $(SERVER_DIR))"
	@echo "  run-server-prod   - Run the server in production mode (cargo run --release in $(SERVER_DIR))"

lint-client:
	cd $(CLIENT_DIR) && npm run lint

lint-server:
	cd $(SERVER_DIR) && cargo clippy

lint: lint-client lint-server

build-client:
	cd $(CLIENT_DIR) && npm run build

build-server:
	cd $(SERVER_DIR) && cargo build

build-server-prod:
	cd $(SERVER_DIR) && cargo build --release

move-assets:
	@echo "Moving assets from $(CLIENT_DIR)/dist to $(SERVER_DIR)/assets"
	mkdir -p $(SERVER_DIR)/assets
	cp -r $(CLIENT_DIR)/dist/* $(SERVER_DIR)/assets/

run-server:
	cd $(SERVER_DIR) && cargo run

run-server-prod:
	cd $(SERVER_DIR) && cargo run --release

build:
	docker build -t image-website .

run:
	docker run -p 3000:3000 image-website
