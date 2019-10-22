FROM rust:1.38.0-slim as builder

WORKDIR /usr/src/app
RUN USER=root cargo init

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

COPY src src

RUN cargo build --release

FROM rust:1.38.0-slim

COPY --from=builder /usr/src/app/target/release/juniper-graphql-test /bin/

CMD ["juniper-graphql-test"]

