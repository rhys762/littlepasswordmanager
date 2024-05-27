
# About

Tauri/rust/vuejs.

This is a small, local password manager. It exists because

- I wanted a local password manager.
- I wanted to learn Rust.

# Building

You need:

- nvm
- cargo and the rest of the rust toolchain
- tauri system prerequisites https://tauri.app/v1/guides/getting-started/prerequisites

Compile with:

    nvm use
    npm i
    npm run tauri build

which will produce the binary

    src-tauri/target/release/little-password-manager

The dev server can be run with

    npm run tauri dev


# Running

The program requires a sqlite3 database to read and write from. Where the program will look for (and create should it not exist) is configured by environmental variables.

The **LPM_DB_PATH** variable is checked first, and is expected to be a database path:

    LPM_DB_PATH=/some/place/lpm.sqlite3 src-tauri/target/release/little-password-manager

If **LPM_DB_PATH** is not set, but **HOME** is, then the program will try to use HOME/.little_password_manager.sqlite3.

If **HOME** is not set, then the program will try ./.little_password_manager.sqlite3, being the location of the calling shell.

I recomend a bash alias

    alias lpm="LPM_DB_PATH=/some/place/lpm.sqlite3 /x/y/z/src-tauri/target/release/little-password-manager"

# Unit Tests

test with

    cd src-tauri
    cargo test