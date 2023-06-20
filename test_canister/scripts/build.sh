cargo run -p test_canister --features "export-api" > test_canister.did
cargo build --target wasm32-unknown-unknown --release --features "export-api"
ic-wasm target/wasm32-unknown-unknown/release/test_canister.wasm -o target/wasm32-unknown-unknown/release/test_canister_opt.wasm shrink
gzip -k target/wasm32-unknown-unknown/release/test_canister_opt.wasm --force