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

#[wasm_bindgen]
pub struct Project {
    native: project::Project,
}

#[wasm_bindgen]
impl Project {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Project {
        Project {
            native: project::Project::new(name),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.native.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.native.name = name;
    }

    #[wasm_bindgen]
    pub fn json(&self) -> String {
        self.native.json()
    }

    // #[wasm_bindgen(getter)]
    // pub fn sketch(&self) -> sketch::Sketch {
    //     sketch::Sketch::from(self.native.sketch.clone())
    // }

    // #[wasm_bindgen(setter)]
    // pub fn set_sketch(&mut self, sketch: sketch::Sketch) {
    //     self.native.sketch = sketch.native;
    // }
}

// Below is just a test struct
#[wasm_bindgen]
pub struct Person {
    // project: project::Project,
    pub age: u64,
    pub height: f64,
    name: String,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Person {
        Person {
            age: 1203,
            height: 1.2,
            name: "Bob".to_string(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
