use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::message::MessageHandler;
use crate::workbench::Workbench;
use crate::IDType;

use super::{Step, StepResult};

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct Rename {
    pub new_name: String,
}

impl MessageHandler for Rename {
    type Parent = Rc<RefCell<Step>>;
    fn handle_message(
        &self,
        step_ref: Self::Parent,
    ) -> anyhow::Result<Option<(IDType, StepResult)>> {
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
    fn handle_message(
        &self,
        workbench_ref: Self::Parent,
    ) -> anyhow::Result<Option<(IDType, StepResult)>> {
        let mut workbench = workbench_ref.borrow_mut();
        workbench.history.remove(self.step_id as usize);
        Ok(None)
    }
}
