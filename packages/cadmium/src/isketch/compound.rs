use std::fmt::Debug;

use isotope::primitives::PrimitiveCell;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use super::compound_rectangle;

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

impl Compound {
    pub fn as_compound_like(&self) -> &dyn CompoundLike {
        match self {
            Compound::Rectangle(rectangle) => rectangle,
        }
    }
}
