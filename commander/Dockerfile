FROM rust:1-alpine as builder

ENV USER root

RUN apk add libc-dev protoc

WORKDIR /build

COPY . .

RUN cargo build --package commander --release

FROM scratch

COPY --from=builder /build/target/release/commander /commander

CMD ["/commander"]
