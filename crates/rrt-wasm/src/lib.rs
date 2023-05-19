use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn render(width: u32, height: u32) -> Uint8ClampedArray {
    console_error_panic_hook::set_once();

    let arr = Uint8ClampedArray::new_with_length(width * height * 4);
    for i in 0..height {
        for j in 0..width {
            let idx = (i * width + j) * 4;
            arr.set_index(idx + 0, ((i as f64 / height as f64) * 255.99) as u8);
            arr.set_index(idx + 1, ((j as f64 / width as f64) * 255.99) as u8);
            arr.set_index(idx + 2, 0);
            arr.set_index(idx + 3, 255);
        }
    }
    arr
}
