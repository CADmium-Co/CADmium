use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::message::{Identifiable, Message};
use crate::workbench::Workbench;

pub mod actions;
mod hash;
mod result;
pub mod sketch_action;

pub use hash::StepHash;
pub use result::StepResult;

/// A step describes a single operation that takes place in a [`Workbench`].
///
/// An operation is often a transformation of the part in the workbench,
/// but it can also be a transformation of the meta-data of the workbench itself.
///
/// Each step is comprised by a [`Message`] that describes the operation,
/// the [`StepHash`] of it and the [`StepResult`] of the operation.
///
/// It is safe to assume that the step `data` field will never change.
#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub hash: StepHash,
    pub name: String,
    pub suppressed: bool,
    pub data: Message,
    pub result: StepResult,
}

impl Step {
    pub fn new(data: Message, result: StepResult) -> Self {
        let hash = (&data).into();
        Self {
            hash: hash,
            name: format!("{}-{}", data, hash),
            suppressed: false,
            data,
            result: result,
        }
    }

    pub fn hash(&self) -> StepHash {
        self.hash
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.data, self.hash)
    }
}

impl Identifiable for Rc<RefCell<Step>> {
    type Parent = Rc<RefCell<Workbench>>;
    const ID_NAME: &'static str = "step_id";

    fn from_parent_id(parent: &Self::Parent, hash: StepHash) -> anyhow::Result<Self> {
        Ok(parent
            .borrow()
            .get_step_by_hash(hash)
            .ok_or(anyhow::anyhow!(
                "No step with hash {} exists in the current workbench",
                hash
            ))?
            .clone())
    }
}
