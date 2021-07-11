FROM rust:latest as builder

WORKDIR /app

COPY Cargo* ./

RUN mkdir src && \
    echo 'fn main(){println!("Hello, world!");}' > src/main.rs && \
    RUSTFLAGS="-C target-cpu=native" cargo build --release -q && \
    rm -f target/release/deps/warframe_bot* src/main.rs

COPY . .

RUN RUSTFLAGS="-C target-cpu=native" cargo build --release -q

FROM ubuntu:21.04

WORKDIR /app

COPY --from=builder /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
COPY locale locale
COPY .env .

RUN apt update -qq ; apt install -yqq ca-certificates && \
    useradd -ms /bin/bash pi ; chown -R pi:pi .

COPY --from=builder /app/target/release/warframe_bot /usr/local/bin/

USER pi

CMD warframe_bot
