FROM rust:1.61 as builder
WORKDIR /build/
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src
COPY ./migrations ./migrations
RUN cargo build --release

FROM ubuntu:22.04 AS run
EXPOSE 5000
WORKDIR /run/
COPY --from=builder /build/target/release/backend .
COPY ./docker-entrypoint.sh .
COPY wait-for-it.sh .
RUN chmod +x ./wait-for-it.sh ./docker-entrypoint.sh
RUN apt-get update -y
RUN apt-get install -y libpq-dev
# docker-entrypoint.sh - wait 5432 port aviable & run to_do app
ENTRYPOINT ["./docker-entrypoint.sh"]
CMD ["./backend"]