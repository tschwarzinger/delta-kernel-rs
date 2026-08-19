# wasmtest

A compile-time smoke test that [`delta_kernel`](../kernel) builds for the
`wasm32-unknown-unknown` target.

This crate is based on DataFusion's
[`datafusion/wasmtest`](https://github.com/apache/datafusion/tree/main/datafusion/wasmtest):
rather than hard-coding wasm-specific features into `delta_kernel`, it keeps the kernel's manifest
clean and instead depends on the kernel from here, enabling the dependency features that wasm needs
via Cargo feature unification.

## How it works

- `delta_kernel` is a path dependency (this is a separate cargo workspace, mirroring
  `datafusion-executor/`, so the wasm-specific declaration does not leak into native builds of the
  root workspace).
- `Cargo.toml` enables `wasm_js` on getrandom 0.3 (rand's version) and 0.4 (uuid's internal
  version), and `rng-getrandom` on uuid — all feature-unified onto the getrandom/uuid instances the
  kernel pulls in transitively.
- `.cargo/config.toml` sets `--cfg getrandom_backend="wasm_js"` for the wasm target (required by
  getrandom alongside the feature; scoped to this workspace only).
- `src/lib.rs` exports a `#[wasm_bindgen]` function touching `delta_kernel::schema` types, so the
  kernel is actually compiled and linked (and a real wasm module is produced).

## Build / verify

Requires the rustup target and cargo's wasm toolchain:

```shell
rustup target add wasm32-unknown-unknown
```

From the repository root (uses the `check-wasm-kernel` cargo alias / CI-parity command):

```shell
cargo check --locked --manifest-path wasmtest/Cargo.toml --target wasm32-unknown-unknown
```

Or use `wasm-pack` from within this directory to build a real wasm module:

```shell
wasm-pack build
```

Tests (via `wasm-pack test --firefox --headless`) can be added later, following the DataFusion
pattern.
