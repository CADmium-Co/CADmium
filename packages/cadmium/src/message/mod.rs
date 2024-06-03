use crate::IDType;

pub mod idwrap;
pub mod message;

pub use message::{Message, MessageResult};
use serde::{Deserialize, Serialize};

pub trait Identifiable: Sized {
    type Parent;
    const ID_NAME: &'static str;
    fn from_parent_id(parent: &Self::Parent, id: IDType) -> Result<Self, anyhow::Error>;
}

pub trait ProjectMessageHandler: wasm_bindgen::convert::RefFromWasmAbi {
    fn handle_project_message(&self, project: &mut crate::project::Project) -> anyhow::Result<Option<IDType>>;
}

pub trait MessageHandler: Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi {
    type Parent: Identifiable;
    fn handle_message(&self, item: Self::Parent) -> anyhow::Result<Option<IDType>>;
}
