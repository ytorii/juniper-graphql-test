FROM rust:1.38.0 as builder

WORKDIR /usr/src/app
RUN USER=root cargo init

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

COPY src src

RUN rm -r target/release

RUN cargo build --release

FROM rust:1.38.0

COPY --from=builder /usr/src/app/target/release/juniper-graphql-test /bin/

EXPOSE 8080

CMD ["juniper-graphql-test"]
