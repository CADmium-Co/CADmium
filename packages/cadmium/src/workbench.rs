use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription};
use crate::error::CADmiumError;
use crate::solid::extrusion::{self, fuse, Extrusion};
use crate::isketch::{IPlane, ISketch};
use crate::realization::Realization;
use crate::solid::point::Point3;
use crate::solid::Solid;
use crate::solid::SolidLike;
use crate::step::{Step, StepData};
use crate::IDType;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

// use truck_base::math::Vector3 as truck_vector3;
use truck_shapeops::and as solid_and;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Workbench {
    pub(crate) name: String,
    pub(crate) history: Vec<Step>,

    // These are free-standing points in 3D space, not part of sketches
    pub(crate) points: BTreeMap<IDType, Point3>,
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
            points_next_id: 0,
            planes: BTreeMap::new(),
            planes_next_id: 0,

            sketches: BTreeMap::new(),
            sketches_next_id: 0,
            solids: BTreeMap::new(),
            solids_next_id: 0,
        };

        wb.add_workbench_point(Point3::new(0.0, 0.0, 0.0)).unwrap();
        wb.add_workbench_plane(Plane::front(), 100.0, 100.0).unwrap();
        wb.add_workbench_plane(Plane::right(), 100.0, 100.0).unwrap();
        wb.add_workbench_plane(Plane::top(), 100.0, 100.0).unwrap();

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

    pub fn update_step_data(&mut self, step_id: &str, new_step_data: StepData) {
        let mut index = 0;
        for step in self.history.iter() {
            if step.unique_id() == step_id {
                break;
            }
            index += 1;
        }

        self.history[index].data = new_step_data;
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
                StepData::WorkbenchPoint { point, .. } => {
                    realized
                        .points
                        .insert(step.id, point.clone());
                }
                StepData::WorkbenchPlane {
                    plane,
                    width,
                    height,
                    ..
                } => {
                    // Do we need to store the IPlane or just the Plane?
                    let rp = IPlane {
                        plane: plane.clone(),
                        width: *width,
                        height: *height,
                        name: step.name.clone(),
                    };
                    realized.planes.insert(step.id, rp);
                }
                StepData::WorkbenchSketch {
                    plane_description,
                    // sketch_id,
                    // width: _,
                    // height: _,
                    ..
                } => match plane_description {
                    PlaneDescription::PlaneId(plane_id) => {
                        let plane = self.planes.get(&plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?;
                        let plane_ref = plane.clone();
                        let sketch = ISketch::new(plane_ref);

                        realized.sketches.insert(
                            step.id,
                            (
                                sketch.clone(),
                                sketch.clone(),
                                step.name.clone(),
                            ),
                        );
                    }
                    PlaneDescription::SolidFace { solid_id: _, normal: _ } => {
                        // let solid = &realized.solids[&solid_id];
                        // let face = solid.get_face_by_normal(&normal).unwrap();
                        // let oriented_surface = face.oriented_surface();

                        // println!("Surface: {:?}", oriented_surface);
                        // let sketch_plane;
                        // match oriented_surface {
                        //     truck_modeling::geometry::Surface::Plane(p) => {
                        //         let plane = Plane::from_truck(p);
                        //         println!("Plane: {:?}", plane);
                        //         sketch_plane = plane;
                        //     }
                        //     _ => {
                        //         panic!("I only know how to put sketches on planes");
                        //     }
                        // }

                        // let new_plane_id = format!("derived_plane_for:{}", step.name);

                        // let rp = IPlane {
                        //     plane: sketch_plane.clone(),
                        //     width: 90.0,
                        //     height: 60.0,
                        //     name: new_plane_id.clone(),
                        // };
                        // realized.planes.insert(new_plane_id.clone(), rp);
                        // let rp = &realized.planes[&new_plane_id];
                        // let sketch_ref = Rc::new(RefCell::new(sketch.clone()));

                        // // TODO: There's no way this is correct. Also a lot of prelude is the same fo Plane case
                        // realized.sketches.insert(
                        //     step.unique_id.to_owned(),
                        //     (
                        //         ISketch::new(&new_plane_id, &rp, sketch_ref.clone()),
                        //         ISketch::new(
                        //             &new_plane_id,
                        //             &rp,
                        //             // TODO: &sketch.split_intersections(false),
                        //             sketch_ref,
                        //         ),
                        //         step.name.clone(),
                        //     ),
                        // );
                    }
                },
                StepData::SolidExtrusion {
                    face_ids,
                    sketch_id,
                    length,
                    offset,
                    mode,
                    direction,
                    ..
                } => {
                    // TODO: Make realization a trait and implement it for Extrusion
                    let sketch_ref = self.sketches.get(sketch_id).unwrap();
                    let sketch = sketch_ref.borrow();
                    let faces = face_ids.iter().map(|id| sketch.faces().get(*id as usize).unwrap().clone()).collect();

                    let new_extrusion = Extrusion::new(faces, sketch_ref.clone(), *length, *offset, direction.clone(), mode.clone());
                    let feature = new_extrusion.to_feature();
                    let solid_like = feature.as_solid_like();
                    let new_solids = solid_like.to_solids()?;

                    match &new_extrusion.mode {
                        extrusion::Mode::New => {
                            new_solids.iter().for_each(|s| {
                                realized.solids.insert(self.solids_next_id, s.clone());
                                self.solids_next_id += 1;
                            });
                        }
                        extrusion::Mode::Add(merge_scope) => {
                            for existing_solid_id in merge_scope {
                                let existing_solid = realized.solids.get(&existing_solid_id).unwrap().clone();
                                let mut existing_solid_to_merge_with =
                                    realized.solids.remove(&existing_solid_id).unwrap();

                                // merge this existing solid with as many of the new solids as possible
                                for new_solid in new_solids.iter() {
                                    let fused = fuse(
                                        &existing_solid_to_merge_with.truck_solid,
                                        &new_solid.truck_solid,
                                    ).unwrap();

                                    let new_merged_sold = Solid::from_truck_solid(existing_solid.name.clone(), fused);
                                    existing_solid_to_merge_with = new_merged_sold;
                                }

                                realized.solids.insert(
                                    existing_solid_id.to_owned(),
                                    existing_solid_to_merge_with,
                                );
                            }
                        }
                        extrusion::Mode::Remove(merge_scope) => {
                            // If this extrusion is in mode "Remove" then we need to subtract the resulting solid
                            // with each of the solids listed in the merge scope
                            for existing_solid_id in merge_scope {
                                let existing_solid = realized.solids.get(&existing_solid_id).unwrap().clone();
                                let mut existing_solid_to_merge_with =
                                    realized.solids.remove(&existing_solid_id).unwrap();

                                // merge this existing solid with as many of the new solids as possible
                                for new_solid in new_solids.iter() {
                                    let punch = new_solid.truck_solid.clone();

                                    let cleared = solid_and(
                                        &existing_solid_to_merge_with.truck_solid,
                                        &punch,
                                        0.1,
                                    ).unwrap();

                                    let new_merged_sold = Solid::from_truck_solid(existing_solid.name.clone(), cleared);
                                    existing_solid_to_merge_with = new_merged_sold;
                                }

                                realized.solids.insert(
                                    existing_solid_id.to_owned(),
                                    existing_solid_to_merge_with,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(realized)
    }
}

// Step operations
impl Workbench {
    pub(super) fn do_workbench_rename(&mut self, new_name: String) -> Result<IDType, anyhow::Error> {
        self.name = new_name;
        // TODO: What ID should be returned here?
        Ok(0)
    }

    pub(super) fn add_workbench_point(&mut self, point: Point3) -> Result<IDType, anyhow::Error> {
        self.points.insert(self.points_next_id, point);
        self.points_next_id += 1;
        Ok(self.points_next_id - 1)
    }

    pub(super) fn add_workbench_plane(&mut self, plane: Plane, _width: f64, _height: f64) -> Result<IDType, anyhow::Error> {
        let plane_cell = Rc::new(RefCell::new(plane));
        self.planes.insert(self.planes_next_id, plane_cell);
        self.planes_next_id += 1;
        Ok(self.planes_next_id - 1)
    }

    pub(super) fn add_workbench_sketch(
        &mut self,
        plane_description: PlaneDescription,
    ) -> Result<IDType, anyhow::Error> {
        println!("Adding sketch with plane description: {:?}", plane_description);
        let plane = match plane_description {
            PlaneDescription::PlaneId(plane_id) =>
                self.planes.get(&plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?,
            PlaneDescription::SolidFace { solid_id: _, normal: _ } => todo!("Implement SolidFace"),
        }.clone();

        let sketch = ISketch::new(plane);
        self.sketches.insert(self.sketches_next_id, Rc::new(RefCell::new(sketch)));
        println!("Added sketch with id: {:?}", self.sketches);
        self.sketches_next_id += 1;
        Ok(self.sketches_next_id - 1)
    }

    pub(crate) fn add_solid_extrusion(
        &mut self,
        _face_ids: Vec<IDType>,
        _sketch_id: IDType,
        _length: f64,
        _offset: f64,
        _mode: extrusion::Mode,
        _direction: extrusion::Direction,
    ) -> Result<IDType, anyhow::Error> {
        // I guess nothing to do? only realization?
        // TODO: What ID should be returned here?
        Ok(0)
    }

    pub(super) fn do_workbench_step_rename(&mut self, step_id: IDType, new_name: String) -> Result<IDType, anyhow::Error> {
        let step = self.history.iter_mut().find(|s| s.id == step_id).ok_or(anyhow::anyhow!("Failed to find step with id {}", step_id))?;
        step.name = new_name;
        Ok(step.id)
    }

    pub(super) fn do_workbench_step_delete(&mut self, step_id: IDType) -> Result<IDType, anyhow::Error> {
        let old_len = self.history.len();
        self.history.retain(|s| s.id != step_id);

        if self.history.len() == old_len {
            return Err(anyhow::anyhow!("Failed to find step with id {}", step_id));
        }

        Ok(step_id)
    }
}
