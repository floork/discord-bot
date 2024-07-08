# Stage 1: Build the application
FROM debian:bullseye-slim as builder

# Install Rust and Git
RUN apt-get update && \
  apt-get upgrade -y && \
  apt-get install -y \
  curl \
  git \
  build-essential \ 
  && \
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
  . "$HOME/.cargo/env"

WORKDIR /usr/src/myapp
COPY . .

# Build the Rust application
RUN . "$HOME/.cargo/env" && cargo install --path .

# Stage 2: Create the final image
FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/discord-bot /usr/local/bin/discord-bot

CMD ["discord-bot"]
