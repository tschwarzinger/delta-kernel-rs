//! wasmtest: a test crate that depends on `delta_kernel` and `delta_kernel_default_engine` and
//! should compile for the `wasm32-unknown-unknown` target. It is based on
//! [`datafusion/wasmtest`].
//!
//! [`datafusion/wasmtest`]: https://github.com/apache/datafusion/tree/main/datafusion/wasmtest

use delta_kernel::schema::{DataType, PrimitiveType, StructField, StructType};
use delta_kernel_default_engine::executor::TaskExecutor;
use delta_kernel_default_engine::executor::wasm::WasmJspiExecutor;
use wasm_bindgen::prelude::*;

/// Builds a tiny Delta schema from [`delta_kernel::schema`] types and returns it as a string.
#[wasm_bindgen]
pub fn make_schema() -> String {
    let fields = vec![StructField::new(
        "name",
        DataType::Primitive(PrimitiveType::String),
        true,
    )];
    match StructType::try_new(fields) {
        Ok(schema) => format!("{schema:?}"),
        Err(e) => format!("schema error: {e}"),
    }
}

/// Constructs the wasm-only default engine executor and exercises its `block_on` bridge plus the
/// `make_send` adapter, proving the JSPI executor surface compiles for wasm32-unknown-unknown.
#[wasm_bindgen]
pub fn make_executor() -> bool {
    let executor = WasmJspiExecutor::new();
    executor.enter();

    // `block_on` drives a future through the microtask queue + JSPI bridge.
    let n: i32 = executor.block_on(async { 2 + 2 });
    if n != 4 {
        return false;
    }

    // `make_send` bridges a future into a Send future (here a plain value future).
    let made = WasmJspiExecutor::new();
    let output = made.block_on(delta_kernel_default_engine::executor::wasm::make_send(
        async { 40 + 2 },
    ));
    if output != 42 {
        return false;
    }

    true
}
