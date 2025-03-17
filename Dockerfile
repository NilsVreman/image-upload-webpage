# Create a multi-stage docker build

# -------------------------------
# Build Client Assets
# -------------------------------
FROM node:23-alpine AS client-build
WORKDIR /app/client

COPY client/package*.json ./
RUN npm install

COPY client/ .
RUN npm run build

# -------------------------------
# Build the Rust Server
# -------------------------------
FROM rust:1.85 AS server-build
WORKDIR /app/server

COPY server/Cargo.* ./
COPY server/src ./src

RUN cargo build --release

# -------------------------------
# Create the Final Runtime Image
# -------------------------------
FROM debian:bookworm-slim AS runtime-build
WORKDIR /app

# Copy server binary
COPY --from=server-build /app/server/target/release/server ./server

# Copy the client assets
COPY --from=client-build /app/client/dist ./assets

# Copy server secrets
COPY server/.env /app/.env

# Expose the server port
EXPOSE 3000

# Set the default command to run the server binary
CMD ["./server"]

