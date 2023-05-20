use js_sys::Promise;

use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn initialize(num_threads: usize, set_panic_hook: bool) -> Promise {
    if set_panic_hook {
        console_error_panic_hook::set_once();
    }
    wasm_bindgen_rayon::init_thread_pool(num_threads)
}
