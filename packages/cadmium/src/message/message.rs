use std::cell::RefCell;
use std::rc::Rc;

use cadmium_macros::MessageEnum;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::step::{StepHash, StepResult};
use crate::IDType;

use super::idwrap::IDWrap;
use super::{MessageHandler, ProjectMessageHandler};

/// All the possible messages that can be sent to the backend.
///
/// Each variant is expected to implement the [`ProjectMessageHandler`] trait.
/// It gets called by the `MessageEnum` derive macro which auto-implements the
/// `handle` method.
#[derive(MessageEnum, Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
#[serde(tag = "type")]
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

impl Message {
	pub fn recalculate(
		&self,
		workbench: Rc<RefCell<crate::workbench::Workbench>>,
	) -> anyhow::Result<Option<(IDType, StepResult)>> {
		match self {
			// TODO: Move inside the derive macro
			Message::WorkbenchRename(v) => v.inner().handle_message(workbench),
			Message::WorkbenchPointAdd(v) => v.inner().handle_message(workbench),
			Message::WorkbenchPlaneAdd(v) => v.inner().handle_message(workbench),
			Message::WorkbenchSketchAdd(v) => v.inner().handle_message(workbench),
			Message::WorkbenchSketchSetPlane(v) => v.inner().handle_message(workbench),
			Message::WorkbenchPointUpdate(v) => v.inner().handle_message(workbench),

			Message::SketchAddPoint(v) => v.inner().handle_message(workbench),
			Message::SketchAddArc(v) => v.inner().handle_message(workbench),
			Message::SketchAddCircle(v) => v.inner().handle_message(workbench),
			Message::SketchAddLine(v) => v.inner().handle_message(workbench),
			Message::SketchAddRectangle(v) => v.inner().handle_message(workbench),
			Message::SketchDeletePrimitive(v) => v.inner().handle_message(workbench),

			Message::FeatureExtrusionAdd(v) => v.inner.handle_message(workbench),
			_ => Ok(None),
		}
	}
}

/// The result of a message handling operation.
#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MessageResult {
	// TODO: Add more data fields and not a blanket string
	/// Whether the operation was successful or not.
	pub success: bool,
	/// The data returned by the operation.
	///
	/// Could be "null", contain an error message (in case of `success == false`) or valid JSON data.
	pub data: String,
}

impl From<anyhow::Result<Option<StepHash>>> for MessageResult {
	fn from(result: anyhow::Result<Option<StepHash>>) -> Self {
		match result {
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

impl From<crate::error::CADmiumError> for MessageResult {
	fn from(e: crate::error::CADmiumError) -> Self {
		Self {
			success: false,
			data: e.to_string(),
		}
	}
}
