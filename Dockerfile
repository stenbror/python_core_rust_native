FROM rust:1.67.1-slim-bullseye AS builder
WORKDIR /usr/src/python_core_rust_native
COPY . .
RUN cargo install --path .

CMD ["/bin/bash"]

