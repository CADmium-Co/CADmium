
use std::fmt::Display;

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

impl Display for StepOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepOperation::Add => write!(f, "Add"),
            StepOperation::Update => write!(f, "Update"),
            StepOperation::Delete => write!(f, "Delete"),
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub(crate) id: IDType,
    pub(crate) operation: StepOperation,
    pub(crate) name: String,
    pub(crate) suppressed: bool,
    pub(crate) data: StepData,
}

impl Step {
    pub fn unique_id(&self) -> String {
        // TODO: Should use the type of StepData instead of name
        format!("{}:{}-{}", self.operation, self.name, self.id)
    }
}

#[derive(StepDataActions, Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum StepData {
    // Project operations
    // TODO: Steps in project::Project are not yet supported as the function just recurses forever
    // #[step_data(skip_all = true)]
    // ProjectRename {
    //     new_name: String,
    // },

    // Workbench operations
    #[step_data(skip_all = true)]
    WorkbenchRename {
        workbench_id: u64,
        new_name: String,
    },
    #[step_data(skip_update = true, skip_delete = true)]
    WorkbenchPoint {
        workbench_id: IDType,
        point: Point3,
    },
    #[step_data(skip_update = true, skip_delete = true)]
    WorkbenchPlane {
        workbench_id: IDType,
        plane: Plane,
        width: f64,
        height: f64,
    },
    #[step_data(skip_update = true, skip_delete = true)]
    WorkbenchSketch {
        workbench_id: IDType,
        plane_description: PlaneDescription,
        // sketch: ISketch,
        // width: f64,
        // height: f64,
    },
    #[step_data(skip_all = true)]
    WorkbenchStepRename {
        workbench_id: IDType,
        step_id: IDType,
        new_name: String,
    },
    // Note that we don't use the auto-generated `delete` operation
    // as we're deleting steps themselves, not their data
    // You can't add_workbench_step_delete for example like you can add_workbench_point
    #[step_data(skip_all = true, skip_history = true)]
    WorkbenchStepDelete {
        workbench_id: IDType,
        step_id: IDType,
    },

    // Sketch Primitives
    #[step_data(workbench_field = "sketches", type_name = "Sketch", skip_update = true, skip_delete = true)]
    SketchPoint {
        workbench_id: IDType,
        sketch_id: IDType,
        point: Point2,
    },
    #[step_data(workbench_field = "sketches", type_name = "Sketch", skip_update = true, skip_delete = true)]
    SketchArc {
        workbench_id: IDType,
        sketch_id: IDType,
        center: IDType,
        radius: f64,
        clockwise: bool,
        start_angle: f64,
        end_angle: f64,
    },
    #[step_data(workbench_field = "sketches", type_name = "Sketch", skip_update = true, skip_delete = true)]
    SketchCircle {
        workbench_id: IDType,
        sketch_id: IDType,
        center: IDType,
        radius: f64,
    },
    #[step_data(workbench_field = "sketches", type_name = "Sketch", skip_update = true, skip_delete = true)]
    SketchLine {
        workbench_id: IDType,
        sketch_id: IDType,
        start: IDType,
        end: IDType,
    },
    // #[step_data(workbench_field = "sketches", type_name = "Sketch")]
    // SketchRectangle {
    //     workbench_id: IDType,
    //     sketch_id: IDType,
    //     start: IDType,
    //     end: IDType,
    // },
    // #[step_data(workbench_field = "solids", type_name = "Solid")]
    #[step_data(skip_update = true, skip_delete = true)]
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
