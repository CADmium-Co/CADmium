// #![allow(dead_code, unused)]
use wasm_bindgen::prelude::*;

pub mod project;
pub mod sketch;

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[wasm_bindgen]
pub fn subtract(a: usize, b: usize) -> usize {
    a - b
}
