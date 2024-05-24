use itertools::Itertools as _;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::message::Message;
use crate::realization::Realization;
use crate::sketch::constraints::Constraint;
use crate::step::StepData;
use crate::workbench::Workbench;
use crate::sketch::{Face, Point2, Sketch};
use crate::extrusion::{Extrusion, ExtrusionMode};
use std::collections::HashMap;
use truck_polymesh::InnerSpace;

// use truck_base::math::Vector3 as truck_vector3;
use truck_modeling::Plane as TruckPlane;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Project {
    pub name: String,
    pub assemblies: Vec<Assembly>,
    pub workbenches: Vec<Workbench>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        Project {
            name: name.to_owned(),
            assemblies: vec![],
            workbenches: vec![],
        }
    }

    pub fn add_defaults(&mut self) {
        let mut w = Workbench::new("Workbench 1");
        w.add_defaults();
        self.workbenches.push(w);
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

    pub fn get_workbench_mut(&mut self, name: &str) -> Option<&mut Workbench> {
        for workbench in self.workbenches.iter_mut() {
            if workbench.name == name {
                return Some(workbench);
            }
        }
        None
    }

    pub fn get_realization(&self, workbench_id: u64, max_steps: u64) -> Realization {
        let workbench = &self.workbenches[workbench_id as usize];
        let realization = workbench.realize(max_steps);
        realization
    }

    pub fn handle_message_string(&mut self, message_string: &str) -> Result<String, String> {
        let message = Message::from_json(message_string);
        match message {
            Err(e) => Err(format!("parsing_error: \"{}\"", e)),
            Ok(msg) => {
                let result = self.handle_message(&msg);

                match result {
                    Ok(res) => Ok(res),
                    Err(e) => Err(format!("message_handling_error: \"{}\"", e)),
                }
            }
        }
    }

    pub fn handle_message(&mut self, message: &Message) -> Result<String, String> {
        match message {
            Message::RenameProject { new_name } => {
                self.name = new_name.to_owned();
                Ok(format!("\"name\": \"{}\"", new_name))
            }
            Message::RenameWorkbench {
                workbench_id,
                new_name,
            } => {
                self.workbenches[*workbench_id as usize].name = new_name.to_owned();
                Ok(format!("\"name\": \"{}\"", new_name))
            }
            Message::RenameStep {
                workbench_id,
                step_id,
                new_name,
            } => {
                // let workbench = &mut self.workbenches[*workbench_id as usize];
                // let current_step_name = workbench.history[*step_id as usize].name.clone();
                // let current_step = workbench.history.get(*step_id as usize).unwrap();

                self.workbenches[*workbench_id as usize]
                    .history
                    .get_mut(*step_id as usize)
                    .unwrap()
                    .name = new_name.to_owned();

                Ok(format!("\"name\": \"{}\"", new_name))
            }
            Message::DeleteLines {
                workbench_id,
                sketch_id,
                line_ids,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                for line_id in line_ids {
                    sketch.delete_line_segment(*line_id);
                }
                Ok("".to_owned())
            }
            Message::DeleteArcs {
                workbench_id,
                sketch_id,
                arc_ids,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                for arc_id in arc_ids {
                    sketch.delete_arc(*arc_id);
                }
                Ok("".to_owned())
            }
            Message::DeleteCircles {
                workbench_id,
                sketch_id,
                circle_ids,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                for circle_id in circle_ids {
                    sketch.delete_circle(*circle_id);
                }
                Ok("".to_owned())
            }
            Message::NewPointOnSketch2 {
                workbench_id,
                sketch_id,
                x,
                y,
                hidden,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                let point_id;
                if *hidden {
                    point_id = sketch.add_hidden_point(*x, *y);
                } else {
                    point_id = sketch.add_point(*x, *y);
                }

                Ok(format!("\"id\": \"{}\"", point_id))
            }
            Message::NewCircleBetweenPoints {
                workbench_id,
                sketch_id,
                center_id,
                edge_id,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                let circle_id = sketch.add_circle_between_points(*center_id, *edge_id);
                Ok(format!("\"id\": \"{}\"", circle_id))
            }
            Message::NewRectangleBetweenPoints {
                workbench_id,
                sketch_id,
                start_id,
                end_id,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                let (point_ids, line_ids) = sketch.add_rectangle_between_points(*start_id, *end_id);
                Ok(format!(
                    "\"point_ids\": [{}], \"line_ids\": [{}]",
                    point_ids.iter().join(","),
                    line_ids.iter().join(",")
                ))
            }
            Message::NewPointOnSketch {
                workbench_id,
                sketch_id,
                point_id,
                x,
                y,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                sketch.add_point_with_id(*x, *y, *point_id).unwrap();
                Ok("".to_owned())
            }
            Message::NewLineOnSketch {
                workbench_id,
                sketch_id,
                start_point_id,
                end_point_id,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                let line_id = sketch.add_segment(*start_point_id, *end_point_id);
                Ok(format!("\"id\": \"{}\"", line_id))
            }
            Message::DeleteLineSegment {
                workbench_id,
                sketch_name,
                line_segment_id,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_mut(sketch_name).unwrap();
                sketch.delete_line_segment(*line_segment_id);
                Ok("".to_owned())
            }
            Message::StepSketch {
                workbench_id,
                sketch_name,
                steps,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_mut(sketch_name).unwrap();
                let mut max_change = 0.0;
                for _ in 0..*steps {
                    max_change = sketch.take_a_step();
                }
                Ok(format!("{}", max_change))
            }
            Message::SolveSketch {
                workbench_id,
                sketch_name,
                max_steps,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_mut(sketch_name).unwrap();
                sketch.solve(*max_steps);
                Ok("".to_owned())
            }
            Message::NewSketchOnPlane {
                workbench_id,
                sketch_name,
                plane_id,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];

                let new_sketch_id = workbench.add_sketch_to_plane(&sketch_name, &plane_id);
                Ok(format!("\"sketch_id\": \"{}\"", new_sketch_id))
            }
            Message::SetSketchPlane {
                workbench_id,
                sketch_id,
                plane_id: pid,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];

                for step in workbench.history.iter_mut() {
                    if step.unique_id == *sketch_id {
                        match &mut step.data {
                            StepData::Sketch {
                                plane_description,
                                width: _width,
                                height: _height,
                                sketch: _sketch,
                            } => {
                                match plane_description {
                                    PlaneDescription::PlaneId(plane_id) => {
                                        *plane_id = pid.to_owned();
                                        return Ok(format!("\"plane_id\": \"{}\"", pid));
                                    }
                                    _ => {
                                        panic!("Not implemented yet");
                                    }
                                }
                                // *pn2 = pid.to_owned();
                            }
                            _ => {}
                        }
                    }
                }

                Ok("".to_owned())
            }
            Message::DeleteSketch {
                workbench_id,
                sketch_name,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let mut index = 0;
                for step in workbench.history.iter() {
                    match &step.data {
                        StepData::Sketch { .. } => {
                            if step.name == *sketch_name {
                                break;
                            }
                        }
                        _ => {}
                    }
                    index += 1;
                }
                workbench.history.remove(index);
                Ok("".to_owned())
            }
            Message::NewExtrusion {
                workbench_id,
                extrusion_name,
                sketch_id,
                face_ids,
                length,
                offset,
                direction,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let extrusion = Extrusion::new(
                    sketch_id.to_owned(),
                    face_ids.to_owned(),
                    *length,
                    *offset,
                    direction.to_owned(),
                    ExtrusionMode::New,
                );
                let extrusion_id = workbench.add_extrusion(extrusion_name, extrusion);
                Ok(format!("\"id\": \"{}\"", extrusion_id))
            }
            Message::UpdateExtrusion {
                workbench_id,
                extrusion_name: _extrusion_name,
                extrusion_id,
                sketch_id,
                face_ids,
                length,
                offset,
                direction,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let extrusion = Extrusion::new(
                    sketch_id.to_owned(),
                    face_ids.to_owned(),
                    *length,
                    *offset,
                    direction.to_owned(),
                    ExtrusionMode::New,
                );
                let as_step_data = StepData::Extrusion { extrusion };
                workbench.update_step_data(extrusion_id, as_step_data);
                Ok(format!("\"id\": \"{}\"", extrusion_id))
            }
            Message::UpdateExtrusionLength {
                workbench_id,
                extrusion_name,
                length,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                for step in workbench.history.iter_mut() {
                    match &mut step.data {
                        StepData::Extrusion { extrusion } => {
                            if step.name == *extrusion_name {
                                extrusion.length = *length;
                                return Ok(format!("\"length\": {}", length));
                            }
                        }
                        _ => {}
                    }
                }
                Err(format!("Extrusion {} not found", extrusion_name))
            }
        }
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Assembly {
    name: String,
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PlaneDescription {
    PlaneId(String),
    SolidFace { solid_id: String, normal: Vector3 },
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Plane {
    pub origin: Point3,
    pub primary: Vector3,
    pub secondary: Vector3,
    pub tertiary: Vector3, // aka Normal
}

impl Plane {
    /*

    z    y

    ^   ^
    |  /
    | /
    |/
    |-------->  x

    So "front" is xz plane with -y normal
    and "top" is xy plane with z normal
    and "right" is yz plane with x normal

     */

    pub fn new(origin: Point3, primary: Vector3, secondary: Vector3, tertiary: Vector3) -> Self {
        Plane {
            origin,
            primary,
            secondary,
            tertiary,
        }
    }

    pub fn front() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(1.0, 0.0, 0.0),
            secondary: Vector3::new(0.0, 0.0, 1.0),
            tertiary: Vector3::new(0.0, -1.0, 0.0),
        }
    }

    pub fn top() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(1.0, 0.0, 0.0),
            secondary: Vector3::new(0.0, 1.0, 0.0),
            tertiary: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn right() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(0.0, 1.0, 0.0),
            secondary: Vector3::new(0.0, 0.0, 1.0),
            tertiary: Vector3::new(1.0, 0.0, 0.0),
        }
    }

    pub fn from_truck(tp: TruckPlane) -> Self {
        let o = tp.origin();
        let u = tp.u_axis().normalize();
        let v = tp.v_axis().normalize();
        let n = tp.normal().normalize();
        Plane {
            origin: Point3::new(o.x, o.y, o.z),
            primary: Vector3::new(u.x, u.y, u.z),
            secondary: Vector3::new(v.x, v.y, v.z),
            tertiary: Vector3::new(n.x, n.y, n.z),
        }
    }

    pub fn project(&self, point: &Point3) -> Point2 {
        let minus_origin = point.minus(&self.origin);
        let x = minus_origin.dot(&self.primary);
        let y = minus_origin.dot(&self.secondary);
        Point2::new(x, y)
    }

    pub fn unproject(&self, point: &Point2) -> Point3 {
        let x = self.origin.plus(self.primary.times(point.x));
        let y = self.origin.plus(self.secondary.times(point.y));
        x.plus(y).to_point3()
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn to_point3(&self) -> Point3 {
        Point3::new(self.x, self.y, self.z)
    }

    pub fn times(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn plus(&self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub hidden: bool,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 {
            x,
            y,
            z,
            hidden: false,
        }
    }

    pub fn plus(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn minus(&self, other: &Point3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn distance_to(&self, other: &Point3) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Line3 {
    pub start: u64,
    pub end: u64,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Arc3 {
    center: u64,
    start: u64,
    end: u64,
    clockwise: bool,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Circle3 {
    center: u64,
    radius: f64,
    top: u64,
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
mod tests {
    use truck_polymesh::obj;

    use crate::extrusion::Direction;
    use truck_meshalgo::tessellation::*;
    use truck_meshalgo::filters::*;

    use super::*;

    #[test]
    fn one_extrusion() {
        let mut p = Project::new("Test Project");
        p.add_defaults();
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

        let realization = p.get_realization(0, 1000);
        let solids = realization.solids;

        let solid = &solids["Ext1:0"];

        println!("{:?}", solid);
    }

    // #[test]
    // fn move_sketch() {
    //     let mut p = Project::new("Test Project");
    //     p.add_defaults();

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
        let mut p = Project::new("Test Project");
        p.add_defaults();

        let message = &Message::RenameStep {
            workbench_id: 0,
            step_id: 1,
            new_name: "Top-2".to_owned(),
        };

        let result = p.handle_message(message);
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
    //     // p.add_defaults();

    //     let file_contents =
    //         std::fs::read_to_string("/Users/matthewferraro/Downloads/first_project.cadmium")
    //             .unwrap();

    //     let p2 = Project::from_json(&file_contents);
    //     println!("{:?}", p2);
    // }

    #[test]
    fn circle_crashing() {
        // let mut p = Project::new("Test Project");
        // p.add_defaults();

        let file_contents =
            std::fs::read_to_string("src/test_inputs/circle_crashing_2.cadmium").unwrap();

        let p2 = Project::from_json(&file_contents);

        let realization = p2.get_realization(0, 1000);
        println!("{:?}", realization);
    }

    // #[test]
    fn bruno() {
        let mut p = Project::new("Test Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch_to_plane("Sketch 1", "Plane-0");
        let s = wb.get_sketch_mut("Sketch 1").unwrap();
        let ll = s.add_point(2.0, 2.0);
        let lr = s.add_point(42.0, 2.0);
        let ul = s.add_point(2.0, 42.0);
        let ur = s.add_point(42.0, 42.0);
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
        mesh.put_together_same_attrs();
        let file = std::fs::File::create("bruno.obj").unwrap();
        obj::write(&mesh, file).unwrap();

        let file = std::fs::File::create("bruno.json").unwrap();
        serde_json::to_writer(file, &p).unwrap();
    }

    // #[test]
    fn secondary_extrusion_with_merge() {
        let mut p = Project::new("Test Project");
        p.add_defaults();
        let wb = p.workbenches.get_mut(0).unwrap();
        wb.add_sketch_to_plane("Sketch 1", "Plane-0");
        let s = wb.get_sketch_mut("Sketch 1").unwrap();
        let ll = s.add_point(2.0, 2.0);
        let lr = s.add_point(42.0, 2.0);
        let ul = s.add_point(2.0, 42.0);
        let ur = s.add_point(42.0, 42.0);
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
        mesh.put_together_same_attrs();
        let file = std::fs::File::create("secondary_extrusion.obj").unwrap();
        obj::write(&mesh, file).unwrap();

        let file = std::fs::File::create("secondary_extrusion.json").unwrap();
        serde_json::to_writer(file, &p).unwrap();
    }
}
