use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::*;
use crate::error::CADmiumError;
use crate::realization::Realization;
use crate::sketch::constraints::Constraint;
use crate::sketch::{Face, Point2, Sketch};
use crate::step::StepData;
use crate::workbench::Workbench;
use std::collections::HashMap;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Project {
    pub name: String,
    pub assemblies: Vec<Assembly>,
    pub workbenches: Vec<Workbench>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        let mut p = Project {
            name: name.to_owned(),
            assemblies: vec![],
            workbenches: vec![],
        };

        let wb = Workbench::new("Workbench 1");
        p.workbenches.push(wb);

        p
    }

    pub fn json(&self) -> String {
        let result = serde_json::to_string(self);
        match result {
            Ok(json) => json,
            Err(e) => format!("Error: {}", e),
        }
    }

    pub fn from_json(json: &str) -> Self {
        let result = serde_json::from_str(json);
        match result {
            Ok(p) => p,
            Err(e) => {
                println!("Error: {}", e);
                Project::new("Error")
            }
        }
    }

    pub fn compute_constraint_errors(&mut self) {
        for workbench in self.workbenches.iter_mut() {
            for step in workbench.history.iter_mut() {
                match &mut step.data {
                    StepData::Sketch { sketch, .. } => {
                        sketch.compute_constraint_errors();
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn get_workbench_mut(&mut self, name: &str) -> Result<&mut Workbench, CADmiumError> {
        self.workbenches
            .iter_mut()
            .find(|wb| wb.name == name)
            .ok_or(CADmiumError::WorkbenchNameNotFound(name.to_string()))
    }

    pub fn get_workbench_by_id_mut(&mut self, id: u64) -> Result<&mut Workbench, CADmiumError> {
        self.workbenches
            .get_mut(id as usize)
            .ok_or(CADmiumError::WorkbenchIDNotFound(id))
    }

    pub fn get_realization(&self, workbench_id: u64, max_steps: u64) -> Realization {
        let workbench = &self.workbenches[workbench_id as usize];
        let realization = workbench.realize(max_steps);
        realization
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Assembly {
    name: String,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RealSketch {
    pub plane_id: String,
    pub plane_name: String,
    pub points: HashMap<u64, Point3>,
    pub points_2d: HashMap<u64, Point2>,
    pub highest_point_id: u64,
    pub line_segments: HashMap<u64, Line3>,
    pub highest_line_segment_id: u64,
    pub circles: HashMap<u64, Circle3>,
    pub highest_circle_id: u64,
    pub arcs: HashMap<u64, Arc3>,
    pub highest_arc_id: u64,
    pub constraints: HashMap<u64, Constraint>,
    pub highest_constraint_id: u64,
    pub faces: Vec<Face>,
}

impl RealSketch {
    pub fn new(plane_name: &str, plane_id: &str, plane: &RealPlane, sketch: &Sketch) -> Self {
        // The key difference between Sketch and RealSketch is that Sketch lives
        // in 2D and RealSketch lives in 3D. So we need to convert the points

        let mut real_sketch = RealSketch {
            plane_name: plane_name.to_owned(),
            plane_id: plane_id.to_owned(),
            points_2d: HashMap::new(),
            points: HashMap::new(),
            highest_point_id: 0,
            line_segments: HashMap::new(),
            highest_line_segment_id: 0,
            circles: HashMap::new(),
            highest_circle_id: 0,
            arcs: HashMap::new(),
            highest_arc_id: 0,
            constraints: HashMap::new(),
            highest_constraint_id: 0,
            faces: vec![],
        };

        let o = plane.plane.origin.clone();
        let x = plane.plane.primary.clone();
        let y = plane.plane.secondary.clone();

        for (point_id, point) in sketch.points.iter() {
            let pt3 = o.plus(x.times(point.x)).plus(y.times(point.y));
            let mut real_point = Point3::new(pt3.x, pt3.y, pt3.z);
            if point.hidden {
                real_point.hidden = true;
            }
            real_sketch.points.insert(*point_id, real_point);

            let pt2 = point.clone();
            real_sketch.points_2d.insert(*point_id, pt2);
        }
        real_sketch.highest_point_id = sketch.highest_point_id;

        for (line_id, line) in sketch.line_segments.iter() {
            let real_line = Line3 {
                start: line.start,
                end: line.end,
            };
            real_sketch.line_segments.insert(*line_id, real_line);
        }

        for (circle_id, circle) in sketch.circles.iter() {
            let real_circle = Circle3 {
                center: circle.center,
                radius: circle.radius,
                top: circle.top,
            };
            real_sketch.circles.insert(*circle_id, real_circle);
        }

        // let mut arc3_lookup: HashMap<(u64, u64, u64), Arc3> = HashMap::new();
        for (arc_id, arc) in sketch.arcs.iter() {
            // println!("\nConverting arc to points");
            // let as_points = sketch.arc_to_points(arc);
            // println!("How many points? {}", as_points.len());
            // let transit = as_points[as_points.len() / 2].clone();

            // let transit_3d = o.plus(x.times(transit.x)).plus(y.times(transit.y));
            // let mut real_point = Point3::new(transit_3d.x, transit_3d.y, transit_3d.z);
            // real_point.hidden = true;

            // let point_id = real_sketch.highest_point_id + 1;
            // real_sketch.points.insert(point_id, real_point);
            // real_sketch.points_2d.insert(point_id, transit);
            // real_sketch.highest_point_id += 1;

            let real_arc = Arc3 {
                center: arc.center,
                start: arc.start,
                end: arc.end,
                // transit: point_id,
                clockwise: arc.clockwise,
            };
            real_sketch.arcs.insert(*arc_id, real_arc);
            // arc3_lookup.insert((arc.start, arc.end, arc.center), real_arc);
        }

        for (constraint_id, constraint) in sketch.constraints.iter() {
            let real_constraint = constraint.clone();
            real_sketch
                .constraints
                .insert(*constraint_id, real_constraint);
        }

        let (faces, _unused_segments) = sketch.find_faces();
        real_sketch.faces = faces;

        real_sketch
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RealPlane {
    pub plane: Plane,
    pub name: String,
    pub width: f64,
    pub height: f64,
}

#[cfg(test)]
pub mod tests {
    use truck_polymesh::obj;

    use crate::extrusion::Direction;
    use crate::extrusion::Extrusion;
    use crate::extrusion::ExtrusionMode;
    use crate::message::Message;
    use truck_meshalgo::filters::*;
    use truck_meshalgo::tessellation::*;

    use super::*;

    pub fn create_test_project() -> Project {
        let mut p = Project::new("Test Project");
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch_to_plane("Sketch 1", "Plane-0");
        let s = wb.get_sketch_mut("Sketch 1").unwrap();
        let ll = s.add_point(0.0, 0.0);
        let lr = s.add_point(40.0, 0.0);
        let ul = s.add_point(0.0, 40.0);
        let ur = s.add_point(40.0, 40.0);
        s.add_segment(ll, lr);
        s.add_segment(lr, ur);
        s.add_segment(ur, ul);
        s.add_segment(ul, ll);

        let extrusion = Extrusion::new(
            "Sketch-0".to_owned(),
            vec![0],
            25.0,
            0.0,
            Direction::Normal,
            ExtrusionMode::New,
        );
        wb.add_extrusion("Ext1", extrusion);

        p
    }

    #[test]
    fn one_extrusion() {
        let p = create_test_project();

        let realization = p.get_realization(0, 1000);
        let solids = realization.solids;

        let solid = &solids["Ext1:0"];

        println!("{:?}", solid);
    }

    // #[test]
    // fn move_sketch() {
    //     let mut p = Project::new("Test Project");

    //     let right_plane_id = p.workbenches[0].plane_name_to_id("Right").unwrap();

    //     let message = &Message::SetSketchPlane {
    //         workbench_id: 0,
    //         sketch_id: "Sketch-0".to_owned(),
    //         plane_id: right_plane_id,
    //     };

    //     let result = p.handle_message(message);
    //     match result {
    //         Ok(res) => println!("{}", res),
    //         Err(e) => println!("{}", e),
    //     }
    //     // println!("{:?}", result);

    //     let realization = p.get_realization(0, 1000);
    // }

    #[test]
    fn rename_plane() {
        let mut p = create_test_project();

        let message = &Message::RenameStep {
            workbench_id: 0,
            step_id: 1,
            new_name: "Top-2".to_owned(),
        };

        let result = message.handle(&mut p);
        match result {
            Ok(res) => println!("{}", res),
            Err(e) => println!("{}", e),
        }
        // let realization = p.get_realization(0, 1000);
    }

    // Removed because this seems pretty redundant with all the other tests that read .cadmium files
    // #[test]
    // fn to_and_from_json() {
    //     // let mut p = Project::new("Test Project");

    //     let file_contents =
    //         std::fs::read_to_string("/Users/matthewferraro/Downloads/first_project.cadmium")
    //             .unwrap();

    //     let p2 = Project::from_json(&file_contents);
    //     println!("{:?}", p2);
    // }

    #[test]
    fn circle_crashing() {
        let file_contents =
            std::fs::read_to_string("src/test_inputs/circle_crashing_2.cadmium").unwrap();

        let p2 = Project::from_json(&file_contents);

        let realization = p2.get_realization(0, 1000);
        println!("{:?}", realization);
    }

    // #[test]
    fn bruno() {
        let mut p = create_test_project();
        let wb = p.workbenches.get_mut(0).unwrap();

        let s2_id = wb.add_sketch_to_solid_face("Sketch-2", "Ext1:0", Vector3::new(0.0, 0.0, 1.0));
        let s2 = wb.get_sketch_mut("Sketch-2").unwrap();

        // smaller
        let ll = s2.add_point(12.0, 12.0);
        let lr = s2.add_point(32.0, 12.0);
        let ul = s2.add_point(12.0, 32.0);
        let ur = s2.add_point(32.0, 32.0);
        // bigger!
        // let ll = s2.add_point(-10.0, -10.0);
        // let lr = s2.add_point(50.0, -10.0);
        // let ul = s2.add_point(-10.0, 50.0);
        // let ur = s2.add_point(50.0, 50.0);
        s2.add_segment(ll, lr);
        s2.add_segment(lr, ur);
        s2.add_segment(ur, ul);
        s2.add_segment(ul, ll);

        // println!("S2: {:?}", s2);

        let extrusion2 = Extrusion::new(
            s2_id.to_owned(),
            vec![0],
            25.0,
            0.0,
            Direction::Normal,
            ExtrusionMode::Add(vec!["Ext1:0".to_string()]),
        );
        wb.add_extrusion("Ext2", extrusion2);

        wb.add_sketch_to_plane("Sketch 3", "Plane-1");
        let s3 = wb.get_sketch_mut("Sketch 3").unwrap();
        let center = s3.add_point(20.0, 15.0);
        s3.add_circle(center, 5.0);

        let extrusion3 = Extrusion::new(
            "Sketch-2".to_owned(),
            vec![0],
            50.0,
            0.0,
            Direction::NegativeNormal,
            ExtrusionMode::Remove(vec!["Ext1:0".to_string()]),
        );
        wb.add_extrusion("Ext3", extrusion3);

        let realization = p.get_realization(0, 1000);
        let solids = realization.solids;

        let num_solids = solids.len();
        println!("Num Solids: {:?}", num_solids);
        assert!(num_solids == 1);

        let final_solid = &solids["Ext1:0"];
        println!("Final solid: {:?}", final_solid.truck_solid);
        let mut mesh = final_solid.truck_solid.triangulation(0.02).to_polygon();
        mesh.put_together_same_attrs(0.1);
        let file = std::fs::File::create("pkg/bruno.obj").unwrap();
        obj::write(&mesh, file).unwrap();

        let file = std::fs::File::create("pkg/bruno.json").unwrap();
        serde_json::to_writer(file, &p).unwrap();
    }

    // #[test]
    fn secondary_extrusion_with_merge() {
        let mut p = create_test_project();
        let wb = p.workbenches.get_mut(0).unwrap();

        let s2_id = wb.add_sketch_to_solid_face("Sketch-2", "Ext1:0", Vector3::new(0.0, 0.0, 1.0));
        let s2 = wb.get_sketch_mut("Sketch-2").unwrap();

        // smaller
        let ll = s2.add_point(12.0, 0.0);
        let lr = s2.add_point(32.0, 0.0);
        let ul = s2.add_point(12.0, 32.0);
        let ur = s2.add_point(32.0, 32.0);
        s2.add_segment(ll, lr);
        s2.add_segment(lr, ur);
        s2.add_segment(ur, ul);
        s2.add_segment(ul, ll);

        // println!("S2: {:?}", s2);

        let extrusion2 = Extrusion::new(
            s2_id.to_owned(),
            vec![0],
            25.0,
            0.0,
            Direction::Normal,
            ExtrusionMode::Add(vec!["Ext1:0".to_string()]),
        );
        wb.add_extrusion("Ext2", extrusion2);

        let realization = p.get_realization(0, 1000);
        let solids = realization.solids;

        let num_solids = solids.len();
        println!("Num Solids: {:?}", num_solids);
        assert!(num_solids == 1);

        let final_solid = &solids["Ext1:0"];
        let mut mesh = final_solid.truck_solid.triangulation(0.02).to_polygon();
        mesh.put_together_same_attrs(0.1);
        let file = std::fs::File::create("secondary_extrusion.obj").unwrap();
        obj::write(&mesh, file).unwrap();

        let file = std::fs::File::create("secondary_extrusion.json").unwrap();
        serde_json::to_writer(file, &p).unwrap();
    }
}
