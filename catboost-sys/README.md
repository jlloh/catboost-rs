# catboost-sys
## Introduction
Low level bindings for C API, adapted from https://github.com/catboost/catboost/tree/master/catboost/rust-package.

## Major Differences vs official bindings
Major differences are:
* Splitting out from the main repo due to the catboost repo being very big
* Change strategy from rebuilding the shared library from source (which makes it hard to publish as a crate) to downloading it from the github official releases
* Fixes to make it work for M1 macs (to download the right `libcatboost` binary)
* Most changes are in `build.rs`, no differences in the APIs
* We keep a copy of the `model_interface` c_bindings from `https://github.com/catboost/catboost/tree/v1.0.6/catboost/libs/model_interface` in this repo to simplify things

## Build Instructions
* Make sure you have the dependencies for bindgen
```
apt-get install -y build-essential pkg-config libssl-dev libclang-dev clang cmake
```
* Build it with `cargo build`
