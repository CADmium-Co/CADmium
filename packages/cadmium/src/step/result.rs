use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::convert::{IntoWasmAbi, RefFromWasmAbi};
use wasm_bindgen::prelude::*;

use crate::archetypes;
use crate::feature::{self, solid};
use crate::isketch::{face, ISketch};

use super::sketch_action::SketchActionResult;

pub trait NodeLike:
    Debug + Clone + Serialize + DeserializeOwned + IntoWasmAbi + RefFromWasmAbi
{
    fn add_link(&mut self, node: StepResult);
    fn links(&self) -> Vec<StepResult>;
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum StepResult {
    Empty,
    // TODO: Add a variable/expression
    Point(Rc<RefCell<feature::point::Point3>>),
    Plane(Rc<RefCell<archetypes::Plane>>),
    Sketch(Rc<RefCell<ISketch>>),
    // Primitive(Rc<RefCell<archetypes::WrappedPrimitive>>),
    // Constraint(Rc<RefCell<isotope::constraints::Constraint>>),
    // Compound(Rc<RefCell<compound::Compound>>),
    SketchAction {
        action: SketchActionResult,
        faces: Vec<face::Face>,
    },
    // We need the solids to be a named field so that we can serialize it with tag = "type"
    // otherwise it would result in { type: "Solid", Solid[] } which isn't valid
    Solid {
        solids: Vec<solid::Solid>,
    },
}
