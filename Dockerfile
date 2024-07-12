FROM rust:1.67

WORKDIR /usr/src/dbot
COPY . .

RUN cargo install --path .

CMD ["discord-bot"]
