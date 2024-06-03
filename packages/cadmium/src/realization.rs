use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::Plane;
use crate::message::idwrap::IDWrap;
use crate::solid::point::Point3;
use crate::isketch::ISketch;
use crate::solid::Solid;
use crate::workbench::Workbench;
use crate::IDType;
use std::collections::BTreeMap;

pub trait Realizable {
    fn realize(&self, realization: Realization) -> anyhow::Result<Realization>;
}

impl<T: Realizable + Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi> Realizable for IDWrap<T> {
    fn realize(&self, realization: Realization) -> anyhow::Result<Realization> {
        self.inner.realize(realization)
    }
}

pub fn btreemap_append<T: Clone>(map: &mut BTreeMap<IDType, T>, item: T) {
    let size = map.len();
    map.insert(size as IDType, item).unwrap();
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Realization {
    // a Realization is what you get if you apply the steps in a Workbench's
    // history and build a bunch of geometry
    pub planes: BTreeMap<IDType, Plane>,
    pub points: BTreeMap<IDType, Point3>,
    // TODO: Why do we return the same sketch twice?
    pub sketches: BTreeMap<IDType, (ISketch, ISketch, String)>,
    pub solids: BTreeMap<IDType, Solid>,
}

impl Realization {
    pub fn new(workbench: &Workbench) -> Self {
        let mut r = Realization {
            planes: BTreeMap::new(),
            points: BTreeMap::new(),
            sketches: BTreeMap::new(),
            solids: BTreeMap::new(),
        };

        r.planes = workbench.planes.iter().map(|(id, plane)| (*id, plane.borrow().clone())).collect();
        r.points = workbench.points.iter().map(|(id, point)| (*id, point.borrow().clone())).collect();
        r.sketches = workbench.sketches.iter().map(|(id, sketch)| {
            let sketch = sketch.borrow();
            (*id, (sketch.clone(), sketch.clone(), "TODO".to_string()))
        }).collect();

        r
    }

    pub fn solid_to_obj(&self, solid_name: IDType, tolerance: f64) -> String {
        let solid = &self.solids[&solid_name];
        let obj_text = solid.to_obj_string(tolerance);
        obj_text
    }

    pub fn save_solid_as_obj_file(&self, solid_name: IDType, filename: &str, tolerance: f64) {
        let solid = &self.solids[&solid_name];
        solid.save_as_obj(filename, tolerance);
    }

    pub fn solid_to_step(&self, solid_name: IDType) -> String {
        let solid = &self.solids[&solid_name];
        let step_text = solid.to_step_string();
        step_text
    }

    pub fn save_solid_as_step_file(&self, solid_name: IDType, filename: &str) {
        let solid = &self.solids[&solid_name];
        solid.save_as_step(filename)
    }
}
