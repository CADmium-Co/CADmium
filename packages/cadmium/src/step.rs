
use cadmium_macros::StepDataActions;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription, Point2, Point3};
use crate::IDType;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum StepOperation {
    Add,
    Update,
    Delete,
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub(crate) operation: StepOperation,
    pub(crate) name: String,
    pub(crate) unique_id: String,
    pub(crate) suppressed: bool,
    pub(crate) data: StepData,
}

#[derive(StepDataActions, Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum StepData {
    WorkbenchPoint {
        point: Point3,
    },
    WorkbenchPlane {
        plane: Plane,
        width: f64,
        height: f64,
    },
    WorkbenchSketch {
        plane_description: PlaneDescription,
        // sketch: ISketch,
        // width: f64,
        // height: f64,
    },
    #[step_data(workbench_field = "sketches", type = "Sketch")]
    SketchPoint {
        point: Point2,
    },
    #[step_data(workbench_field = "sketches", type = "Sketch")]
    SketchArc {
        center: IDType,
        radius: f64,
        clockwise: bool,
        start_angle: f64,
        end_angle: f64,
    },
    #[step_data(workbench_field = "sketches", type = "Sketch")]
    SketchCircle {
        center: IDType,
        radius: f64,
    },
    #[step_data(workbench_field = "sketches", type = "Sketch")]
    SketchLine {
        start: IDType,
        end: IDType,
    }
    // Solid {
    //     Extrusion {
    //         extrusion: Extrusion,
    //     },
    // }
}
