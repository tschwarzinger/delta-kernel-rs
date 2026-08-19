//! wasmtest: a test crate that depends on `delta_kernel` and should compile for the
//! `wasm32-unknown-unknown` target. It is based on [`datafusion/wasmtest`].
//!
//! Note that this crate only depends on the kernel, not the default engine, as it requires a
//! multithreaded tokio runtime, which is not available on `wasm32-unknown-unknown`.
//!
//! [`datafusion/wasmtest`]: https://github.com/apache/datafusion/tree/main/datafusion/wasmtest

use delta_kernel::schema::{DataType, PrimitiveType, StructField, StructType};
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
