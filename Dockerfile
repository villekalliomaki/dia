FROM rust:slim

# Also copies config.toml
WORKDIR /usr/src/dia
COPY . .

RUN cargo install --path .

CMD ["dia"]