// JavaScript Promise Integration (JSPI) bridge for `WasmJspiExecutor::block_on`.
//
// `WasmJspiExecutor::block_on` is called from synchronous kernel/engine code. To run a
// promise-driven async-I/O future to completion it must block the wasm caller while yielding to
// the JS event loop. JSPI (JavaScript Promise Integration) provides exactly that: when the module
// is built with `wasm-bindgen --jspi` and executed on a JSPI-capable host (V8 with
// `--experimental-wasm-jspi`), this imported function is treated as *suspending*, so calling it
// parks the wasm stack and resumes once `promise` settles.
//
// Without JSPI enabled this still compiles and can be provided, but real async I/O (e.g. fetch)
// will not progress from a synchronous context and `block_on` would spin/hang.
export function jspiBlockOnPromise(promise) {
    // Under `wasm-bindgen --jspi` this return is what the wasm caller suspends on.
    return promise;
}
