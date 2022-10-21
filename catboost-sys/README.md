# catboost-sys
* Download catboost binary from `https://github.com/catboost/catboost/releases/tag/v1.0.6`.
  * If you are using Linux, download `libcatboostmodel.so`
  * If you are using MacOS, download `libcatboostmodel.dylib`
  * Place the file in `/usr/lib/`
  * Create a soft link to `x.x.1`, e.g. `sudo ln -s libcatboostmodel.so libcatboostmodel.so.1` 
## Introduction
Low level bindings for C API, adapted from https://github.com/catboost/catboost/tree/master/catboost/rust-package.

## Major Differences vs official bindings
Major differences are:
* We assume libcatboost shared library is already downloaded (see instructions below)
* Splitting out from the main repo due to the catboost repo being very big
* Most changes are in `build.rs`, no differences in the APIs
* We keep a copy of the `model_interface` c_bindings from `https://github.com/catboost/catboost/tree/v1.0.6/catboost/libs/model_interface` in this repo to simplify things

## Build Instructions
* Make sure you have the dependencies for bindgen
```
apt-get install -y build-essential pkg-config libssl-dev libclang-dev clang cmake
```
* Download catboost binary from `https://github.com/catboost/catboost/releases/tag/v1.0.6`.
  * If you are using Linux, download `libcatboostmodel.so`
  * If you are using MacOS, download `libcatboostmodel.dylib`
  * Place the file in `/usr/lib/`
  * Create a soft link to `x.x.1`, e.g. `sudo ln -s libcatboostmodel.so libcatboostmodel.so.1` 
* Build it with `cargo build`
