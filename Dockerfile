FROM rust:slim-bookworm

WORKDIR /usr/src/myapp
COPY . .

RUN apt install pkg-config
RUN cargo build --release

CMD ["target/release/discord-bot"]
