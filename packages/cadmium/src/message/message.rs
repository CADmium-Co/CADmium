use cadmium_macros::MessageEnum;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::IDType;

use super::ProjectMessageHandler;
use super::idwrap::IDWrap;

#[derive(MessageEnum, Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub enum Message {
    ProjectRename(crate::project::ProjectRename),
    WorkbenchRename(IDWrap<crate::workbench::WorkbenchRename>),
    WorkbenchPointAdd(IDWrap<crate::workbench::AddPoint>),
    WorkbenchPlaneAdd(IDWrap<crate::workbench::AddPlane>),
    WorkbenchSketchAdd(IDWrap<crate::workbench::AddSketch>),
    WorkbenchPointUpdate(IDWrap<IDWrap<crate::solid::point::WorkbenchPointUpdate>>),

    SketchAddPoint(IDWrap<IDWrap<crate::isketch::AddPoint>>),
    SketchAddArc(IDWrap<IDWrap<crate::isketch::AddArc>>),
    SketchAddCircle(IDWrap<IDWrap<crate::isketch::AddCircle>>),
    SketchAddLine(IDWrap<IDWrap<crate::isketch::AddLine>>),
    SketchDeletePrimitive(IDWrap<IDWrap<crate::isketch::DeletePrimitive>>),
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MessageResult {
    pub success: bool,
    pub data: String,
}

impl From<anyhow::Result<Option<IDType>>> for MessageResult {
    fn from(result: anyhow::Result<Option<IDType>>) -> Self {
        match result {
            // TODO: The Success should be a stable enum
            Ok(msg) => Self {
                success: true,
                data: if let Some(id) = msg { id.to_string() } else { "null".to_string() }
            },
            Err(e) => Self {
                success: false,
                data: e.backtrace().to_string()
            },
        }
    }
}
