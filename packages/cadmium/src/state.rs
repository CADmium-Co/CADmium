use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use isotope::primitives;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::feature::solid::Solid;
use crate::isketch::ISketch;
use crate::step::Step;
use crate::workbench::Workbench;
use crate::IDType;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct StateInner {
    pub workbench_name: String,
    pub solids: Vec<Solid>,
    pub sketches: BTreeMap<IDType, Rc<RefCell<ISketch>>>,
    // pub sketch_primitives: BTreeMap<IDType, primitives::Primitive>,
    pub history: Vec<Rc<RefCell<Step>>>,
}

impl From<&Workbench> for StateInner {
    fn from(wb: &Workbench) -> Self {
        Self {
            workbench_name: wb.name.clone(),
            solids: wb.get_solids(),
            sketches: wb.sketches.clone(),
            // sketch_primitives: wb.sketch_primitives,
            history: wb.history.clone(),
        }
    }
}

#[wasm_bindgen]
pub struct State(pub StateInner);
