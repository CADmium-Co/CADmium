use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::Point3;
use crate::project::{RealPlane, RealSketch};
use crate::solid::Solid;
use std::collections::HashMap;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Realization {
    // a Realization is what you get if you apply the steps in a Workbench's
    // history and build a bunch of geometry
    pub planes: HashMap<String, RealPlane>,
    pub points: HashMap<String, Point3>,
    pub sketches: HashMap<String, (RealSketch, RealSketch, String)>,
    pub solids: HashMap<String, Solid>,
}

impl Realization {
    pub fn new() -> Self {
        Realization {
            planes: HashMap::new(),
            points: HashMap::new(),
            sketches: HashMap::new(),
            solids: HashMap::new(),
        }
    }

    pub fn solid_to_obj(&self, solid_name: &str, tolerance: f64) -> String {
        let solid = &self.solids[solid_name];
        let obj_text = solid.to_obj_string(tolerance);
        obj_text
    }

    pub fn save_solid_as_obj_file(&self, solid_name: &str, filename: &str, tolerance: f64) {
        let solid = &self.solids[solid_name];
        solid.save_as_obj(filename, tolerance);
    }

    pub fn solid_to_step(&self, solid_name: &str) -> String {
        let solid = &self.solids[solid_name];
        let step_text = solid.to_step_string();
        step_text
    }

    pub fn save_solid_as_step_file(&self, solid_name: &str, filename: &str) {
        let solid = &self.solids[solid_name];
        solid.save_as_step(filename)
    }
}
