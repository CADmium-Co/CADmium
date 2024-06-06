use log::info;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription};
use crate::error::CADmiumError;
use crate::isketch::ISketch;
use crate::feature::Feature;
use crate::feature::point::Point3;
use crate::step::Step;
use crate::IDType;

use crate::message::*;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Workbench {
    pub name: String,
    pub history: Vec<Rc<RefCell<Step>>>,

    // These are free-standing points in 3D space, not part of sketches
    pub points: BTreeMap<IDType, Rc<RefCell<Point3>>>,
    pub points_next_id: IDType,

    pub planes: BTreeMap<IDType, Rc<RefCell<Plane>>>,
    pub planes_next_id: IDType,

    pub sketches: BTreeMap<IDType, Rc<RefCell<ISketch>>>,
    pub sketches_next_id: IDType,
    pub features: BTreeMap<IDType, Rc<RefCell<Feature>>>,
    pub features_next_id: IDType,
}

impl Workbench {
    pub fn new(name: &str) -> Self {
        info!("Creating new workbench: {:?}", name);
        let mut wb = Workbench {
            name: name.to_owned(),
            history: vec![],

            points: BTreeMap::new(),
            points_next_id: 1,
            planes: BTreeMap::new(),
            planes_next_id: 3,

            sketches: BTreeMap::new(),
            sketches_next_id: 0,
            features: BTreeMap::new(),
            features_next_id: 0,
        };

        wb.points.insert(0, Rc::new(RefCell::new(Point3::new(0.0, 0.0, 0.0))));
        wb.planes.insert(0, Rc::new(RefCell::new(Plane::front())));
        wb.planes.insert(1, Rc::new(RefCell::new(Plane::front())));
        wb.planes.insert(2, Rc::new(RefCell::new(Plane::front())));

        wb
    }

    pub fn get_first_plane_id(&self) -> Option<IDType> {
        if !self.planes.is_empty() {
            Some(self.planes.keys().next().unwrap().to_owned())
        } else {
            None
        }
    }

    pub fn get_last_plane_id(&self) -> Option<IDType> {
        if !self.planes.is_empty() {
            Some(self.planes.keys().last().unwrap().to_owned())
        } else {
            None
        }
    }

    pub fn get_sketch_by_id(&self, id: IDType) -> Result<Rc<RefCell<ISketch>>, CADmiumError> {
        self.sketches.get(&id).ok_or(CADmiumError::SketchIDNotFound(id)).cloned()
    }

    pub fn add_message_step(&mut self, message: &Message) {
        self.history.push(
            Rc::new(
                RefCell::new(
                    Step::new(self.history.len() as IDType, message.clone()))));
    }
}

impl Identifiable for Rc<RefCell<Workbench>> {
    type Parent = crate::project::Project;
    const ID_NAME: &'static str = "workbench_id";

    fn from_parent_id(parent: &crate::project::Project, id: IDType) -> anyhow::Result<Self> {
        Ok(parent.get_workbench_by_id(id)?)
    }
}

// Add to history any messages that have Workbench as the parent
// impl<T> MessageHandler for IDWrap<T>
// where
//     T: MessageHandler<Parent = Rc<RefCell<Workbench>>> + Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi
// {
//     type Parent = Rc<RefCell<Workbench>>;
//     fn handle_message(&self, workbench_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
//         let mut workbench = workbench_ref.borrow_mut();
//         workbench.add_message_step(&self);
//         self.handle_message(workbench_ref)
//     }

// }

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl MessageHandler for AddPoint {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, sketch_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let mut wb = sketch_ref.borrow_mut();

        let new_id = wb.points_next_id;
        wb.points.insert(new_id, Rc::new(RefCell::new(Point3::new(self.x, self.y, self.z))));
        wb.points_next_id += 1;
        Ok(Some(new_id))
    }
}


#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddPlane {
    plane: Plane,
    width: f64,
    height: f64,
}

impl MessageHandler for AddPlane {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, sketch_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let mut wb = sketch_ref.borrow_mut();

        let new_id = wb.planes_next_id;
        wb.planes.insert(new_id, Rc::new(RefCell::new(self.plane.clone())));
        wb.planes_next_id += 1;
        Ok(Some(new_id))
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddSketch {
    pub plane_description: PlaneDescription,
}

impl MessageHandler for AddSketch {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, workbench_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let mut wb = workbench_ref.borrow_mut();
        let sketch = ISketch::try_from_plane_description(&wb, &self.plane_description)?;

        let new_id = wb.sketches_next_id;
        wb.sketches.insert(new_id, Rc::new(RefCell::new(sketch)));
        wb.sketches_next_id += 1;
        Ok(Some(new_id))
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WorkbenchRename {
    pub new_name: String,
}

impl MessageHandler for WorkbenchRename {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, workbench_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let mut workbench = workbench_ref.borrow_mut();
        workbench.name = self.new_name.clone();
        Ok(None)
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct SetSketchPlane {
    pub sketch_id: IDType,
    pub plane_description: PlaneDescription,
}

impl MessageHandler for SetSketchPlane {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, workbench_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let wb = workbench_ref.borrow();

        let plane = match self.plane_description {
            PlaneDescription::PlaneId(plane_id) =>
                wb.planes.get(&plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?,
            PlaneDescription::SolidFace { solid_id: _, normal: _ } => todo!("Implement SolidFace"),
        }.clone();

        let sketch = wb.sketches.get(&self.sketch_id).ok_or(anyhow::anyhow!("Failed to find sketch with id {}", self.sketch_id))?;
        sketch.borrow_mut().plane = plane;

        Ok(None)
    }
}
