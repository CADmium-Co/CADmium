#![allow(dead_code, unused)]
use wasm_bindgen::prelude::*;

pub mod project;
pub mod sketch;

// #[wasm_bindgen]
// pub fn add(a: usize, b: usize) -> usize {
//     a + b
// }

#[wasm_bindgen]
pub struct Project {
    native: project::Project,
}

#[wasm_bindgen]
impl Project {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Project {
        let mut p = Project {
            native: project::Project::new(name),
        };

        p.native.add_defaults();
        p
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.native.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.native.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn json(&self) -> String {
        self.native.json()
    }

    #[wasm_bindgen]
    pub fn get_realization(&self, workbench_id: u32, max_steps: u32) -> String {
        self.native
            .get_realization(workbench_id as u64, 1000 as u64)
    }

    #[wasm_bindgen]
    pub fn send_message(&mut self, message: String) -> String {
        let result = self.native.handle_message_string(&message);
        match result {
            Ok(s) => s,
            Err(e) => e,
        }
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
