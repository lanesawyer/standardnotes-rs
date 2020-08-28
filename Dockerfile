FROM rustlang/rust:nightly

WORKDIR /usr/src/standardnotes-rs
COPY . .

# RUN cargo clippy
RUN cargo test
RUN cargo build --release

CMD ["./target/release/standardnotes-rs"]
