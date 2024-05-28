use isotope::sketch::Sketch;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription, Point3, Vector3};
use crate::error::CADmiumError;
use crate::extrusion::{fuse, Extrusion, ExtrusionMode};
use crate::isketch::{IPlane, ISketch};
use crate::realization::Realization;
use crate::solid::Solid;
use crate::step::{Step, StepData};
use crate::IDType;

use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

// use truck_base::math::Vector3 as truck_vector3;
use truck_shapeops::and as solid_and;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Workbench {
    pub(crate) name: String,
    pub(crate) history: Vec<Step>,

    // These are free-standing points in 3D space, not part of sketches
    pub(crate) points: BTreeMap<IDType, Point3>,
    pub(crate) points_next_id: IDType,

    pub(crate) planes: BTreeMap<IDType, Plane>,
    pub(crate) planes_next_id: IDType,

    pub(crate) sketches: BTreeMap<IDType, Rc<RefCell<ISketch>>>,
    pub(crate) sketches_next_id: IDType,
    pub(crate) solids: BTreeMap<IDType, Rc<RefCell<Solid>>>,
    pub(crate) solids_next_id: IDType,
}

impl Workbench {
    pub fn new(name: &str) -> Self {
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

    pub fn get_sketch_by_id(&mut self, id: IDType) -> Result<&mut Rc<RefCell<ISketch>>, CADmiumError> {
        self.sketches.get_mut(&id).ok_or(CADmiumError::SketchIDNotFound(id))
    }

    pub fn update_step_data(&mut self, step_id: &str, new_step_data: StepData) {
        let mut index = 0;
        for step in self.history.iter() {
            if step.unique_id == step_id {
                break;
            }
            index += 1;
        }

        self.history[index].data = new_step_data;
    }

    // pub fn add_extrusion(&mut self, name: &str, extrusion: Extrusion) -> u64 {
    //     // If the extrusion name is empty string, then we need to generate a new name
    //     // Let's use "Extrusion n" where n is the number of extrusions
    //     let extrusion_name = if name == "" {
    //         format!("Extrusion {}", *counter + 1)
    //     } else {
    //         name.to_owned()
    //     };
    //     self.history
    //         .push(Step::new_extrusion(&extrusion_name, extrusion, *counter));
    // }

    pub fn realize(&self, max_steps: u64) -> Result<Realization, anyhow::Error> {
        let mut realized = Realization::new();
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
                        let plane = &realized.planes.get(&plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?;
                        let sketch = self.get_sketch_by_id(step.id)?.borrow().clone();

                        realized.sketches.insert(
                            step.id,
                            (
                                sketch.clone(),
                                sketch.clone(),
                                step.name.clone(),
                            ),
                        );
                    }
                    PlaneDescription::SolidFace { solid_id, normal } => {
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
                /*
                StepData::Extrusion { extrusion } => {
                    let (_sketch, split_sketch, _name) = &realized.sketches[&extrusion.sketch_id];
                    let plane = &realized.planes[&split_sketch.plane_id];

                    match &extrusion.mode {
                        ExtrusionMode::New => {
                            // if this extrusion is in mode "New" then this old behavior is correct!

                            let solids = Solid::from_extrusion(
                                step.name.clone(),
                                plane,
                                split_sketch,
                                extrusion,
                            );

                            for (name, solid) in solids {
                                realized.solids.insert(name, solid);
                            }
                        }
                        ExtrusionMode::Add(merge_scope) => {
                            // if this extrusion is in mode "Add" Then we need to merge the resulting solids
                            // with each of the solids listed in the merge scope

                            let new_solids = Solid::from_extrusion(
                                step.name.clone(),
                                plane,
                                split_sketch,
                                extrusion,
                            );

                            // NO LONGER NEEDED
                            // // this is some bullshit, but bear with me. To make the solids merge properly we need to
                            // // lengthen the extrusion a tiny bit, basically build in some buffer
                            // let mut longer_extrusion = extrusion.clone();
                            // longer_extrusion.length += 0.001;
                            // longer_extrusion.offset -= 0.001;
                            // let solids = Solid::from_extrusion(
                            //     step.name.clone(),
                            //     plane,
                            //     split_sketch,
                            //     &longer_extrusion,
                            // );

                            for existing_solid_name in merge_scope {
                                let mut existing_solid_to_merge_with =
                                    realized.solids.remove(&existing_solid_name).unwrap();

                                // merge this existing solid with as many of the new solids as possible
                                for (_, new_solid) in new_solids.iter() {
                                    // let new_candidate = translated(
                                    //     &solid.truck_solid,
                                    //     TruckVector3::new(0.0, 0.0, 1.0),
                                    // );
                                    // println!("\nTranslated new candidate: {:?}", new_candidate);

                                    // let result =
                                    //     solid_or(&existing_solid.truck_solid, &new_candidate, 0.1);

                                    let fused = fuse(
                                        &existing_solid_to_merge_with.truck_solid,
                                        &new_solid.truck_solid,
                                    );

                                    match fused {
                                        Some(s) => {
                                            existing_solid_to_merge_with = Solid::from_truck_solid(
                                                existing_solid_name.to_owned(),
                                                s,
                                            );
                                        }
                                        None => {
                                            println!("Failed to merge with OR");
                                        }
                                    }
                                }

                                realized.solids.insert(
                                    existing_solid_name.to_owned(),
                                    existing_solid_to_merge_with,
                                );
                            }
                        }

                        ExtrusionMode::Remove(merge_scope) => {
                            // If this extrusion is in mode "Remove" then we need to subtract the resulting solid
                            // with each of the solids listed in the merge scope
                            println!("Okay, let's remove");
                            let new_solids = Solid::from_extrusion(
                                step.name.clone(),
                                plane,
                                split_sketch,
                                extrusion,
                            );

                            for existing_solid_name in merge_scope {
                                let mut existing_solid_to_merge_with =
                                    realized.solids.remove(&existing_solid_name).unwrap();

                                // merge this existing solid with as many of the new solids as possible
                                for (_, new_solid) in new_solids.iter() {
                                    // let translated_solid = translated(
                                    //     &solid.truck_solid,
                                    //     TruckVector3::new(0.0, 0.0, 1.0),
                                    // );
                                    // println!("\nTranslated new candidate: {:?}", new_candidate);

                                    // let result =
                                    //     solid_or(&existing_solid.truck_solid, &new_candidate, 0.1);

                                    let punch = new_solid.truck_solid.clone();
                                    // punch.not();
                                    println!("Have a punch");

                                    let cleared = solid_and(
                                        &existing_solid_to_merge_with.truck_solid,
                                        &punch,
                                        0.1,
                                    );

                                    println!("have cleared");

                                    match cleared {
                                        Some(s) => {
                                            println!("Merged with AND");
                                            // println!("{:?}", s);
                                            existing_solid_to_merge_with = Solid::from_truck_solid(
                                                existing_solid_name.to_owned(),
                                                s,
                                            );
                                        }
                                        None => {
                                            println!("Failed to merge with AND");
                                        }
                                    }
                                }

                                realized.solids.insert(
                                    existing_solid_name.to_owned(),
                                    existing_solid_to_merge_with,
                                );
                                println!("inserted the solid back in")
                            }
                        }
                    }
                }
            */
            }
        }

        Ok(realized)
    }
}

