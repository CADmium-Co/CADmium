use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::extrusion::{fuse, Extrusion, ExtrusionMode, Solid};
use crate::project::{Plane, PlaneDescription, Point3, RealPlane, RealSketch, Vector3};
use crate::realization::Realization;
use crate::sketch::Sketch;
use crate::step::{Step, StepData};

use std::collections::HashMap;

// use truck_base::math::Vector3 as truck_vector3;
use truck_shapeops::and as solid_and;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Workbench {
    pub(crate) name: String,
    pub(crate) history: Vec<Step>,
    pub(crate) step_counters: HashMap<String, u64>,
}

impl Workbench {
    pub fn new(name: &str) -> Self {
        Workbench {
            name: name.to_owned(),
            history: vec![],
            step_counters: HashMap::from([
                ("Point".to_owned(), 0),
                ("Plane".to_owned(), 0),
                ("Sketch".to_owned(), 0),
                ("Extrusion".to_owned(), 0),
            ]),
        }
    }

    pub fn get_first_plane_id(&self) -> Option<String> {
        for step in self.history.iter() {
            match &step.data {
                StepData::Plane {
                    plane: _,
                    width: _,
                    height: _,
                } => {
                    return Some(step.unique_id.clone());
                }
                _ => {}
            }
        }
        None
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

    pub fn last_plane_id(&self) -> Option<String> {
        let mut last_plane_id = None;
        for step in self.history.iter() {
            match &step.data {
                StepData::Plane {
                    plane: _,
                    width: _,
                    height: _,
                } => {
                    last_plane_id = Some(step.unique_id.clone());
                }
                _ => {}
            }
        }
        last_plane_id
    }

    pub fn json(&self) -> String {
        let result = serde_json::to_string(self);
        match result {
            Ok(json) => json,
            Err(e) => format!("Error: {}", e),
        }
    }

    pub fn get_sketch_mut(&mut self, name: &str) -> Option<&mut Sketch> {
        for step in self.history.iter_mut() {
            match &mut step.data {
                StepData::Sketch {
                    plane_description: _,
                    width: _,
                    height: _,
                    sketch,
                } => {
                    if name == step.name {
                        return Some(sketch);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn get_sketch_by_id_mut(&mut self, id: &str) -> Option<&mut Sketch> {
        for step in self.history.iter_mut() {
            match &mut step.data {
                StepData::Sketch {
                    plane_description: _,
                    width: _,
                    height: _,
                    sketch,
                } => {
                    if id == step.unique_id {
                        return Some(sketch);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub fn add_defaults_2(&mut self) {
        self.add_point("Origin", Point3::new(0.0, 0.0, 0.0));
        self.add_plane("Top", Plane::top());
        self.add_plane("Front", Plane::front());
        self.add_plane("Right", Plane::right());

        // let sketch_id = self.add_sketch_to_plane("Sketch 1", "Top");

        // let sketch = self.get_sketch_mut("Sketch 1").unwrap();

        // square in center
        // let width = 0.5;
        // let height = 0.5;
        // let p0 = sketch.add_fixed_point(-width / 2.0, -height / 2.0);
        // let p1 = sketch.add_point(-width / 2.0, height / 2.0);
        // let p2 = sketch.add_point(width / 2.0, height / 2.0);
        // let p3 = sketch.add_point(width / 2.0, -height / 2.0);
        // let seg_0 = sketch.add_segment(p0, p1);
        // let seg_1 = sketch.add_segment(p1, p2);
        // let seg_2 = sketch.add_segment(p2, p3);
        // let seg_3 = sketch.add_segment(p3, p0);

        // let big_p0 = sketch.add_point(-0.1, -0.1);
        // let big_p1 = sketch.add_point(0.55, -0.1);
        // let big_p2 = sketch.add_point(0.55, 0.55);
        // let big_p3 = sketch.add_point(-0.1, 0.55);
        // let big_seg_0 = sketch.add_segment(big_p0, big_p1);
        // let big_seg_1 = sketch.add_segment(big_p1, big_p2);
        // let big_seg_2 = sketch.add_segment(big_p2, big_p3);
        // let big_seg_3 = sketch.add_segment(big_p3, big_p0);

        // self.add_extrusion(
        //     "Ext 1",
        //     Extrusion {
        //         sketch_id,
        //         face_ids: vec![0, 1],
        //         length: 0.15,
        //         offset: 0.0,
        //         direction: Direction::Normal,
        //     },
        // );
    }

    pub fn add_defaults(&mut self) {
        self.add_point("Origin", Point3::new(0.0, 0.0, 0.0));
        self.add_plane("Front", Plane::front());
        self.add_plane("Right", Plane::right());
        // let top_plane_id = self.add_plane("Top", Plane::top());

        // let sketch_id = self.add_sketch_to_plane("Sketch 1", &top_plane_id);

        // let sketch = self.get_sketch_mut("Sketch 1").unwrap();

        // square in upper right
        // let p0 = sketch.add_fixed_point(0.0, 0.0);
        // let p1 = sketch.add_point(45.0, 0.0);
        // let p2 = sketch.add_point(45.0, 45.0);
        // let p3 = sketch.add_point(0.0, 45.0);
        // let seg_0 = sketch.add_segment(p0, p1);
        // let seg_1 = sketch.add_segment(p1, p2);
        // let seg_2 = sketch.add_segment(p2, p3);
        // let seg_3 = sketch.add_segment(p3, p0);

        // let big_p0 = sketch.add_point(-0.1, -0.1);
        // let big_p1 = sketch.add_point(0.55, -0.1);
        // let big_p2 = sketch.add_point(0.55, 0.55);
        // let big_p3 = sketch.add_point(-0.1, 0.55);
        // let big_seg_0 = sketch.add_segment(big_p0, big_p1);
        // let big_seg_1 = sketch.add_segment(big_p1, big_p2);
        // let big_seg_2 = sketch.add_segment(big_p2, big_p3);
        // let big_seg_3 = sketch.add_segment(big_p3, big_p0);

        // sketch.add_segment_vertical_constraint(seg_3);
        // sketch.add_segment_horizontal_constraint(seg_0);
        // sketch.add_segment_length_constraint(seg_0, 0.52);
        // sketch.add_segment_length_constraint(seg_1, 0.52);
        // sketch.add_segment_length_constraint(seg_2, 0.52);
        // sketch.add_segment_length_constraint(seg_3, 0.52);

        // Simple circle in lower left
        // let p4 = sketch.add_point(-0.5, -0.25);
        // sketch.add_circle(p4, 0.3);

        // // intersecting circle!
        // let p5 = sketch.add_point(-0.8, -0.25);
        // let c2 = sketch.add_circle(p5, 0.3);

        // sketch.add_circle_diameter_constraint(c2, 0.6);

        // Rounded square in lower right
        // let shrink = 0.4;
        // let offset_x = 0.1;
        // let offset_y = -0.70;
        // let a = sketch.add_point(0.25 * shrink + offset_x, 0.00 * shrink + offset_y);
        // let b = sketch.add_point(0.75 * shrink + offset_x, 0.00 * shrink + offset_y);
        // let c = sketch.add_point(1.00 * shrink + offset_x, 0.25 * shrink + offset_y);
        // let d = sketch.add_point(1.00 * shrink + offset_x, 0.75 * shrink + offset_y);
        // let e = sketch.add_point(0.75 * shrink + offset_x, 1.00 * shrink + offset_y);
        // let f = sketch.add_point(0.25 * shrink + offset_x, 1.00 * shrink + offset_y);
        // let g = sketch.add_point(0.00 * shrink + offset_x, 0.75 * shrink + offset_y);
        // let h = sketch.add_point(0.00 * shrink + offset_x, 0.25 * shrink + offset_y);
        // let i = sketch.add_point(0.75 * shrink + offset_x, 0.25 * shrink + offset_y);
        // let j = sketch.add_point(0.75 * shrink + offset_x, 0.75 * shrink + offset_y);
        // let k = sketch.add_point(0.25 * shrink + offset_x, 0.75 * shrink + offset_y);
        // let l = sketch.add_point(0.25 * shrink + offset_x, 0.25 * shrink + offset_y);

        // sketch.add_segment(a, b);
        // sketch.add_arc(i, b, c, false);
        // sketch.add_segment(c, d);
        // sketch.add_arc(j, d, e, false);
        // sketch.add_segment(e, f);
        // sketch.add_arc(k, f, g, false);
        // sketch.add_segment(g, h);
        // sketch.add_arc(l, h, a, false);

        // self.add_extrusion(
        //     "Ext 1",
        //     Extrusion {
        //         sketch_id,
        //         face_ids: vec![],
        //         length: 0.25,
        //         offset: 0.0,
        //         direction: Direction::Normal,
        //     },
        // );

        // self.add_extrusion(
        //     "Ext 2",
        //     Extrusion {
        //         sketch_name: "Sketch 1".to_owned(),
        //         face_ids: vec![0, 1],
        //         length: 0.15,
        //         offset: 0.0,
        //         direction: Vector3::new(0.0, 0.0, 1.0),
        //     },
        // );
    }

    pub fn add_point(&mut self, name: &str, point: Point3) {
        let counter = self.step_counters.get_mut("Point").unwrap();
        self.history.push(Step::new_point(name, point, *counter));
        *counter += 1;
    }

    pub fn add_plane(&mut self, name: &str, plane: Plane) -> String {
        let counter = self.step_counters.get_mut("Plane").unwrap();
        self.history.push(Step::new_plane(name, plane, *counter));
        *counter += 1;

        self.plane_name_to_id(name).unwrap()
    }

    pub fn plane_name_to_id(&self, plane_name: &str) -> Option<String> {
        for step in self.history.iter() {
            if step.name == plane_name {
                match &step.data {
                    StepData::Plane {
                        plane: _,
                        width: _,
                        height: _,
                    } => {
                        return Some(step.unique_id.clone());
                    }
                    _ => {}
                }
            }
        }
        None
    }

    pub fn add_sketch_to_solid_face(
        &mut self,
        new_sketch_name: &str,
        solid_id: &str,
        normal: Vector3,
    ) -> String {
        // TODO: maybe this shouldn't just take in a normal. Maybe it should take in the o, p, q points as well
        // that way it could try to match even if there are multiple faces on this solid which have the same normal vector
        // println!("New Normal! {:?}", normal);
        // called like: wb.add_sketch_to_solid_face("Sketch-2", "Ext1:0", Vector3::new(0.0, 0.0, 1.0));

        let counter = self.step_counters.get_mut("Sketch").unwrap();
        let new_step = Step::new_sketch_on_solid_face(&new_sketch_name, solid_id, normal, *counter);
        let new_step_id = new_step.unique_id.clone();
        self.history.push(new_step);
        *counter += 1;

        new_step_id
    }

    pub fn add_sketch_to_plane(&mut self, name: &str, plane_id: &str) -> String {
        if plane_id != "" {
            // if the plane id is specified, check to make sure a plane with that ID exists
            let mut plane_exists = false;
            for step in self.history.iter() {
                if step.unique_id == plane_id {
                    match &step.data {
                        StepData::Plane {
                            plane: _,
                            width: _,
                            height: _,
                        } => {
                            plane_exists = true;
                        }
                        _ => {}
                    }
                }
            }

            if !plane_exists {
                return format!("failed to find plane with id {}", plane_id);
            }
        }
        // if the plane id is empty string, that's okay it's a placeholder

        // If the sketch name is empty string, then we need to generate a new name
        // Let's use "Sketch n" where n is the number of sketches
        let counter = self.step_counters.get_mut("Sketch").unwrap();
        let sketch_name = if name == "" {
            format!("Sketch {}", *counter + 1)
        } else {
            name.to_owned()
        };

        let new_step = Step::new_sketch(&sketch_name, &plane_id, *counter);
        let new_step_id = new_step.unique_id.clone();
        self.history.push(new_step);
        *counter += 1;

        new_step_id
    }

    pub fn add_extrusion(&mut self, name: &str, extrusion: Extrusion) -> u64 {
        // If the extrusion name is empty string, then we need to generate a new name
        // Let's use "Extrusion n" where n is the number of extrusions
        let counter = self.step_counters.get_mut("Extrusion").unwrap();
        let extrusion_name = if name == "" {
            format!("Extrusion {}", *counter + 1)
        } else {
            name.to_owned()
        };
        self.history
            .push(Step::new_extrusion(&extrusion_name, extrusion, *counter));
        *counter += 1;
        *counter - 1
    }

    pub fn realize(&self, max_steps: u64) -> Realization {
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
                StepData::Point { point } => {
                    realized
                        .points
                        .insert(step.unique_id.to_owned(), point.clone());
                }
                StepData::Plane {
                    plane,
                    width,
                    height,
                } => {
                    let rp = RealPlane {
                        plane: plane.clone(),
                        width: *width,
                        height: *height,
                        name: step.name.clone(),
                    };
                    realized.planes.insert(step.unique_id.to_owned(), rp);
                }
                StepData::Sketch {
                    width: _,
                    height: _,
                    plane_description,
                    sketch,
                } => match plane_description {
                    PlaneDescription::PlaneId(plane_id) => {
                        if plane_id == "" {
                            println!("Sketch {} has no plane", step.name);
                            continue;
                        }

                        let plane = &realized.planes[plane_id];

                        realized.sketches.insert(
                            step.unique_id.to_owned(),
                            (
                                RealSketch::new(&plane.name, plane_id, plane, sketch),
                                RealSketch::new(
                                    &plane.name,
                                    plane_id,
                                    plane,
                                    &sketch.split_intersections(false),
                                ),
                                step.name.clone(),
                            ),
                        );
                    }
                    PlaneDescription::SolidFace { solid_id, normal } => {
                        let solid = &realized.solids[solid_id];
                        let face = solid.get_face_by_normal(normal).unwrap();
                        let oriented_surface = face.oriented_surface();

                        println!("Surface: {:?}", oriented_surface);
                        let sketch_plane;
                        match oriented_surface {
                            truck_modeling::geometry::Surface::Plane(p) => {
                                let plane = Plane::from_truck(p);
                                println!("Plane: {:?}", plane);
                                sketch_plane = plane;
                            }
                            _ => {
                                panic!("I only know how to put sketches on planes");
                            }
                        }

                        let new_plane_id = format!("derived_plane_for:{}", step.name);

                        let rp = RealPlane {
                            plane: sketch_plane.clone(),
                            width: 90.0,
                            height: 60.0,
                            name: new_plane_id.clone(),
                        };
                        realized.planes.insert(new_plane_id.clone(), rp);
                        let rp = &realized.planes[&new_plane_id];

                        realized.sketches.insert(
                            step.unique_id.to_owned(),
                            (
                                RealSketch::new(&new_plane_id, &new_plane_id, &rp, sketch),
                                RealSketch::new(
                                    &new_plane_id,
                                    &new_plane_id,
                                    &rp,
                                    &sketch.split_intersections(false),
                                ),
                                step.name.clone(),
                            ),
                        );
                    }
                },
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
                                    realized.solids.remove(existing_solid_name).unwrap();

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
                                    realized.solids.remove(existing_solid_name).unwrap();

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
            }
        }

        realized
    }
}
