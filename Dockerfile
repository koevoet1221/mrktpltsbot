# Build stage: Use rust:alpine as the base image for compilation
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /src
COPY . .
RUN cargo build --release

# Final stage: Use scratch for a minimal image
FROM scratch
COPY --from=builder /src/target/release/mrktpltsbot /app

ENV DB=/data/mrktpltsbot.sqlite3

ENV SEARCH_INTERVAL_SECS=60

ENV TELEGRAM_POLL_TIMEOUT_SECS=60

ENV MARKTPLAATS_SEARCH_LIMIT=30
ENV MARKTPLAATS_SEARCH_IN_TITLE_AND_DESCRIPTION=true

VOLUME /data
WORKDIR /data

ENTRYPOINT ["/app", "run"]
