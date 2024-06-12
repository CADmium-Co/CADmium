use std::cell::RefCell;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use tsify_next::Tsify;
use wasm_bindgen::convert::RefFromWasmAbi;

use crate::step::StepResult;
use crate::workbench::Workbench;
use crate::IDType;

mod de;
mod ser;

use super::{Identifiable, MessageHandler, ProjectMessageHandler};

#[derive(Tsify, Debug, Clone)]
#[tsify(from_wasm_abi)]
pub struct IDWrap<T: Clone + Serialize + DeserializeOwned + RefFromWasmAbi> {
    pub id: u64,
    pub inner: T,
}

impl<T: Clone + Serialize + DeserializeOwned + RefFromWasmAbi> IDWrap<T> {
    pub fn new(id: IDType, h: T) -> Self {
        Self { id, inner: h }
    }

    pub fn id(&self) -> IDType {
        self.id
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

// First level message handler
impl<'a, T> ProjectMessageHandler for IDWrap<T>
where
    T: MessageHandler<Parent = Rc<RefCell<Workbench>>>
        + Clone
        + Serialize
        + DeserializeOwned
        + RefFromWasmAbi,
    crate::message::message::Message: From<Self>,
{
    fn handle_project_message(
        &self,
        project: &mut crate::project::Project,
    ) -> anyhow::Result<Option<IDType>> {
        let wb_cell = T::Parent::from_parent_id(project, self.id)?;
        let result = self.inner.handle_message(wb_cell.clone())?;
        let node = if let Some((_id, node)) = result {
            node
        } else {
            StepResult::Empty
        };

        let mut wb = wb_cell.borrow_mut();
        wb.add_message_step(&self.clone().into(), node);

        // Return the step ID (hash) instead of the message handler returned ID
        Ok(wb.history.last().map(|step| step.borrow().hash()).clone())
    }
}

// Second level message handler
impl<T, C, P> MessageHandler for IDWrap<T>
where
    T: MessageHandler<Parent = C> + Clone + Serialize + DeserializeOwned + RefFromWasmAbi,
    C: Identifiable<Parent = P>,
    P: Identifiable,
{
    type Parent = C::Parent;
    fn handle_message(&self, parent: Self::Parent) -> anyhow::Result<Option<(IDType, StepResult)>> {
        let msg_parent = C::from_parent_id(&parent, self.id)?;
        self.inner.handle_message(msg_parent)
    }
}
