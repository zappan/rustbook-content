#!/bin/bash

# Check if exactly one argument is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <project-name>"
    exit 1
fi

# cargo new creates a new project
cargo new $1

# -----------------------------------------------------------------------------------
# The command creates a new directory for the project, and Cargo creates its files
# in a directory of the given name. Cargo generates two files and one directory
# for us: a Cargo.toml file and a src directory with a main.rs file inside.
#
# It has also initialized a new Git repository along with a .gitignore file.
# Git files won’t be generated if you run cargo new within an existing Git repository
# -----------------------------------------------------------------------------------
# With simple projects, Cargo doesn’t provide a lot of value over just using rustc,
# but it will prove its worth as your programs become more intricate. Once programs
# grow to multiple files or need a dependency, it’s much easier to let Cargo
# coordinate the build.
# -----------------------------------------------------------------------------------
