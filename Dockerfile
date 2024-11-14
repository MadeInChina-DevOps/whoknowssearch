FROM rust:1.74-slim as builder

WORKDIR /usr/src/app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/whoknows_nooneknows /app/
COPY --from=builder /usr/src/app/templates /app/templates
COPY --from=builder /usr/src/app/static /app/static

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["./whoknows_nooneknows"]