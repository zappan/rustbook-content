#!/bin/bash

# compile the progream with optimizations.
cargo build --release

# -----------------------------------------------------------------------------------
# This command will create an executable in target/release instead of target/debug.
# The optimizations make your Rust code run faster, but turning them on lengthens the
# time it takes for your program to compile.
# -----------------------------------------------------------------------------------
