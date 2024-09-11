#!/bin/bash

# Cargo also provides a command called cargo check. This command quickly checks
# your code to make sure it compiles but doesn’t produce an executable:
cargo check

# Often, cargo check is much faster than cargo build because it skips the step of producing
# an executable. If you’re continually checking your work while writing the code, using cargo
# check will speed up the process of letting you know if your project is still compiling!

# As such, many Rustaceans run cargo check periodically as they write their program to
# make sure it compiles. Then they run cargo build when they’re ready to use the executable.
