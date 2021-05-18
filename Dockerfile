FROM rust:slim

# Also copies config.toml
WORKDIR /usr/src/dia
COPY . .

# Build with --debug to slim down build times
RUN cargo install --path .

CMD ["dia"]