use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
    a + b
}
