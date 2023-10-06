#![allow(dead_code, unused)]
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

#[wasm_bindgen]
pub fn multiply(a: usize, b: usize) -> usize {
    a * b
}

// #[wasm_bindgen]
// pub fn new_person() -> Person {
//     Person::new()
// }

#[wasm_bindgen]
pub struct Person {
    // project: project::Project,
    age: u64,
}

// #[wasm_bindgen]
// impl Person {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Person {
//         Person { age: 12 }
//     }
// }
