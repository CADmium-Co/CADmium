use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::fmt::Debug;
use std::rc::Rc;

use serde::Deserialize;
use serde::Serialize;
use tsify_next::Tsify;

use super::*;

pub trait SolidLike: Debug {
    fn references(&self) -> Vec<FeatureCell>;
    fn get_truck_solids(&self) -> anyhow::Result<Vec<TruckClosedSolid>>;
    fn to_feature(&self) -> Feature;

    fn to_solids(&self) -> anyhow::Result<Vec<Solid>> {
        let truck_solids = self.get_truck_solids()?;

        Ok(truck_solids.iter().map(|truck_solid| {
            Solid::from_truck_solid("".to_owned(), truck_solid.clone())
        }).collect())
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum FeatureCell {
    Extrusion(Rc<RefCell<extrusion::Extrusion>>),
}

impl FeatureCell {
    pub fn borrow(&self) -> Ref<dyn SolidLike> {
        match self {
            FeatureCell::Extrusion(e) => e.borrow(),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<dyn SolidLike > {
        match self {
            FeatureCell::Extrusion(e) => e.borrow_mut(),
        }
    }

    pub fn as_ptr(&self) -> *const dyn SolidLike {
        match self {
            FeatureCell::Extrusion(e) => e.as_ptr(),
        }
    }
}

impl PartialEq for FeatureCell {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.as_ptr(), other.as_ptr())
    }
}