// Step operations
impl Workbench {
    pub(super) fn add_workbench_point(&mut self, point: Point3) -> Result<IDType, anyhow::Error> {
        self.points.insert(self.points_next_id, point).ok_or(anyhow::anyhow!("Failed to insert point"));
        self.points_next_id += 1;
        Ok(self.points_next_id - 1)
    }

    pub(super) fn add_workbench_plane(&mut self, plane: Plane, width: f64, height: f64) -> Result<IDType, anyhow::Error> {
        self.planes.insert(self.planes_next_id, plane).ok_or(anyhow::anyhow!("Failed to insert plane"));
        self.planes_next_id += 1;
        Ok(self.planes_next_id - 1)
    }

    pub(super) fn add_workbench_sketch(
        &mut self,
        plane_description: PlaneDescription,
    ) -> Result<IDType, anyhow::Error> {
        let plane = match plane_description {
            PlaneDescription::PlaneId(plane_id) =>
                self.planes.get(&plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?,
            PlaneDescription::SolidFace { solid_id, normal } => todo!("Implement SolidFace"),
        };

        let plane_cell = Rc::new(RefCell::new(plane.clone()));

        let sketch = ISketch::new(plane_cell);
        self.sketches
            .insert(self.sketches_next_id, Rc::new(RefCell::new(sketch)))
            .ok_or(anyhow::anyhow!("Failed to insert sketch"));
        self.sketches_next_id += 1;
        Ok(self.sketches_next_id - 1)
    }
}
