use cadmium_macros::NoRealize;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription};
use crate::error::CADmiumError;
use crate::isketch::ISketch;
use crate::solid::point::Point3;
use crate::solid::Solid;
use crate::step::Step;
use crate::IDType;

use crate::message::*;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Workbench {
    pub(crate) name: String,
    pub(crate) history: Vec<Step>,

    // These are free-standing points in 3D space, not part of sketches
    pub(crate) points: BTreeMap<IDType, Rc<RefCell<Point3>>>,
    pub(crate) points_next_id: IDType,

    pub(crate) planes: BTreeMap<IDType, Rc<RefCell<Plane>>>,
    pub(crate) planes_next_id: IDType,

    pub(crate) sketches: BTreeMap<IDType, Rc<RefCell<ISketch>>>,
    pub(crate) sketches_next_id: IDType,
    pub(crate) solids: BTreeMap<IDType, Rc<RefCell<Solid>>>,
    pub(crate) solids_next_id: IDType,
}

impl Workbench {
    pub fn new(name: &str) -> Self {
        println!("Creating new workbench: {:?}", name);
        let mut wb = Workbench {
            name: name.to_owned(),
            history: vec![],

            points: BTreeMap::new(),
            points_next_id: 1,
            planes: BTreeMap::new(),
            planes_next_id: 3,

            sketches: BTreeMap::new(),
            sketches_next_id: 0,
            solids: BTreeMap::new(),
            solids_next_id: 0,
        };

        wb.points.insert(0, Rc::new(RefCell::new(Point3::new(0.0, 0.0, 0.0)))).unwrap();
        wb.planes.insert(0, Rc::new(RefCell::new(Plane::front()))).unwrap();
        wb.planes.insert(1, Rc::new(RefCell::new(Plane::front()))).unwrap();
        wb.planes.insert(2, Rc::new(RefCell::new(Plane::front()))).unwrap();

        wb
    }

    pub fn get_first_plane_id(&self) -> Option<IDType> {
        if self.planes.len() > 0 {
            Some(self.planes.keys().next().unwrap().to_owned())
        } else {
            None
        }
    }

    pub fn get_last_plane_id(&self) -> Option<IDType> {
        if self.planes.len() > 0 {
            Some(self.planes.keys().last().unwrap().to_owned())
        } else {
            None
        }
    }

    pub fn get_sketch_by_id(&self, id: IDType) -> Result<Rc<RefCell<ISketch>>, CADmiumError> {
        println!("Getting sketch by id: {:?} {:?}", id, self.sketches);
        self.sketches.get(&id).ok_or(CADmiumError::SketchIDNotFound(id)).cloned()
    }

    pub fn add_message_step(&mut self, message: &Message) {
        self.history.push(Step::new(self.history.len() as IDType, message.clone()));
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

#[derive(Tsify, NoRealize, Debug, Clone, Serialize, Deserialize)]
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


#[derive(Tsify, NoRealize, Debug, Clone, Serialize, Deserialize)]
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

#[derive(Tsify, NoRealize, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddSketch {
    plane_description: PlaneDescription,
}

impl MessageHandler for AddSketch {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, sketch_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let mut wb = sketch_ref.borrow_mut();

        println!("Adding sketch with plane description: {:?}", self.plane_description);
        let sketch = ISketch::try_from_plane_description(&wb, &self.plane_description)?;
        let new_id = wb.sketches_next_id;
        wb.sketches.insert(new_id, Rc::new(RefCell::new(sketch)));
        println!("Added sketch with id: {:?}", wb.sketches);
        wb.sketches_next_id += 1;
        Ok(Some(new_id))
    }
}

#[derive(Tsify, NoRealize, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WorkbenchRename {
    new_name: String,
}

impl MessageHandler for WorkbenchRename {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, workbench_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let mut workbench = workbench_ref.borrow_mut();
        workbench.name = self.new_name.clone();
        Ok(None)
    }
}
