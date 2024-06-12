use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use xxhash_rust::xxh3::xxh3_64;

use crate::message::{Identifiable, Message};
use crate::workbench::Workbench;
use crate::IDType;

pub mod actions;
pub mod result;

pub use result::StepResult;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(transparent)]
#[repr(transparent)]
pub struct StepHash(pub u64);

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub hash: IDType,
    pub name: String,
    pub suppressed: bool,
    pub data: Message,
    pub result: StepResult,
}

impl Step {
    pub fn new(data: Message, result: StepResult) -> Self {
        let hash = xxh3_64(serde_json::to_string(&data).unwrap().as_bytes());
        Self {
            hash,
            name: format!("{}-{}", data, hash),
            suppressed: false,
            data,
            result: result,
        }
    }

    pub fn hash(&self) -> IDType {
        self.hash as IDType
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

    fn from_parent_id(parent: &Self::Parent, id: IDType) -> anyhow::Result<Self> {
        Ok(parent
            .borrow()
            .get_step_by_hash(id)
            .ok_or(anyhow::anyhow!(
                "No step with hash {} exists in the current workbench",
                id
            ))?
            .clone())
    }
}
