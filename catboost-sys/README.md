# catboost-sys
## Introduction
Low level bindings for C API, adapted from https://github.com/catboost/catboost/tree/master/catboost/rust-package.

## Major Differences vs official bindings
Major differences are:
* Splitting out from the main repo due to the catboost repo being very big
* Change strategy from rebuilding it from source (which makes it hard to publish as a crate) to downloading it from the github official releases
* Fixes to make it work for M1 macs (to download the right `libcatboost` binary)
* Most changes are in `build.rs`, no differences in the APIs
* Also to decouple and to avoid mangling dependencies of the main catboost repo and the bindings here, similar to the `onnxruntime-rs` crate, to build this from source, you are required to:
  * Clone the catboost repo (with the version you want) by yourself
  * Set an environment variable called `CATBOOST_REPO`
* This allows the crate to find the `model_interface` C bindings

## Build Instructions
* Clone the official [catboost repo](https://github.com/catboost/catboost)
* Check out the branch for release 1.0.6
* Set an environment variable `CATBOOST_DIR` to point to the absolute path where you cloned the repo
* Build it with `cargo build`