FROM rust:alpine as builder

RUN apk add -qq openssl-dev musl-dev libc6-compat

WORKDIR /app

COPY Cargo* ./

RUN mkdir src && \
    echo "fn main(){println!(\"Hello, world!\");}" > src/main.rs && \
    RUSTFLAGS="-C target-cpu=native" cargo build --release -q && \
    rm -f target/release/deps/warframe_bot* src/main.rs

COPY . .

RUN RUSTFLAGS="-C target-cpu=native" cargo build --release -q

FROM alpine:latest

RUN addgroup -g 1000 pi && adduser -D -s /bin/sh -u 1000 -G pi pi

WORKDIR /app

COPY --from=builder /app/target/release/warframe_bot /usr/local/bin/
COPY locale locale
COPY .env .

RUN apk add -qq --no-cache libc6-compat tzdata && \
  cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && \
  apk del -q tzdata && \
  chown -R pi:pi .

USER pi

CMD warframe_bot
