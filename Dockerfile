FROM rust

COPY . . 
RUN cargo build --release
CMD ["./target/release/assigment2"]
