FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /zet
WORKDIR /zet/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /zet/fuzz/target/x86_64-unknown-linux-gnu/release/zet-fuzz /