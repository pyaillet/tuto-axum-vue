FROM rust:1-slim-buster AS builder

RUN mkdir -p /usr/src/server
WORKDIR /usr/src/server
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN mkdir -p /app

COPY --from=builder /usr/src/server/target/release/back /app/server

WORKDIR /app

CMD [ '/app/server' ]
