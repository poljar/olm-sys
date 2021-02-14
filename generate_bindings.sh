#!/usr/bin/env bash

# Use bindgen for generating the bindings for this libary.
# Make sure to read both
# https://rust-lang.github.io/rust-bindgen/requirements.html
# and
# https://rust-lang.github.io/rust-bindgen/command-line-usage.html
# first.
bindgen --size_t-is-usize wrapper.h -o bindings.rs -- -I./olm/include

# Prepend headers and write everything into the final file.
cat <( awk '{print "// "$0}' copyright_header.txt) \
    <(printf '\n') \
    <( awk '{print "//! "$0}' README.md | tail -n +3) \
    <(printf '\n') \
    bindings_header.rs \
    bindings.rs \
    > src/lib.rs

rustfmt src/lib.rs
