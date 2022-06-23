FROM rust:1.59 as builder-rust
WORKDIR /usr/src/app
COPY ./auto-promote .
RUN cargo install --path .

FROM golang:1.17.8 as builder-golang
WORKDIR /usr/src/app
COPY ./hcl-tweak .
RUN go build -o hcl-tweak ./main.go

FROM debian:bookworm-slim

COPY --from=builder-rust /usr/local/cargo/bin/auto-promote /usr/local/bin/auto-promote
COPY --from=builder-golang /usr/src/app/hcl-tweak /usr/local/bin/hcl-tweak

CMD ["auto-promote"]