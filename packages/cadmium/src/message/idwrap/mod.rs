use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::IDType;

mod de;
mod ser;

use super::{Identifiable, MessageHandler, ProjectMessageHandler};

#[derive(Tsify, Debug, Clone)]
#[tsify(from_wasm_abi)]
pub struct IDWrap<T: Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi> {
    pub id: u64,
    pub inner: T,
}

impl<T: Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi> IDWrap<T> {
    pub fn new(id: IDType, h: T) -> Self {
        Self {
            id,
            inner: h,
        }
    }

    pub fn id(&self) -> IDType {
        self.id
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }
}

// First level message handler
impl<T, U> ProjectMessageHandler for IDWrap<T>
where
    T: MessageHandler<Parent = U> + Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi,
    U: Identifiable<Parent = crate::project::Project>,
{
    fn handle_project_message(&self, project: &mut crate::project::Project) -> anyhow::Result<Option<IDType>> {
        let prnt = U::from_parent_id(project, self.id)?;
        self.inner.handle_message(prnt)
    }
}

// Second level message handler
impl<T, C, P> MessageHandler for IDWrap<T>
where
    T: MessageHandler<Parent = C> + Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi,
    C: Identifiable<Parent = P>,
    P: Identifiable<Parent = crate::project::Project>,
{
    type Parent = C::Parent;
    fn handle_message(&self, parent: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let prnt = C::from_parent_id(&parent, self.id)?;
        self.inner.handle_message(prnt)
    }
}
