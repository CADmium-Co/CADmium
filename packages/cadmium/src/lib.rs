use archetypes::FromSketchPrimitive;
use feature::solid::SolidArray;
use message::{Message, MessageResult};
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

    #[wasm_bindgen]
    pub fn get_sketch_primitive(
        &self,
        workbench_id: IDType,
        sketch_id: IDType,
        primitive_id: IDType,
    ) -> archetypes::WrappedPrimitive {
        let binding = self.native.get_workbench_by_id(workbench_id).unwrap();
        let workbench = binding.borrow();
        let binding = workbench
            .get_sketch_by_id(sketch_id)
            .unwrap()
            .borrow()
            .sketch();
        let sketch = binding.borrow();
        let binding = sketch.primitives();
        let primitive = binding.get(&primitive_id).unwrap().borrow().to_primitive();

        match primitive {
            isotope::primitives::Primitive::Point2(point) => archetypes::WrappedPrimitive::Point2(
                archetypes::Point2::from_sketch(&sketch, &point),
            ),
            isotope::primitives::Primitive::Line(line) => {
                archetypes::WrappedPrimitive::Line2(archetypes::Line2::from_sketch(&sketch, &line))
            }
            isotope::primitives::Primitive::Circle(circle) => {
                archetypes::WrappedPrimitive::Circle2(archetypes::Circle2::from_sketch(
                    &sketch, &circle,
                ))
            }
            isotope::primitives::Primitive::Arc(arc) => {
                archetypes::WrappedPrimitive::Arc2(archetypes::Arc2::from_sketch(&sketch, &arc))
            }
        }
    }
}
