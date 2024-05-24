
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription, Point3, Vector3};
use crate::sketch::Sketch;
use crate::extrusion::Extrusion;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum StepData {
    Point {
        point: Point3,
    },
    Plane {
        plane: Plane,
        width: f64,
        height: f64,
    },
    Sketch {
        plane_description: PlaneDescription,
        width: f64,
        height: f64,
        sketch: Sketch,
    },
    Extrusion {
        extrusion: Extrusion,
    },
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub(crate) name: String,
    pub(crate) unique_id: String,
    pub(crate) suppressed: bool,
    pub(crate) data: StepData,
}

impl Step {
    pub fn new_point(name: &str, point: Point3, point_id: u64) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Point-{}", point_id),
            suppressed: false,
            data: StepData::Point {
                point: point.clone(),
            },
        }
    }

    pub fn new_plane(name: &str, plane: Plane, plane_id: u64) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Plane-{}", plane_id),
            suppressed: false,
            data: StepData::Plane {
                plane,
                height: 100.0,
                width: 100.0,
            },
        }
    }

    pub fn new_sketch(name: &str, plane_id: &str, sketch_id: u64) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Sketch-{}", sketch_id),
            suppressed: false,
            data: StepData::Sketch {
                plane_description: PlaneDescription::PlaneId(plane_id.to_owned()),
                width: 1.25,
                height: 0.75,
                sketch: Sketch::new(),
            },
        }
    }

    pub fn new_sketch_on_solid_face(
        name: &str,
        solid_id: &str,
        normal: Vector3,
        sketch_id: u64,
    ) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Sketch-{}", sketch_id),
            suppressed: false,
            data: StepData::Sketch {
                plane_description: PlaneDescription::SolidFace {
                    solid_id: solid_id.to_owned(),
                    normal,
                },
                width: 12.5,
                height: 7.5,
                sketch: Sketch::new(),
            },
        }
    }

    pub fn new_extrusion(name: &str, extrusion: Extrusion, extrusion_id: u64) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Extrusion-{}", extrusion_id),
            suppressed: false,
            data: StepData::Extrusion { extrusion },
        }
    }
}
