FROM rust:slim-bookworm

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["bash"]
