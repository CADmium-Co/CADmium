use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::solid::extrusion::Direction;
use crate::project::Project;
use crate::step::StepData;
use crate::IDType;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MessageResult {
    pub success: bool,
    pub data: String,
}

impl From<Result<String, anyhow::Error>> for MessageResult {
    fn from(result: Result<String, anyhow::Error>) -> Self {
        match result {
            // TODO: The Success should be a stable enum
            Ok(msg) => Self {
                success: true,
                data: format!("{{ {} }}", msg)
            },
            Err(e) => Self {
                success: false,
                data: e.backtrace().to_string()
            },
        }
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Message {
    StepAction {
        name: String,
        data: StepData,
    },
    RenameProject {
        new_name: String,
    },
    SetSketchPlane {
        workbench_id: u64,
        sketch_id: IDType,
        plane_id: IDType,
    },
    UpdateExtrusion {
        workbench_id: IDType,
        extrusion_name: String,
        extrusion_id: IDType,
        sketch_id: IDType,
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
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => format!("Error: {}", e),
        }
    }

    pub fn from_json(json: &str) -> Result<Message, anyhow::Error> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn handle(&self, project: &mut Project) -> Result<String, anyhow::Error> {
        match self {
            Message::StepAction {
                name,
                data,
            } => {
                let id = data.do_action(project, name.clone())?;
                Ok(format!("\"id\": \"{}\"", id))
            }
            Message::RenameProject { new_name } => {
                project.name = new_name.to_owned();
                Ok(format!("\"name\": \"{}\"", new_name))
            }
            Message::SetSketchPlane {
                workbench_id,
                sketch_id,
                plane_id: pid,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let plane = workbench.planes.iter().find(|(p, _)| *p == pid).ok_or(anyhow::anyhow!(""))?;
                let sketch_ref = workbench.get_sketch_by_id(*sketch_id)?;
                let mut sketch = sketch_ref.borrow_mut();
                sketch.plane = plane.1.clone();

                Ok(format!("\"plane_id\": \"{}\"", plane.0))
            }
            Message::UpdateExtrusion {
                workbench_id,
                extrusion_name: _extrusion_name,
                extrusion_id,
                sketch_id,
                face_ids,
                length:_,
                offset:_,
                direction:_,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id(*sketch_id)?;
                let _faces = sketch
                    .borrow()
                    .faces()
                    .iter()
                    .enumerate()
                    .filter_map(|(k, v)| {
                        if face_ids.contains(&(k as u64)) {
                            Some(v.clone())
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>();
                let _extrusion = workbench.solids.get(extrusion_id).ok_or(anyhow::anyhow!("Could not find extrusion ID!"))?.borrow_mut();

                todo!("Update Extrusion")
                // let new_extrusion = extrusion::Extrusion::new(faces, sketch, *length, *offset, *direction, extrusion.mode).to_feature().as_solid_like();

                // let as_step_data = StepData::Extrusion { extrusion };
                // workbench.update_step_data(extrusion_id, as_step_data);
                // Ok(format!("\"id\": \"{}\"", extrusion_id))
            }
            Message::UpdateExtrusionLength {
                workbench_id:_,
                extrusion_name:_,
                length:_,
            } => {
                // let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                // let step = workbench.get_step_mut(&extrusion_name)?;

                // if let StepData::Extrusion { extrusion } = &mut step.data {
                //     extrusion.length = *length;
                //     return Ok(format!("\"length\": {}", length));
                // }

                // Err(CADmiumError::IncorrectStepDataType("Extrusion".to_owned()).into())
                todo!("Update Extrusion Length")
            }
        }
    }
}
