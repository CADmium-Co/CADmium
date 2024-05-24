use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::extrusion::Direction;


#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Message {
    RenameWorkbench {
        workbench_id: u64,
        new_name: String,
    },
    RenameStep {
        workbench_id: u64,
        step_id: u64,
        new_name: String,
    },
    RenameProject {
        new_name: String,
    },
    DeleteLines {
        workbench_id: u64,
        sketch_id: String,
        line_ids: Vec<u64>,
    },
    DeleteArcs {
        workbench_id: u64,
        sketch_id: String,
        arc_ids: Vec<u64>,
    },
    DeleteCircles {
        workbench_id: u64,
        sketch_id: String,
        circle_ids: Vec<u64>,
    },
    NewPointOnSketch {
        workbench_id: u64,
        sketch_id: String,
        point_id: u64,
        x: f64,
        y: f64,
    },
    NewPointOnSketch2 {
        workbench_id: u64,
        sketch_id: String,
        x: f64,
        y: f64,
        hidden: bool,
    },
    NewCircleBetweenPoints {
        workbench_id: u64,
        sketch_id: String,
        center_id: u64,
        edge_id: u64,
    },
    NewRectangleBetweenPoints {
        workbench_id: u64,
        sketch_id: String,
        start_id: u64,
        end_id: u64,
    },
    NewLineOnSketch {
        workbench_id: u64,
        sketch_id: String,
        start_point_id: u64,
        end_point_id: u64,
    },
    DeleteLineSegment {
        workbench_id: u64,
        sketch_name: String,
        line_segment_id: u64,
    },
    StepSketch {
        workbench_id: u64,
        sketch_name: String,
        steps: u64,
    },
    SolveSketch {
        workbench_id: u64,
        sketch_name: String,
        max_steps: u64,
    },
    NewSketchOnPlane {
        workbench_id: u64,
        sketch_name: String,
        plane_id: String,
    },
    SetSketchPlane {
        workbench_id: u64,
        sketch_id: String,
        plane_id: String,
    },
    DeleteSketch {
        workbench_id: u64,
        sketch_name: String,
    },
    NewExtrusion {
        workbench_id: u64,
        extrusion_name: String,
        sketch_id: String,
        face_ids: Vec<u64>,
        length: f64,
        offset: f64,
        direction: Direction,
    },
    UpdateExtrusion {
        workbench_id: u64,
        extrusion_name: String,
        extrusion_id: String,
        sketch_id: String,
        face_ids: Vec<u64>,
        length: f64,
        offset: f64,
        direction: Direction,
    },
    UpdateExtrusionLength {
        workbench_id: u64,
        extrusion_name: String,
        length: f64,
    },
}

impl Message {
    pub fn as_json(&self) -> String {
        let result = serde_json::to_string(self);
        match result {
            Ok(json) => json,
            Err(e) => format!("Error: {}", e),
        }
    }

    pub fn from_json(json: &str) -> Result<Message, String> {
        let result = serde_json::from_str(json);
        match result {
            Ok(msg) => Ok(msg),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}
