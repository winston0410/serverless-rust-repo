# Serverless Rust bloat reproduction

This repo is to reproduce the bloat issue in Rust in serverless.

## Flow to reproduce

I tried to build two binaries in `apps/example/Cargo.toml`, each binary will be used for a serverless handler:

```toml
[[bin]]
name = "v1-hello-get"
path = "src/v1/hello/get.rs"

[[bin]]
name = "v1-bye-get"
path = "src/v1/bye/get.rs"
```

Build with the follow command:

```sh
cargo +nightly build -Z build-std-features=panic_immediate_abort -Z build-std=std,panic_abort --release --target aarch64-unknown-linux-gnu
```

Check the size of `./target/aarch64-unknown-linux-gnu/release/v1-hello-get` and `./target/aarch64-unknown-linux-gnu/release/v1-bye-get`

```sh
ls -lh ./target/aarch64-unknown-linux-gnu/release/v1-hello-get
ls -lh ./target/aarch64-unknown-linux-gnu/release/v1-bye-get
```

```sh
-rwxr-xr-x  1 xxx  staff   697K May 25 10:06 ./target/aarch64-unknown-linux-gnu/release/v1-hello-get
-rwxr-xr-x  1 xxx  staff   697K May 25 10:06 ./target/aarch64-unknown-linux-gnu/release/v1-bye-get
```

After that uncomment the commented code in `apps/example/Cargo.toml` and `apps/example/src/v1/hello/get.rs`, and build again

```toml
# aws-sdk-dynamodb = "0.12.0"
# aws-config = "0.12.0"
```

```rust
    // apps/example/src/v1/hello/get.rs
    // let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");
    // let config = aws_config::from_env().region(region_provider).load().await;
    // let client = aws_sdk_dynamodb::Client::new(&config);
```

Even though we haven't done anything to `apps/example/src/v1/bye/get.rs`, the **size of the binary increased**.

```sh
-rwxr-xr-x  1 xxx  staff   3.1M May 25 10:18 ./target/aarch64-unknown-linux-gnu/release/v1-hello-get
-rwxr-xr-x  1 xxx  staff   1.2M May 25 10:18 ./target/aarch64-unknown-linux-gnu/release/v1-bye-get
```