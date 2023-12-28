#![allow(dead_code, unused)]
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

pub mod extrusion;
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
        console_error_panic_hook::set_once();
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
    pub fn to_json(&self) -> String {
        self.native.json()
    }

    #[wasm_bindgen]
    pub fn from_json(json: String) -> Project {
        let p = project::Project::from_json(&json);
        Project { native: p }
    }

    #[wasm_bindgen]
    pub fn compute_constraint_errors(&mut self) {
        self.native.compute_constraint_errors();
    }

    #[wasm_bindgen]
    pub fn get_realization(&self, workbench_id: u32, max_steps: u32) -> Realization {
        let realized = self
            .native
            .get_realization(workbench_id as u64, max_steps as u64);

        Realization { native: realized }
    }

    #[wasm_bindgen]
    pub fn get_workbench(&self, workbench_index: u32) -> String {
        let wb = &self.native.workbenches[workbench_index as usize];
        wb.json()
    }

    #[wasm_bindgen]
    pub fn send_message(&mut self, message: String) -> String {
        let result = self.native.handle_message_string(&message);
        match result {
            Ok(s) => format!("{{ \"success\": {{ {} }} }}", s),
            Err(e) => format!("{{ \"error\": {{ {} }} }}", e),
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

#[wasm_bindgen]
pub struct Realization {
    native: project::Realization,
}

#[wasm_bindgen]
impl Realization {
    #[wasm_bindgen]
    pub fn to_json(&self) -> String {
        let result = serde_json::to_string(&self.native);
        match result {
            Ok(json) => json,
            Err(e) => format!("Error: {}", e),
        }
    }

    #[wasm_bindgen]
    pub fn solid_to_obj(&self, solid_name: String, tolerance: f64) -> String {
        self.native.solid_to_obj(&solid_name, tolerance)
    }

    #[wasm_bindgen]
    pub fn solid_to_step(&self, solid_name: String) -> String {
        self.native.solid_to_step(&solid_name)
    }
}
