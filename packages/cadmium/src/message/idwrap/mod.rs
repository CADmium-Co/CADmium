//! [`IDWrap`] is the solution to the problem of needing some kind of identifier
//! for the ascendants of each message.
//!
//! For example the [`AddLine`](crate::isketch::primitive::AddLine)
//! message requires a `workbench_id` and a `sketch_id`. That would pollute all
//! the message structs with the exact same fields and code to handle them.
//!
//! Instead, the [`IDWrap`] struct is used to wrap the message and provide the
//! necessary identifier - it accepts a generic type that implements the
//! [`MessageHandler`] trait (the `inner` field) and adds an `id` field to
//! store the identifier.
//!
//! That though introduces some problems:
//! - Now serde (and WASM by extent) expects the `id` field to be present in the
//!  serialized message, which is not the case for the original message.
//! - The resulting rust API is quite cumbersome to use.
//!
//! The first is easily solved by custom serialization and deserialization
//! implementations for the [`IDWrap`] struct. That way instead of expecting
//!
//! ```json
//! { id: 0, inner: { x: 0.0, y: 0.0, z: 0.0 } }
//! ```
//!
//! we expect
//!
//! ```json
//! { workbench_id: 0, x: 0.0, y: 0.0, z: 0.0 }
//! ```
//!
//! The second problem is not solved yet and currently constructing and handling
//! a message looks like the example of the [`IDWrapable`] trait.

use std::cell::RefCell;
use std::rc::Rc;

use log::warn;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tsify_next::Tsify;
use wasm_bindgen::convert::RefFromWasmAbi;

use crate::step::{StepHash, StepResult};
use crate::workbench::Workbench;
use crate::IDType;

mod de;
mod ser;

use super::{Identifiable, MessageHandler, ProjectMessageHandler};

/// A wrapper struct that adds an identifier to a message handler struct.
///
/// For more check out the module level documentation of [`idwrap`](super::idwrap).
#[derive(Tsify, Debug, Clone)]
#[tsify(from_wasm_abi)]
pub struct IDWrap<T: Clone + Serialize + DeserializeOwned + RefFromWasmAbi> {
    pub id: StepHash,
    pub inner: T,
}

impl<T: Clone + Serialize + DeserializeOwned + RefFromWasmAbi> IDWrap<T> {
    pub fn new(id: StepHash, h: T) -> Self {
        Self { id, inner: h }
    }

    pub fn id(&self) -> StepHash {
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
    ) -> anyhow::Result<Option<StepHash>> {
        let wb_cell = T::Parent::from_parent_id(project, self.id)?;
        let result = self.inner.handle_message(wb_cell.clone())?;
        let (id, node) = if let Some((id, node)) = result {
            (Some(id), node)
        } else {
            (None, StepResult::Empty)
        };

        let mut wb = wb_cell.borrow_mut();
        wb.add_message_step(&self.clone().into(), node);
        let hash = wb.history.last().map(|step| step.borrow().hash()).clone();

        if let Some(id) = id {
            if let Some(hash) = hash {
                crate::ID_MAP.with_borrow_mut(|m| m.insert(hash, id));
            } else {
                warn!("IDWrap::handle_project_message: IDWrap returned an ID, but no hash was found in the workbench history");
            }
        }

        // Return the step ID (hash) instead of the message handler returned ID
        Ok(hash)
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

/// A helper trait that allows any type that implements [`MessageHandler`] to be wrapped in an [`IDWrap`].
///
/// # Examples
///
/// ```rust
/// use cadmium::{create_project, get_project, send_message};
/// use cadmium::message::idwrap::IDWrapable;
/// use cadmium::message::ProjectMessageHandler;
/// use cadmium::step::StepHash;
/// use cadmium::workbench::AddPoint;
///
/// let project_id = create_project("My Project");
/// // When creating a project a workbench is always created
/// // Workbenches are using the "StepHash" as an index for compatibility
/// let workbench_hash = StepHash::from_int(0);
/// let mut project = get_project(project_id).unwrap();
/// // The following just describes the point we want to add
/// let message = AddPoint { x: 0.0, y: 1.0, z: 2.0 };
/// // Now we describe which parent the point should be added to (a workbench in this case)
/// let message_idwrapped = message.id_wrap(workbench_hash);
/// let result = message_idwrapped.handle_project_message(&mut project).unwrap().unwrap();
///
/// println!("The new point has the hash: {}", result);
/// ```
pub trait IDWrapable<T: MessageHandler + Clone + Serialize + DeserializeOwned + RefFromWasmAbi> {
    fn id_wrap(self, id: StepHash) -> IDWrap<T>;
}

impl<T, C> IDWrapable<T> for T
where
    T: MessageHandler<Parent = C> + Clone + Serialize + DeserializeOwned + RefFromWasmAbi,
    C: Identifiable,
{
    fn id_wrap(self, id: StepHash) -> IDWrap<T> {
        IDWrap { id, inner: self }
    }
}
