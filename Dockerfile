FROM rust:1.53 as builder
RUN cargo new --bin my-portfolio
WORKDIR ./my-portfolio
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
ADD . ./
RUN cargo build --release 



FROM ubuntu:latest
COPY --from=builder ./my-portfolio/target/release/portfolio /usr/local/bin
COPY ./files /root/files
WORKDIR /root
CMD ["portfolio"]

