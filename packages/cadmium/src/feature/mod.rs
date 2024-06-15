use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use serde::Deserialize;
use serde::Serialize;
use tsify_next::Tsify;

pub mod extrusion;
pub mod helpers;
pub mod point;
pub mod prelude;
pub mod solid;

use prelude::*;

pub trait SolidLike: Debug {
    fn references(&self) -> Vec<Rc<RefCell<Feature>>>;
    fn get_truck_solids(&self) -> anyhow::Result<Vec<TruckClosedSolid>>;
    fn to_feature(&self) -> Feature;

    fn to_solids(&self) -> anyhow::Result<Vec<Solid>> {
        let truck_solids = self.get_truck_solids()?;

        Ok(truck_solids
            .iter()
            .map(|truck_solid| Solid::from_truck_solid("".to_owned(), truck_solid.clone()))
            .collect())
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[non_exhaustive]
pub enum Feature {
    Extrusion(extrusion::Extrusion),
}

impl Feature {
    pub fn as_solid_like(&self) -> &dyn SolidLike {
        match self {
            Feature::Extrusion(extrusion) => extrusion,
        }
    }
}
