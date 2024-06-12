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
use crate::isketch::{compound, ISketch};

pub trait NodeLike:
    Debug + Clone + Serialize + DeserializeOwned + IntoWasmAbi + RefFromWasmAbi
{
    fn add_link(&mut self, node: Node);
    fn links(&self) -> Vec<Node>;
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Node {
    // TODO: Add a variable/expression
    Point(Rc<RefCell<feature::point::Point3>>),
    Plane(Rc<RefCell<archetypes::Plane>>),
    Sketch(Rc<RefCell<ISketch>>, Vec<isotope::decompose::face::Face>),
    Primitive(Rc<RefCell<archetypes::WrappedPrimitive>>),
    Constraint(Rc<RefCell<isotope::constraints::Constraint>>),
    Compound(Rc<RefCell<compound::Compound>>),
    Solid(Vec<solid::Solid>),
}
