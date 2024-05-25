
# About

Tauri/rust/vuejs.

This is a small, local password manager. It exists because

- I wanted a local password manager.
- I wanted to learn Rust.

# Building

You need cargo and the rest of the Rust toolchain installed.

Compile with:

    cargo build --release

which will produce

    target/release/littlepasswordmanager

# Running

The program requires a path to a sqlite database to function, it will create the database specified if it doesn't already exist. I recomend a bash alias

    alias lpm='a/b/c/target/release/littlepasswordmanager /somewhere/safe/lpm.sqlite3'

# Unit Tests

test with

    cargo test