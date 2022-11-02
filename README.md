# wasix-integration-tests

Integration test suite for the Webassembly `wasix` toolchain.

* ./simple: Rust binaries that compile to wasm
* ./wasm: Collection of precompiled Webassembly modules

* ./snapshots: Snapshot test suite

## Running Snapshot Tests

```bash
# NOTE: must first install wasmer with WASIX support.

cargo install cargo-insta
cd snapshots
cargo insta test
```

To review (accept or reject) changed snapshots:

```
cargo insta review
```
