#!/bin/bash

########################################
############# CSCI 2951-O ##############
########################################

# Update this file with instructions on how to compile your code

# Check if cargo exists
if [ -x "$(command -v cargo)" ]; then
    cargo build --release
else
    echo "Error: cargo is not installed. Installing it..." >&2
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env

    cargo build --release
fi