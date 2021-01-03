FROM rust:1.48.0

WORKDIR /usr/src/get_ip_server
ADD . .

EXPOSE 45045

RUN cargo install --path .

CMD ["get_ip_server"]
