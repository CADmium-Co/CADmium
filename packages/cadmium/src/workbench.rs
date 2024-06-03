use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription};
use crate::error::CADmiumError;
use crate::isketch::{IPlane, ISketch};
use crate::realization::Realization;
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

    pub fn realize(&mut self, max_steps: u64) -> Result<Realization, anyhow::Error> {
        let mut realized = Realization::new();
        realized.planes.insert(0, IPlane { plane: Plane::front(), width: 100.0, height: 100.0, name: "front".to_string() });
        realized.planes.insert(1, IPlane { plane: Plane::right(), width: 100.0, height: 100.0, name: "right".to_string() });
        realized.planes.insert(2, IPlane { plane: Plane::top(), width: 100.0, height: 100.0, name: "top".to_string() });
        realized.points.insert(0, Point3::new(0.0, 0.0, 0.0));
        let max_steps = max_steps as usize; // just coerce the type once

        for (step_n, step) in self.history.iter().enumerate() {
            // println!("{:?}", step.name);
            if step_n >= max_steps {
                break;
            }

            let step_data = &step.data;
            // println!("{:?}", step_data);
            match step_data {
                // StepData::WorkbenchPoint { point, .. } => {
                //     realized
                //         .points
                //         .insert(step.id, point.clone());
                // }
                // StepData::WorkbenchPlane {
                //     plane,
                //     width,
                //     height,
                //     ..
                // } => {
                //     // Do we need to store the IPlane or just the Plane?
                //     let rp = IPlane {
                //         plane: plane.clone(),
                //         width: *width,
                //         height: *height,
                //         name: step.name.clone(),
                //     };
                //     realized.planes.insert(step.id, rp);
                // }
                // StepData::WorkbenchSketch {
                //     plane_description,
                //     // sketch_id,
                //     // width: _,
                //     // height: _,
                //     ..
                // } => match plane_description {
                //     PlaneDescription::PlaneId(plane_id) => {
                //         let plane = self.planes.get(&plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?;
                //         let plane_ref = plane.clone();
                //         let sketch = ISketch::new(plane_ref);

                //         realized.sketches.insert(
                //             step.id,
                //             (
                //                 sketch.clone(),
                //                 sketch.clone(),
                //                 step.name.clone(),
                //             ),
                //         );
                //     }
                //     PlaneDescription::SolidFace { solid_id: _, normal: _ } => {
                //         // let solid = &realized.solids[&solid_id];
                //         // let face = solid.get_face_by_normal(&normal).unwrap();
                //         // let oriented_surface = face.oriented_surface();

                //         // println!("Surface: {:?}", oriented_surface);
                //         // let sketch_plane;
                //         // match oriented_surface {
                //         //     truck_modeling::geometry::Surface::Plane(p) => {
                //         //         let plane = Plane::from_truck(p);
                //         //         println!("Plane: {:?}", plane);
                //         //         sketch_plane = plane;
                //         //     }
                //         //     _ => {
                //         //         panic!("I only know how to put sketches on planes");
                //         //     }
                //         // }

                //         // let new_plane_id = format!("derived_plane_for:{}", step.name);

                //         // let rp = IPlane {
                //         //     plane: sketch_plane.clone(),
                //         //     width: 90.0,
                //         //     height: 60.0,
                //         //     name: new_plane_id.clone(),
                //         // };
                //         // realized.planes.insert(new_plane_id.clone(), rp);
                //         // let rp = &realized.planes[&new_plane_id];
                //         // let sketch_ref = Rc::new(RefCell::new(sketch.clone()));

                //         // // TODO: There's no way this is correct. Also a lot of prelude is the same fo Plane case
                //         // realized.sketches.insert(
                //         //     step.unique_id.to_owned(),
                //         //     (
                //         //         ISketch::new(&new_plane_id, &rp, sketch_ref.clone()),
                //         //         ISketch::new(
                //         //             &new_plane_id,
                //         //             &rp,
                //         //             // TODO: &sketch.split_intersections(false),
                //         //             sketch_ref,
                //         //         ),
                //         //         step.name.clone(),
                //         //     ),
                //         // );
                //     }
                // },
                // StepData::SolidExtrusion {
                //     face_ids,
                //     sketch_id,
                //     length,
                //     offset,
                //     mode,
                //     direction,
                //     ..
                // } => {
                //     // TODO: Make realization a trait and implement it for Extrusion
                //     let sketch_ref = self.sketches.get(sketch_id).unwrap();
                //     let sketch = sketch_ref.borrow();
                //     let faces = face_ids.iter().map(|id| sketch.faces().get(*id as usize).unwrap().clone()).collect();

                //     let new_extrusion = Extrusion::new(faces, sketch_ref.clone(), *length, *offset, direction.clone(), mode.clone());
                //     let feature = new_extrusion.to_feature();
                //     let solid_like = feature.as_solid_like();
                //     let new_solids = solid_like.to_solids()?;

                //     match &new_extrusion.mode {
                //         extrusion::Mode::New => {
                //             new_solids.iter().for_each(|s| {
                //                 realized.solids.insert(self.solids_next_id, s.clone());
                //                 self.solids_next_id += 1;
                //             });
                //         }
                //         extrusion::Mode::Add(merge_scope) => {
                //             for existing_solid_id in merge_scope {
                //                 let existing_solid = realized.solids.get(&existing_solid_id).unwrap().clone();
                //                 let mut existing_solid_to_merge_with =
                //                     realized.solids.remove(&existing_solid_id).unwrap();

                //                 // merge this existing solid with as many of the new solids as possible
                //                 for new_solid in new_solids.iter() {
                //                     let fused = fuse(
                //                         &existing_solid_to_merge_with.truck_solid,
                //                         &new_solid.truck_solid,
                //                     ).unwrap();

                //                     let new_merged_sold = Solid::from_truck_solid(existing_solid.name.clone(), fused);
                //                     existing_solid_to_merge_with = new_merged_sold;
                //                 }

                //                 realized.solids.insert(
                //                     existing_solid_id.to_owned(),
                //                     existing_solid_to_merge_with,
                //                 );
                //             }
                //         }
                //         extrusion::Mode::Remove(merge_scope) => {
                //             // If this extrusion is in mode "Remove" then we need to subtract the resulting solid
                //             // with each of the solids listed in the merge scope
                //             for existing_solid_id in merge_scope {
                //                 let existing_solid = realized.solids.get(&existing_solid_id).unwrap().clone();
                //                 let mut existing_solid_to_merge_with =
                //                     realized.solids.remove(&existing_solid_id).unwrap();

                //                 // merge this existing solid with as many of the new solids as possible
                //                 for new_solid in new_solids.iter() {
                //                     let punch = new_solid.truck_solid.clone();

                //                     let cleared = solid_and(
                //                         &existing_solid_to_merge_with.truck_solid,
                //                         &punch,
                //                         0.1,
                //                     ).unwrap();

                //                     let new_merged_sold = Solid::from_truck_solid(existing_solid.name.clone(), cleared);
                //                     existing_solid_to_merge_with = new_merged_sold;
                //                 }

                //                 realized.solids.insert(
                //                     existing_solid_id.to_owned(),
                //                     existing_solid_to_merge_with,
                //                 );
                //             }
                //         }
                //     }
                // }
                _ => {}
            }
        }

        Ok(realized)
    }
}

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
    plane_description: PlaneDescription,
}

impl MessageHandler for AddSketch {
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, sketch_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let mut wb = sketch_ref.borrow_mut();

        println!("Adding sketch with plane description: {:?}", self.plane_description);
        let plane = match self.plane_description {
            PlaneDescription::PlaneId(plane_id) =>
                wb.planes.get(&plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?,
            PlaneDescription::SolidFace { solid_id: _, normal: _ } => todo!("Implement SolidFace"),
        }.clone();

        let sketch = ISketch::new(plane);
        let new_id = wb.sketches_next_id;
        wb.sketches.insert(new_id, Rc::new(RefCell::new(sketch)));
        println!("Added sketch with id: {:?}", wb.sketches);
        wb.sketches_next_id += 1;
        Ok(Some(new_id))
    }
}

impl Identifiable for Rc<RefCell<Workbench>> {
    type Parent = crate::project::Project;
    const ID_NAME: &'static str = "workbench_id";

    fn from_parent_id(parent: &crate::project::Project, id: IDType) -> anyhow::Result<Self> {
        Ok(parent.get_workbench_by_id(id)?)
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
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
