use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::archetypes;
use crate::isketch::{compound, ISketch};

use super::StepResult;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "action_type")]
pub enum SketchActionResult {
    Primitive(Rc<RefCell<archetypes::WrappedPrimitive>>),
    Constraint(Rc<RefCell<isotope::constraints::Constraint>>),
    Compound(Rc<RefCell<compound::Compound>>),
}

impl From<Rc<RefCell<archetypes::WrappedPrimitive>>> for SketchActionResult {
    fn from(primitive: Rc<RefCell<archetypes::WrappedPrimitive>>) -> Self {
        SketchActionResult::Primitive(primitive)
    }
}

impl From<Rc<RefCell<isotope::constraints::Constraint>>> for SketchActionResult {
    fn from(constraint: Rc<RefCell<isotope::constraints::Constraint>>) -> Self {
        SketchActionResult::Constraint(constraint)
    }
}

impl From<Rc<RefCell<compound::Compound>>> for SketchActionResult {
    fn from(compound: Rc<RefCell<compound::Compound>>) -> Self {
        SketchActionResult::Compound(compound)
    }
}

pub trait IntoSketchActionResult {
    fn into_result(self, sketch: &ISketch) -> StepResult;
}

impl<T: Into<SketchActionResult>> IntoSketchActionResult for T {
    fn into_result(self, sketch: &ISketch) -> StepResult {
        let faces = sketch.faces();
        StepResult::SketchAction {
            action: self.into(),
            faces,
        }
    }
}
