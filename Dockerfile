# Stage 1: Build the application
FROM debian:bullseye-slim as builder

# Install Rust, Git, and OpenSSL development libraries
RUN apt-get update && \
  apt-get upgrade -y && \
  apt-get install -y \
  curl \
  git \
  build-essential \
  pkg-config \
  libssl-dev && \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
  . "$HOME/.cargo/env"

WORKDIR /usr/src/myapp
COPY . .

# Build the Rust application
RUN . "$HOME/.cargo/env" && cargo install --path .

# Stage 2: Create the final image
FROM debian:bullseye-slim

# Copy the built binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/discord-bot /usr/local/bin/discord-bot

# Command to run the application
CMD ["discord-bot"]
