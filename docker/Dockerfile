FROM rust:1.67

WORKDIR /usr/src/discord-bot
COPY . .

RUN cargo build --release

CMD ["target/release/discord-bot"]
