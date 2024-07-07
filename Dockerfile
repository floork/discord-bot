FROM rust:slim-bookworm

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release

CMD ["target/release/discord-bot"]
