use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::message::{Identifiable, Message, MessageHandler};
use crate::workbench::Workbench;
use crate::{interop, IDType};

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub id: IDType,
    pub name: String,
    pub suppressed: bool,
    pub data: Message,
    pub interop_node: Option<interop::node::Node>,
}

impl Step {
    pub fn new(id: IDType, data: Message, interop_node: Option<interop::Node>) -> Self {
        Step {
            id,
            name: format!("{}-{}", data, id),
            suppressed: false,
            data,
            interop_node,
        }
    }

    pub fn unique_id(&self) -> String {
        format!("{}", self)
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.data, self.id)
    }
}


impl Identifiable for Rc<RefCell<Step>> {
    type Parent = Rc<RefCell<Workbench>>;
    const ID_NAME: &'static str = "step_id";

    fn from_parent_id(parent: &Self::Parent, id: IDType) -> anyhow::Result<Self> {
        Ok(parent
            .borrow()
            .history.get(id as usize)
            .ok_or(anyhow::anyhow!("No step with ID {} exists in the current workbench", id))?
            .clone())
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct Rename {
    pub new_name: String,
}

impl MessageHandler for Rename {
    type Parent = Rc<RefCell<Step>>;
    fn handle_message(&self, step_ref: Self::Parent) -> anyhow::Result<Option<(IDType, interop::Node)>> {
        let mut step = step_ref.borrow_mut();
        step.name = self.new_name.clone();
        Ok(None)
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct Delete {
    pub step_id: IDType,
}

impl MessageHandler for Delete {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, workbench_ref: Self::Parent) -> anyhow::Result<Option<(IDType, interop::Node)>> {
        let mut workbench = workbench_ref.borrow_mut();
        workbench.history.remove(self.step_id as usize);
        Ok(None)
    }
}
