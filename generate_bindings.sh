#!/usr/bin/env bash

# Use bindgen for generating the bindings for this libary.
# Make sure to read both
# https://rust-lang.github.io/rust-bindgen/requirements.html
# and
# https://rust-lang.github.io/rust-bindgen/command-line-usage.html
# first.
bindgen --size_t-is-usize wrapper.h -o bindings.rs -- -I./olm/include
cat bindings_header.rs bindings.rs > src/lib.rs
