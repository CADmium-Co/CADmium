use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use isotope::primitives::PrimitiveCell;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::message::{Identifiable, MessageHandler};
use crate::{interop, IDType};

use super::{compound_rectangle, ISketch};

pub trait CompoundLike: Debug {
    fn references(&self) -> Vec<PrimitiveCell>;
    fn created_references(&self) -> Vec<PrimitiveCell>;
    fn populate_created_references(
        &self,
        sketch: &mut isotope::sketch::Sketch,
    ) -> anyhow::Result<()> {
        for reference in self.created_references() {
            sketch.add_primitive(reference)?;
        }
        Ok(())
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[non_exhaustive]
pub enum Compound {
    Rectangle(compound_rectangle::Rectangle),
}

impl Identifiable for Rc<RefCell<Compound>> {
    type Parent = Rc<RefCell<ISketch>>;
    const ID_NAME: &'static str = "compound_id";

    fn from_parent_id(parent: &Self::Parent, id: IDType) -> anyhow::Result<Self> {
        Ok(parent
            .borrow()
            .compounds
            .get(&id)
            .ok_or(anyhow::anyhow!("No feature with ID {} was found", id))?
            .clone())
    }
}

impl Compound {
    pub fn as_compound_like(&self) -> &dyn CompoundLike {
        match self {
            Compound::Rectangle(rectangle) => rectangle,
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct DeleteCompound {
    id: IDType,
}

impl MessageHandler for DeleteCompound {
    type Parent = Rc<RefCell<ISketch>>;
    fn handle_message(
        &self,
        sketch_ref: Rc<RefCell<ISketch>>,
    ) -> anyhow::Result<Option<(IDType, interop::Node)>> {
        let mut isketch = sketch_ref.borrow_mut();
        let mut sketch = isketch.sketch.borrow_mut();
        let compound = isketch
            .compounds
            .get(&self.id)
            .ok_or(anyhow::anyhow!("No compound with ID {} was found", self.id))?;

        for reference in compound.borrow().as_compound_like().created_references() {
            let id = sketch.get_primitive_id(&reference).ok_or(anyhow::anyhow!(
                "Failed to find primitive with reference {:?}",
                reference
            ))?;
            sketch.delete_primitive(id)?;
        }
        drop(sketch);

        isketch.compounds.remove(&self.id);
        Ok(None)
    }
}
