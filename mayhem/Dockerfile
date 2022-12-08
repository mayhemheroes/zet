FROM fuzzers/cargo-fuzz:0.11.0 as builder

# Put sources in /zet
ADD . /zet
WORKDIR /zet

RUN cargo fuzz build

ENTRYPOINT []
CMD ["/zet/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz"]
