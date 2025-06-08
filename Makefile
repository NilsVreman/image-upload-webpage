# Deployment variables
DOMAIN  ?= $(shell grep -E '^DOMAIN='   .env | cut -d= -f2)
LE_EMAIL?= $(shell grep -E '^LE_EMAIL=' .env | cut -d= -f2)

# Directories
CLIENT_DIR = client
SERVER_DIR = server

help:
	@echo "Makefile targets:"
	@echo "  lint-client         - Lint the Vue (TypeScript/Vite) project in $(CLIENT_DIR)"
	@echo "  lint-server         - Lint the Rust/Axum project in $(SERVER_DIR)"
	@echo "  build-client        - Build the client (npm build in $(CLIENT_DIR))"
	@echo "  build-server        - Build the server in production mode (cargo build --release in $(SERVER_DIR))"
	@echo "  move-assets         - Move assets from $(CLIENT_DIR)/dist to $(SERVER_DIR)/assets"
	@echo "  run-server          - Run the server in production mode (cargo run --release in $(SERVER_DIR))"
	@echo "  build               - Build the Docker images using Docker Compose"
	@echo "  up                  - Create and start Docker containers in detached mode using Docker Compose"
	@echo "  down                - Stop and remove Docker containers using Docker Compose"
	@echo "  init-cert           - Run first-time Let's Encrypt issuance"
	@echo "  schedule-cert-renew - Install cron for automatic cert renewals"

.PHONY: lint-client
lint-client:
	cd $(CLIENT_DIR) && npm run lint

.PHONY: lint-server
lint-server:
	cd $(SERVER_DIR) && cargo clippy

.PHONY: lint
lint: lint-client lint-server

.PHONY: build-client
build-client:
	cd $(CLIENT_DIR) && npm run build

.PHONY: build-server
build-server:
	cd $(SERVER_DIR) && cargo build --release

.PHONY: move-assets
move-assets:
	@echo "Moving assets from $(CLIENT_DIR)/dist to $(SERVER_DIR)/assets"
	mkdir -p $(SERVER_DIR)/assets
	cp -r $(CLIENT_DIR)/dist/* $(SERVER_DIR)/assets/

.PHONY: build-local
build-local: build-client move-assets build-server

.PHONY: run-local
run-server:
	cd $(SERVER_DIR) && cargo run --release

.PHONY: build
build:
	COMPOSE_BAKE=true docker compose build

.PHONY: up
up:
	docker compose up -d

.PHONY: down
down:
	docker compose down

.PHONY: init-cert
init-cert:
	docker compose run --rm certbot certonly \
		--webroot -w /usr/share/nginx/html \
		-d $(DOMAIN) -d www.$(DOMAIN) \
		--email $(LE_EMAIL) \
		--agree-tos --non-interactive
	docker compose exec nginx nginx -s reload

.PHONY: schedule-cert-renew
schedule-cert-renew:
	@(crontab -l 2>/dev/null; \
	  echo "0 4 * * * cd $(CURDIR) && docker compose run --rm certbot renew && docker compose exec nginx nginx -s reload") \
	  | crontab -
