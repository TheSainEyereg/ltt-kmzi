# Task 1: AES-OFB implementation

To use runtime sbox generation, use `--features sbox_runtime` cargo key
```sh
# With runtime sbox calculation
cargo test --features sbox_runtime
cargo build --features sbox_runtime
cargo build --features sbox_runtime --release 
cargo run --features sbox_runtime
cargo run --features sbox_runtime --release

# Without runtime sbox calculation
cargo test
cargo build
cargo build --release
cargo run
cargo run --release
```