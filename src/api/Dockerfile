# Start from the latest Rust base image
FROM rust:latest as builder

# Create a new directory in the container
WORKDIR /usr/src/api

# Copy the Cargo.toml and Cargo.lock files to the new directory
COPY ./src/api/Cargo.* ./

# This build step will cache your dependencies
RUN cargo fetch

# Copy the source code into the container
COPY ./src/api/src ./src

# Build the application
RUN cargo build --release

# Start a new stage
FROM debian:buster-slim

# Create a new directory
WORKDIR /api

# Copy the binary from the builder stage to the new directory
COPY --from=builder /usr/src/api/target/release/api .

# Expose port 8080
EXPOSE 80

# Command to run the application
CMD ["./api"]