use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::message::{Message, MessageHandler};
use crate::IDType;

pub trait Realizable: MessageHandler {
    fn realize(&self, parent: <Self as MessageHandler>::Parent) -> anyhow::Result<()>;
}

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

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub(crate) id: IDType,
    pub(crate) operation: StepOperation,
    pub(crate) name: String,
    pub(crate) suppressed: bool,
    pub(crate) data: Message,
}

impl Step {
    pub fn unique_id(&self) -> String {
        // TODO: Should use the type of StepData instead of name
        format!("{}:{}-{}", self.operation, self.name, self.id)
    }
}
