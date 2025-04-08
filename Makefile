# Directories
CLIENT_DIR = client
SERVER_DIR = server

.PHONY: help lint-client lint-server lint build-client build-server build-server-prod move-assets run-server run-server-prod build up down

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
	@echo "  build             - Build the Docker images using Docker Compose"
	@echo "  up                - Create and start Docker containers in detached mode using Docker Compose"
	@echo "  down              - Stop and remove Docker containers using Docker Compose"

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
	COMPOSE_BAKE=true docker compose build

up:
	docker compose up -d

down:
	docker compose down
