
use cadmium_macros::StepDataActions;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription, Point2};
use crate::solid::extrusion;
use crate::solid::point::Point3;
use crate::IDType;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum StepOperation {
    Add,
    Update,
    Delete,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub(crate) id: IDType,
    pub(crate) operation: StepOperation,
    pub(crate) name: String,
    pub(crate) unique_id: String, // TODO: remove this field, it's not needed
    pub(crate) suppressed: bool,
    pub(crate) data: StepData,
}

#[derive(StepDataActions, Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum StepData {
    // Workbench Primitives
    WorkbenchPoint {
        workbench_id: IDType,
        point: Point3,
    },
    WorkbenchPlane {
        workbench_id: IDType,
        plane: Plane,
        width: f64,
        height: f64,
    },
    WorkbenchSketch {
        workbench_id: IDType,
        plane_description: PlaneDescription,
        // sketch: ISketch,
        // width: f64,
        // height: f64,
    },

    // Sketch Primitives
    #[step_data(workbench_field = "sketches", type = "Sketch")]
    SketchPoint {
        workbench_id: IDType,
        sketch_id: IDType,
        point: Point2,
    },
    #[step_data(workbench_field = "sketches", type = "Sketch")]
    SketchArc {
        workbench_id: IDType,
        sketch_id: IDType,
        center: IDType,
        radius: f64,
        clockwise: bool,
        start_angle: f64,
        end_angle: f64,
    },
    #[step_data(workbench_field = "sketches", type = "Sketch")]
    SketchCircle {
        workbench_id: IDType,
        sketch_id: IDType,
        center: IDType,
        radius: f64,
    },
    #[step_data(workbench_field = "sketches", type = "Sketch")]
    SketchLine {
        workbench_id: IDType,
        sketch_id: IDType,
        start: IDType,
        end: IDType,
    },
    SolidExtrusion {
        workbench_id: IDType,
        face_ids: Vec<IDType>,
        sketch_id: IDType,
        length: f64,
        offset: f64,
        mode: extrusion::Mode,
        direction: extrusion::Direction,
    },
}
