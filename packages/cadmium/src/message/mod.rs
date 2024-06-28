use crate::step::{StepHash, StepResult};
use crate::IDType;

pub mod idwrap;
mod message;

pub use message::{Message, MessageResult};
use serde::{Deserialize, Serialize};
use wasm_bindgen::convert::RefFromWasmAbi;

/// A trait for types that can be identified by a hash and a parent.
///
/// For example a [`Sketch`](crate::isketch::ISketch), given that it has a
/// [`Workbench`](crate::workbench::Workbench) as a parent and is identified by
/// the hash of an [`AddSketch`](crate::workbench::AddSketch) message.
pub trait Identifiable: Sized {
	type Parent;
	const ID_NAME: &'static str;
	fn from_parent_id(parent: &Self::Parent, hash: StepHash) -> Result<Self, anyhow::Error>;
}

/// A trait for types that can handle messages without the identity-dereference mechanism.
///
/// For example a [`ProjectRename`](crate::project::ProjectRename) message can be handled
/// by the [`Project`](crate::project::Project) without needing to dereference the parent.
pub trait ProjectMessageHandler: RefFromWasmAbi {
	fn handle_project_message(
		&self,
		project: &mut crate::project::Project,
	) -> anyhow::Result<Option<StepHash>>;
}

/// A trait for types that can handle messages and need a parent instance to do so.
///
/// For example an [`AddPoint`](crate::workbench::AddPoint) message needs a
/// [`Workbench`](crate::workbench::Workbench) to add the point to.
pub trait MessageHandler: Serialize + for<'de> Deserialize<'de> + RefFromWasmAbi {
	type Parent: Identifiable;
	fn handle_message(&self, item: Self::Parent) -> anyhow::Result<Option<(IDType, StepResult)>>;
}
