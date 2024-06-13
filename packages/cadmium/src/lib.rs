use std::cell::RefCell;
use std::collections::BTreeMap;

use feature::solid::SolidArray;
use message::{Message, MessageResult};
use step::StepHash;
use tsify_next::declare;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

pub mod archetypes;
pub mod error;
pub mod feature;
pub mod isketch;
pub mod message;
pub mod project;
pub mod step;
pub mod workbench;

#[declare]
pub type IDType = u64;

thread_local! {
    // TODO: This is a bad solution to the hash <-> crate-internal-ID mapping problem
    static ID_MAP: RefCell<BTreeMap<StepHash, IDType>> = RefCell::new(BTreeMap::new());
}

#[wasm_bindgen]
pub struct Project {
    native: project::Project,
}

#[wasm_bindgen]
impl Project {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Project {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());

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
        // self.native.compute_constraint_errors();
    }

    #[wasm_bindgen]
    pub fn get_workbench(&self, workbench_index: u32) -> workbench::Workbench {
        // TODO: Use get() and return a Result
        self.native
            .workbenches
            .get(workbench_index as usize)
            .unwrap()
            .borrow()
            .clone() // This single call pollutes Clone derives for all MessageHandlers
    }

    #[wasm_bindgen]
    pub fn send_message(&mut self, message: &Message) -> MessageResult {
        message.handle(&mut self.native).into()
    }

    #[wasm_bindgen]
    pub fn get_workbench_solids(&self, workbench_index: u32) -> SolidArray {
        SolidArray(
            self.native
                .workbenches
                .get(workbench_index as usize)
                .unwrap()
                .borrow()
                .get_solids(),
        )
    }
}
