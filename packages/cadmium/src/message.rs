use itertools::Itertools as _;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::archetypes::PlaneDescription;
use crate::error::CADmiumError;
use crate::extrusion::{Direction, Extrusion, ExtrusionMode};
use crate::project::Project;
use crate::step::StepData;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum MessageResult {
    #[serde(rename = "success")]
    Success(String),
    #[serde(rename = "error")]
    Error(String),
}

impl From<Result<String, anyhow::Error>> for MessageResult {
    fn from(result: Result<String, anyhow::Error>) -> Self {
        match result {
            // TODO: The Success should be a stable enum
            Ok(msg) => MessageResult::Success(format!("{{ {} }}", msg)),
            Err(e) => MessageResult::Error(e.backtrace().to_string()),
        }
    }
}

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
    DeleteStep {
        workbench_id: u64,
        step_name: String,
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
            Message::RenameProject { new_name } => {
                project.name = new_name.to_owned();
                Ok(format!("\"name\": \"{}\"", new_name))
            }
            Message::RenameWorkbench {
                workbench_id,
                new_name,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                workbench.name = new_name.to_owned();
                Ok(format!("\"name\": \"{}\"", new_name))
            }
            Message::RenameStep {
                workbench_id,
                step_id,
                new_name,
            } => {
                project
                    .get_workbench_by_id_mut(*workbench_id)?
                    .history
                    .get_mut(*step_id as usize)
                    .ok_or(CADmiumError::StepIDNotFound(step_id.to_string()))?
                    .name = new_name.to_owned();

                Ok(format!("\"name\": \"{}\"", new_name))
            }
            Message::DeleteLines {
                workbench_id,
                sketch_id,
                line_ids,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id_mut(sketch_id)?;
                for line_id in line_ids {
                    sketch.delete_line_segment(*line_id);
                }
                Ok("".to_owned())
            }
            Message::DeleteArcs {
                workbench_id,
                sketch_id,
                arc_ids,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id_mut(sketch_id)?;
                for arc_id in arc_ids {
                    sketch.delete_arc(*arc_id);
                }
                Ok("".to_owned())
            }
            Message::DeleteCircles {
                workbench_id,
                sketch_id,
                circle_ids,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id_mut(sketch_id)?;
                for circle_id in circle_ids {
                    sketch.delete_circle(*circle_id);
                }
                Ok("".to_owned())
            }
            Message::NewPointOnSketch2 {
                workbench_id,
                sketch_id,
                x,
                y,
                hidden,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id_mut(sketch_id)?;
                let point_id;
                if *hidden {
                    point_id = sketch.add_hidden_point(*x, *y);
                } else {
                    point_id = sketch.add_point(*x, *y);
                }

                Ok(format!("\"id\": \"{}\"", point_id))
            }
            Message::NewCircleBetweenPoints {
                workbench_id,
                sketch_id,
                center_id,
                edge_id,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id_mut(sketch_id)?;
                let circle_id = sketch.add_circle_between_points(*center_id, *edge_id);
                Ok(format!("\"id\": \"{}\"", circle_id))
            }
            Message::NewRectangleBetweenPoints {
                workbench_id,
                sketch_id,
                start_id,
                end_id,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id_mut(sketch_id)?;
                let (point_ids, line_ids) = sketch.add_rectangle_between_points(*start_id, *end_id);
                Ok(format!(
                    "\"point_ids\": [{}], \"line_ids\": [{}]",
                    point_ids.iter().join(","),
                    line_ids.iter().join(",")
                ))
            }
            Message::NewPointOnSketch {
                workbench_id,
                sketch_id,
                point_id,
                x,
                y,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id_mut(sketch_id)?;
                sketch.add_point_with_id(*x, *y, *point_id)?;
                Ok("".to_owned())
            }
            Message::NewLineOnSketch {
                workbench_id,
                sketch_id,
                start_point_id,
                end_point_id,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_by_id_mut(sketch_id)?;
                let line_id = sketch.add_segment(*start_point_id, *end_point_id);
                Ok(format!("\"id\": \"{}\"", line_id))
            }
            Message::DeleteLineSegment {
                workbench_id,
                sketch_name,
                line_segment_id,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_mut(sketch_name)?;
                sketch.delete_line_segment(*line_segment_id);
                Ok("".to_owned())
            }
            Message::StepSketch {
                workbench_id,
                sketch_name,
                steps,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_mut(sketch_name)?;
                let mut max_change = 0.0;
                for _ in 0..*steps {
                    max_change = sketch.take_a_step();
                }
                Ok(format!("{}", max_change))
            }
            Message::SolveSketch {
                workbench_id,
                sketch_name,
                max_steps,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let sketch = workbench.get_sketch_mut(sketch_name)?;
                sketch.solve(*max_steps);
                Ok("".to_owned())
            }
            Message::NewSketchOnPlane {
                workbench_id,
                sketch_name,
                plane_id,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;

                let new_sketch_id = workbench.add_sketch_to_plane(&sketch_name, &plane_id);
                Ok(format!("\"sketch_id\": \"{}\"", new_sketch_id))
            }
            Message::SetSketchPlane {
                workbench_id,
                sketch_id,
                plane_id: pid,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let step = workbench.get_step_by_id_mut(&sketch_id)?;
                let plane_description: &mut PlaneDescription = if let StepData::Sketch { plane_description, .. } = &mut step.data {
                    plane_description
                } else {
                    return Err(CADmiumError::IncorrectStepDataType("Sketch".to_owned()).into());
                };

                match plane_description {
                    PlaneDescription::PlaneId(ref mut plane_id) => {
                        *plane_id = pid.to_owned();
                        Ok(format!("\"plane_id\": \"{}\"", pid))
                    }
                    _ => Err(CADmiumError::NotImplemented.into())
                }
            }
            Message::DeleteStep {
                workbench_id,
                step_name,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let index = workbench.history
                    .iter()
                    .position(|step| step.name == *step_name)
                    .ok_or(CADmiumError::StepNameNotFound(step_name.to_owned()))?;

                // Since the index was found and not given by the user, it should be safe to remove
                workbench.history.remove(index);
                Ok("".to_owned())
            }
            Message::NewExtrusion {
                workbench_id,
                extrusion_name,
                sketch_id,
                face_ids,
                length,
                offset,
                direction,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let extrusion = Extrusion::new(
                    sketch_id.to_owned(),
                    face_ids.to_owned(),
                    *length,
                    *offset,
                    direction.to_owned(),
                    ExtrusionMode::New,
                );
                let extrusion_id = workbench.add_extrusion(extrusion_name, extrusion);
                Ok(format!("\"id\": \"{}\"", extrusion_id))
            }
            Message::UpdateExtrusion {
                workbench_id,
                extrusion_name: _extrusion_name,
                extrusion_id,
                sketch_id,
                face_ids,
                length,
                offset,
                direction,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let extrusion = Extrusion::new(
                    sketch_id.to_owned(),
                    face_ids.to_owned(),
                    *length,
                    *offset,
                    direction.to_owned(),
                    ExtrusionMode::New,
                );
                let as_step_data = StepData::Extrusion { extrusion };
                workbench.update_step_data(extrusion_id, as_step_data);
                Ok(format!("\"id\": \"{}\"", extrusion_id))
            }
            Message::UpdateExtrusionLength {
                workbench_id,
                extrusion_name,
                length,
            } => {
                let workbench = project.get_workbench_by_id_mut(*workbench_id)?;
                let step = workbench.get_step_mut(&extrusion_name)?;

                if let StepData::Extrusion { extrusion } = &mut step.data {
                    extrusion.length = *length;
                    return Ok(format!("\"length\": {}", length));
                }

                Err(CADmiumError::IncorrectStepDataType("Extrusion".to_owned()).into())
            }
        }
    }
}
