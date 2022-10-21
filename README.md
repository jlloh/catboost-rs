CatBoost Rust Package
======================
### Cargo Package
Cargo package can be found [here](https://crates.io/crates/catboost-rs)


### Historical Context
* This started off as an [attempt](https://github.com/catboost/catboost/pull/2161) to publish the catboost bindings to cargo
* Eventually decided to unofficially split out the code and maintain separate rust bindings, similar to `onnxruntime-rs`
* Note that this is voluntarily maintained and not yet endorsed by the official catboost team

### Differences versus official [rust package](https://github.com/catboost/catboost/tree/master/catboost/rust-package)
* Necessary changes made to publish crate instead of referencing the git repo, which is pretty big
* The official `catboost-sys` one attempts to rebuild the shared library, whereas this one downloads it from the github release page.
* The `build.rs` script is with assumption `libcatboost` shared library already downloaded separately.
* Also marked the Model as `Send` so that it can be used across threads, due to the documentation stating it's thread safe. Note that this is not yet extensively tested though.
* As of the present the catboost version is hardcoded, it is currently 1.0.6.

### If you need dependencies for bindgen
```
apt-get install -y curl build-essential pkg-config libssl-dev libclang-dev clang cmake
```

### Install libcatboost
* Download catboost binary from `https://github.com/catboost/catboost/releases/tag/v1.0.6`.
  * If you are using Linux, download `libcatboostmodel.so`
  * If you are using MacOS, download `libcatboostmodel.dylib`
  * Place the file in `/usr/lib/`
  * Create a soft link to `x.x.1`, e.g. `sudo ln -s libcatboostmodel.so libcatboostmodel.so.1` 

### Basic usage example
1. Add a dependency to your Cargo.toml:
```
[dependencies]
catboost-rs = "0.1.6"
```
2. To use catboost, it assumes the shared libraries are available. You will need to download the shared library from the official [releases page](https://github.com/catboost/catboost/releases). If you are using linux, download `libcatboostmodel.so`. If you are using Mac, download `libcatboostmodel.dylib`. As of the present, only version 1.0.6 is supported.
3. Move these libraries to `/usr/lib` 
4. Now you can apply pretrained model in your code:
```rust
// Bring catboost module into the scope
use catboost_rs as catboost;

fn main() {
    // Load the trained model
    let model = catboost::Model::load("tmp/model.bin").unwrap();

    println!("Number of cat features {}", model.get_cat_features_count());
    println!("Number of float features {}", model.get_float_features_count());

    // Apply the model
    let prediction = model
        .calc_model_prediction(
            vec![
                vec![-10.0, 5.0, 753.0],
                vec![30.0, 1.0, 760.0],
                vec![40.0, 0.1, 705.0],
            ],
            vec![
                vec![String::from("north")],
                vec![String::from("south")],
                vec![String::from("south")],
            ],
        )
        .unwrap();
    println!("Prediction {:?}", prediction);
}
```

### Documentation
Run `cargo doc --open` in `catboost/rust-package` directory.

### Tests
Run `cargo test` in `catboost/rust-package` directory.
