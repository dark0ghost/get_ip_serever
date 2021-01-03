FROM rust:1.48.0
WORKDIR /usr/src/get_ip_server
ADD . .

RUN cargo run --package get_ip_server --bin get_ip_server --release
