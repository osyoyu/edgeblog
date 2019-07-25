edgeblog
===

## Build & Deploy
Build a release build:

    cargo build --target wasm32-unknown-unknown --release

Then optimize it and put it in the package:

    wasm-opt -o terrarium_package/module.wasm target/wasm32-unknown-unknown/release/edgeblog_crate.wasm

Deploy!

    terrctl terrarium_package
