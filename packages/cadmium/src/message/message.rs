use cadmium_macros::MessageEnum;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::step::StepHash;

use super::idwrap::IDWrap;
use super::ProjectMessageHandler;

#[derive(MessageEnum, Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub enum Message {
    ProjectRename(crate::project::ProjectRename),
    WorkbenchRename(IDWrap<crate::workbench::WorkbenchRename>),
    WorkbenchPointAdd(IDWrap<crate::workbench::AddPoint>),
    WorkbenchPlaneAdd(IDWrap<crate::workbench::AddPlane>),
    WorkbenchSketchAdd(IDWrap<crate::workbench::AddSketch>),
    WorkbenchSketchSetPlane(IDWrap<crate::workbench::SetSketchPlane>),
    WorkbenchPointUpdate(IDWrap<IDWrap<crate::feature::point::WorkbenchPointUpdate>>),

    SketchAddPoint(IDWrap<IDWrap<crate::isketch::primitive::SketchAddPointMessage>>),
    SketchAddArc(IDWrap<IDWrap<crate::isketch::primitive::AddArc>>),
    SketchAddCircle(IDWrap<IDWrap<crate::isketch::primitive::AddCircle>>),
    SketchAddLine(IDWrap<IDWrap<crate::isketch::primitive::AddLine>>),
    SketchAddRectangle(IDWrap<IDWrap<crate::isketch::compound_rectangle::Add>>),
    SketchDeletePrimitive(IDWrap<IDWrap<crate::isketch::primitive::DeletePrimitive>>),

    FeatureExtrusionAdd(IDWrap<crate::feature::extrusion::Add>),

    StepRename(IDWrap<IDWrap<crate::step::actions::Rename>>),
    StepDelete(IDWrap<crate::step::actions::Delete>),
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MessageResult {
    pub success: bool,
    pub data: String,
}

impl From<anyhow::Result<Option<StepHash>>> for MessageResult {
    fn from(result: anyhow::Result<Option<StepHash>>) -> Self {
        match result {
            // TODO: The Success should be a stable enum
            Ok(msg) => Self {
                success: true,
                data: if let Some(id) = msg {
                    id.to_string()
                } else {
                    "null".to_string()
                },
            },
            Err(e) => Self {
                success: false,
                data: e.to_string() + "\n\n" + e.backtrace().to_string().as_str(),
            },
        }
    }
}
