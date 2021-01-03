FROM rustlang/rust:nightly
ENV SN_SECRET=test_secret

WORKDIR /usr/src/standardnotes-rs
COPY . .

# RUN cargo clippy
RUN cargo test
RUN cargo build --release

CMD ["./target/release/standardnotes-rs"]
