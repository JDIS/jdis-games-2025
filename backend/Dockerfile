# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set the working directory in the builder stage
WORKDIR /usr/src/app

# Copy your Rust project files
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the release binary
RUN cargo build --release

# Stage 2: Create the final lean image
FROM debian:bookworm-slim

# Set the working directory in the final image
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/jdis-games-2025-backend ./jdis-games-2025-backend

# Expose the port your server listens on
EXPOSE 32945

# Command to run your application when the container starts
CMD ["./jdis-games-2025-backend"]
