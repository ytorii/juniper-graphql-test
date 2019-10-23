FROM rust as builder

WORKDIR /usr/src/app
RUN USER=root cargo init

COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release

RUN rm -r src

COPY src src

RUN cargo build --release

FROM rust

COPY --from=builder /usr/src/app/target/release/juniper-graphql-test /bin/

EXPOSE 8080

CMD ["juniper-graphql-test"]
