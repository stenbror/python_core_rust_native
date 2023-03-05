# python_core_rust_native

This project is to research parser and tokenizer for Python 3.11 or later written in rust with all needed unittests to support it.
Sometime in the future i might extend it to a fully python interpreter for embedded use and on several operating systems on PC and Mac,

### Building local on your machine

- git clone https://github.com/stenbror/python_core_rust_native
- cargo build
- cargo test

### Build and test with docker environment

- docker build -t python_core_rust_native .
- docker run -it --rm --name build_and_test python_core_rust_native
- cargo build
- cargo test
