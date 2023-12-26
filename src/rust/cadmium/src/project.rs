use crate::{
    extrusion::{Direction, Extrusion, Solid},
    sketch::{Constraint, Face, Point2, Sketch},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub assemblies: Vec<Assembly>,
    pub workbenches: Vec<Workbench>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        let p = Project {
            name: name.to_owned(),
            assemblies: vec![],
            workbenches: vec![],
        };

        p
    }

    pub fn add_defaults(&mut self) {
        let mut w = Workbench::new("Workbench 1");
        w.add_defaults();
        self.workbenches.push(w);

        let mut w2 = Workbench::new("Workbench 2");
        w2.add_defaults_2();
        self.workbenches.push(w2);
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
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let current_step_name = workbench.history[*step_id as usize].name.clone();
                let current_step = workbench.history.get(*step_id as usize).unwrap();

                self.workbenches[*workbench_id as usize]
                    .history
                    .get_mut(*step_id as usize)
                    .unwrap()
                    .name = new_name.to_owned();

                Ok(format!("\"name\": \"{}\"", new_name))
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
            Message::NewPointOnSketch {
                workbench_id,
                sketch_id,
                point_id,
                x,
                y,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];
                let sketch = workbench.get_sketch_by_id_mut(sketch_id).unwrap();
                sketch.add_point_with_id(*x, *y, *point_id);
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
                    max_change = sketch.step();
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
                Ok(("".to_owned()))
            }
            Message::NewSketch {
                workbench_id,
                sketch_name,
                plane_name,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];

                workbench.add_sketch(&sketch_name, &plane_name);
                Ok(format!("\"sketch_name\": \"{}\"", sketch_name))
            }
            Message::UpdateSketchPlane {
                workbench_id,
                sketch_name,
                plane_id: pid,
            } => {
                let workbench = &mut self.workbenches[*workbench_id as usize];

                for step in workbench.history.iter_mut() {
                    if step.name == *sketch_name {
                        match &mut step.data {
                            StepData::Sketch {
                                plane_id: pn2,
                                width,
                                height,
                                sketch,
                            } => {
                                *pn2 = pid.to_owned();

                                return Ok(format!("\"plane_name\": \"{}\"", pid));
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
                let extrusion = Extrusion {
                    sketch_id: sketch_id.to_owned(),
                    face_ids: face_ids.to_owned(),
                    length: *length,
                    offset: *offset,
                    direction: direction.to_owned(),
                };
                workbench.add_extrusion(extrusion_name, extrusion);
                Ok("".to_owned())
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Assembly {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workbench {
    name: String,
    history: Vec<Step>,
    step_counters: HashMap<String, u64>,
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
                    plane,
                    width,
                    height,
                } => {
                    return Some(step.unique_id.clone());
                }
                _ => {}
            }
        }
        None
    }

    pub fn last_plane_id(&self) -> Option<String> {
        let mut last_plane_id = None;
        for step in self.history.iter() {
            match &step.data {
                StepData::Plane {
                    plane,
                    width,
                    height,
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
                    plane_id: _,
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
                    plane_id: _,
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

        let sketch_id = self.add_sketch("Sketch 1", "Top");

        let sketch = self.get_sketch_mut("Sketch 1").unwrap();

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

        self.add_extrusion(
            "Ext 1",
            Extrusion {
                sketch_id,
                face_ids: vec![0, 1],
                length: 0.15,
                offset: 0.0,
                direction: Direction::Normal,
            },
        );
    }

    pub fn add_defaults(&mut self) {
        self.add_point("Origin", Point3::new(0.0, 0.0, 0.0));
        self.add_plane("Top", Plane::top());
        self.add_plane("Front", Plane::front());
        self.add_plane("Right", Plane::right());

        let sketch_id = self.add_sketch("Sketch 1", "Top");

        let sketch = self.get_sketch_mut("Sketch 1").unwrap();

        // square in upper right
        let p0 = sketch.add_fixed_point(0.1, 0.00);
        let p1 = sketch.add_point(0.45, 0.0);
        let p2 = sketch.add_point(0.45, 0.25);
        let p3 = sketch.add_point(0.0, 0.25);
        let seg_0 = sketch.add_segment(p0, p1);
        let seg_1 = sketch.add_segment(p1, p2);
        let seg_2 = sketch.add_segment(p2, p3);
        let seg_3 = sketch.add_segment(p3, p0);

        let big_p0 = sketch.add_point(-0.1, -0.1);
        let big_p1 = sketch.add_point(0.55, -0.1);
        let big_p2 = sketch.add_point(0.55, 0.55);
        let big_p3 = sketch.add_point(-0.1, 0.55);
        let big_seg_0 = sketch.add_segment(big_p0, big_p1);
        let big_seg_1 = sketch.add_segment(big_p1, big_p2);
        let big_seg_2 = sketch.add_segment(big_p2, big_p3);
        let big_seg_3 = sketch.add_segment(big_p3, big_p0);

        // sketch.add_segment_vertical_constraint(seg_3);
        // sketch.add_segment_horizontal_constraint(seg_0);
        // sketch.add_segment_length_constraint(seg_0, 0.52);
        // sketch.add_segment_length_constraint(seg_1, 0.52);
        // sketch.add_segment_length_constraint(seg_2, 0.52);
        // sketch.add_segment_length_constraint(seg_3, 0.52);

        // Simple circle in lower left
        let p4 = sketch.add_point(-0.5, -0.25);
        sketch.add_circle(p4, 0.3);

        // // intersecting circle!
        let p5 = sketch.add_point(-0.8, -0.25);
        let c2 = sketch.add_circle(p5, 0.3);

        // sketch.add_circle_diameter_constraint(c2, 0.6);

        // Rounded square in lower right
        let shrink = 0.4;
        let offset_x = 0.1;
        let offset_y = -0.70;
        let a = sketch.add_point(0.25 * shrink + offset_x, 0.00 * shrink + offset_y);
        let b = sketch.add_point(0.75 * shrink + offset_x, 0.00 * shrink + offset_y);
        let c = sketch.add_point(1.00 * shrink + offset_x, 0.25 * shrink + offset_y);
        let d = sketch.add_point(1.00 * shrink + offset_x, 0.75 * shrink + offset_y);
        let e = sketch.add_point(0.75 * shrink + offset_x, 1.00 * shrink + offset_y);
        let f = sketch.add_point(0.25 * shrink + offset_x, 1.00 * shrink + offset_y);
        let g = sketch.add_point(0.00 * shrink + offset_x, 0.75 * shrink + offset_y);
        let h = sketch.add_point(0.00 * shrink + offset_x, 0.25 * shrink + offset_y);
        let i = sketch.add_point(0.75 * shrink + offset_x, 0.25 * shrink + offset_y);
        let j = sketch.add_point(0.75 * shrink + offset_x, 0.75 * shrink + offset_y);
        let k = sketch.add_point(0.25 * shrink + offset_x, 0.75 * shrink + offset_y);
        let l = sketch.add_point(0.25 * shrink + offset_x, 0.25 * shrink + offset_y);

        sketch.add_segment(a, b);
        sketch.add_arc(i, b, c, false);
        sketch.add_segment(c, d);
        sketch.add_arc(j, d, e, false);
        sketch.add_segment(e, f);
        sketch.add_arc(k, f, g, false);
        sketch.add_segment(g, h);
        sketch.add_arc(l, h, a, false);

        self.add_extrusion(
            "Ext 1",
            Extrusion {
                sketch_id,
                face_ids: vec![2, 3],
                length: 0.25,
                offset: 0.0,
                direction: Direction::Normal,
            },
        );

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

    pub fn add_plane(&mut self, name: &str, plane: Plane) {
        let counter = self.step_counters.get_mut("Plane").unwrap();
        self.history.push(Step::new_plane(name, plane, *counter));
        *counter += 1;
    }

    pub fn plane_name_to_id(&self, plane_name: &str) -> Option<String> {
        for step in self.history.iter() {
            if step.name == plane_name {
                match &step.data {
                    StepData::Plane {
                        plane,
                        width,
                        height,
                    } => {
                        return Some(step.unique_id.clone());
                    }
                    _ => {}
                }
            }
        }
        None
    }

    pub fn add_sketch(&mut self, name: &str, plane_name: &str) -> String {
        // if the plane name is empty string, default to whichever plane is currently first
        // in the feature history
        let plane_id = if plane_name == "" {
            self.get_first_plane_id().unwrap()
        } else {
            self.plane_name_to_id(plane_name).unwrap()
        };

        // If the sketch name is empty string, then we need to generate a new name
        // Let's use "Sketch n" where n is the number of sketches
        let counter = self.step_counters.get_mut("Sketch").unwrap();
        let sketch_name = if name == "" {
            format!("Sketch {}", counter)
        } else {
            name.to_owned()
        };

        let new_step = Step::new_sketch(name, &plane_id, *counter);
        let new_step_id = new_step.unique_id.clone();
        self.history.push(new_step);
        *counter += 1;

        new_step_id
    }

    pub fn add_extrusion(&mut self, name: &str, extrusion: Extrusion) {
        let counter = self.step_counters.get_mut("Extrusion").unwrap();
        self.history
            .push(Step::new_extrusion(name, extrusion, *counter));
        *counter += 1;
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
                    width,
                    height,
                    plane_id,
                    sketch,
                } => {
                    let plane = &realized.planes[plane_id];

                    realized.sketches.insert(
                        step.unique_id.to_owned(),
                        (
                            RealSketch::new(&plane.name, plane_id, plane, sketch),
                            RealSketch::new(
                                &plane.name,
                                plane_id,
                                plane,
                                &sketch.split_intersections(),
                            ),
                            step.name.clone(),
                        ),
                    );
                }
                StepData::Extrusion { extrusion } => {
                    let (_sketch, split_sketch, _name) = &realized.sketches[&extrusion.sketch_id];
                    let plane = &realized.planes[&split_sketch.plane_id];
                    let solids =
                        Solid::from_extrusion(step.name.clone(), plane, split_sketch, extrusion);
                    for (name, solid) in solids {
                        realized.solids.insert(name, solid);
                    }
                }
                _ => println!("Unknown step type"),
            }
        }

        realized
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Realization {
    // a Realization is what you get if you apply the steps in a Workbench's
    // history and build a bunch of geometry
    pub planes: HashMap<String, RealPlane>,
    pub points: HashMap<String, Point3>,
    pub sketches: HashMap<String, (RealSketch, RealSketch, String)>,
    pub solids: HashMap<String, Solid>,
}

impl Realization {
    pub fn new() -> Self {
        Realization {
            planes: HashMap::new(),
            points: HashMap::new(),
            sketches: HashMap::new(),
            solids: HashMap::new(),
        }
    }

    pub fn solid_to_obj(&self, solid_name: &str, tolerance: f64) -> String {
        let solid = &self.solids[solid_name];
        let obj_text = solid.to_obj_string(tolerance);
        obj_text
    }

    pub fn save_solid_as_obj_file(&self, solid_name: &str, filename: &str, tolerance: f64) {
        let solid = &self.solids[solid_name];
        solid.save_as_obj(filename, tolerance);
    }

    pub fn solid_to_step(&self, solid_name: &str) -> String {
        let solid = &self.solids[solid_name];
        let step_text = solid.to_step_string();
        step_text
    }

    pub fn save_solid_as_step_file(&self, solid_name: &str, filename: &str) {
        let solid = &self.solids[solid_name];
        solid.save_as_step(filename)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: String,
    unique_id: String,
    suppressed: bool,
    data: StepData,
}

impl Step {
    pub fn new_point(name: &str, point: Point3, point_id: u64) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Point-{}", point_id),
            suppressed: false,
            data: StepData::Point {
                point: point.clone(),
            },
        }
    }

    pub fn new_plane(name: &str, plane: Plane, plane_id: u64) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Plane-{}", plane_id),
            suppressed: false,
            data: StepData::Plane {
                plane,
                height: 1.0,
                width: 1.0,
            },
        }
    }

    pub fn new_sketch(name: &str, plane_id: &str, sketch_id: u64) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Sketch-{}", sketch_id),
            suppressed: false,
            data: StepData::Sketch {
                plane_id: plane_id.to_owned(),
                width: 1.25,
                height: 0.75,
                sketch: Sketch::new(),
            },
        }
    }

    pub fn new_extrusion(name: &str, extrusion: Extrusion, extrusion_id: u64) -> Self {
        Step {
            name: name.to_owned(),
            unique_id: format!("Extrusion-{}", extrusion_id),
            suppressed: false,
            data: StepData::Extrusion { extrusion },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StepData {
    Point {
        point: Point3,
    },
    Plane {
        plane: Plane,
        width: f64,
        height: f64,
    },
    Sketch {
        plane_id: String,
        width: f64,
        height: f64,
        sketch: Sketch,
    },
    Extrusion {
        extrusion: Extrusion,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line3 {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arc3 {
    center: u64,
    start: u64,
    end: u64,
    clockwise: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle3 {
    center: u64,
    radius: f64,
    top: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

        let mut arc3_lookup: HashMap<(u64, u64, u64), Arc3> = HashMap::new();
        for (arc_id, arc) in sketch.arcs.iter() {
            // println!("\nConverting arc to points");
            let as_points = sketch.arc_to_points(arc);
            // println!("How many points? {}", as_points.len());
            let transit = as_points[as_points.len() / 2].clone();

            let transit_3d = o.plus(x.times(transit.x)).plus(y.times(transit.y));
            let mut real_point = Point3::new(transit_3d.x, transit_3d.y, transit_3d.z);
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

        println!("Finding faces?");
        let (faces, unused_segments) = sketch.find_faces();
        real_sketch.faces = faces;

        real_sketch
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealPlane {
    pub plane: Plane,
    pub name: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    RenameWorkbench {
        workbench_id: u64,
        new_name: String,
    },
    RenameStep {
        workbench_id: u64,
        step_id: u64,
        new_name: String,
    },
    RenameProject {
        new_name: String,
    },
    NewPointOnSketch {
        workbench_id: u64,
        sketch_id: String,
        point_id: u64,
        x: f64,
        y: f64,
    },
    NewPointOnSketch2 {
        workbench_id: u64,
        sketch_id: String,
        x: f64,
        y: f64,
        hidden: bool,
    },
    NewCircleBetweenPoints {
        workbench_id: u64,
        sketch_id: String,
        center_id: u64,
        edge_id: u64,
    },
    NewLineOnSketch {
        workbench_id: u64,
        sketch_id: String,
        start_point_id: u64,
        end_point_id: u64,
    },
    DeleteLineSegment {
        workbench_id: u64,
        sketch_name: String,
        line_segment_id: u64,
    },
    StepSketch {
        workbench_id: u64,
        sketch_name: String,
        steps: u64,
    },
    SolveSketch {
        workbench_id: u64,
        sketch_name: String,
        max_steps: u64,
    },
    NewSketch {
        workbench_id: u64,
        sketch_name: String,
        plane_name: String,
    },
    UpdateSketchPlane {
        workbench_id: u64,
        sketch_name: String,
        plane_id: String,
    },
    DeleteSketch {
        workbench_id: u64,
        sketch_name: String,
    },
    NewExtrusion {
        workbench_id: u64,
        extrusion_name: String,
        sketch_id: String,
        face_ids: Vec<u64>,
        length: f64,
        offset: f64,
        direction: Direction,
    },
    UpdateExtrusionLength {
        workbench_id: u64,
        extrusion_name: String,
        length: f64,
    },
}

impl Message {
    pub fn as_json(&self) -> String {
        let result = serde_json::to_string(self);
        match result {
            Ok(json) => json,
            Err(e) => format!("Error: {}", e),
        }
    }

    pub fn from_json(json: &str) -> Result<Message, String> {
        let result = serde_json::from_str(json);
        match result {
            Ok(msg) => Ok(msg),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_project() {
        let mut p = Project::new("Test Project");
        p.add_defaults();
        // let r = p.get_realization(0, 1000);

        // let msg = &Message::NewPointOnSketch {
        //     workbench_id: 0,
        //     sketch_name: "Sketch 1".to_owned(),
        //     point_id: 100,
        //     x: -1.0,
        //     y: -1.0,
        // };

        let json = r#"{"NewPointOnSketch":{"workbench_id":0,"sketch_id":"Sketch-0","point_id":100,"x":-1.0,"y":-1.0}}"#;
        let msg = Message::from_json(json).unwrap();
        let res = p.handle_message(&msg);

        println!("As json: {}", msg.as_json());

        // println!("{:?}", res);
    }

    #[test]
    fn one_extrusion() {
        let mut p = Project::new("Test Project");
        p.add_defaults();

        let message = &Message::NewSketch {
            workbench_id: 0,
            sketch_name: "".to_owned(),
            plane_name: "".to_owned(),
        };

        let result = p.handle_message(message);
        println!("{:?}", result);

        let realization = p.get_realization(0, 1000);
    }

    #[test]
    fn move_sketch() {
        let mut p = Project::new("Test Project");
        p.add_defaults();

        let right_plane_id = p.workbenches[0].plane_name_to_id("Right").unwrap();

        let message = &Message::UpdateSketchPlane {
            workbench_id: 0,
            sketch_name: "Sketch 1".to_owned(),
            plane_id: right_plane_id,
        };

        let result = p.handle_message(message);
        match result {
            Ok(res) => println!("{}", res),
            Err(e) => println!("{}", e),
        }
        // println!("{:?}", result);

        let realization = p.get_realization(0, 1000);
    }

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
        let realization = p.get_realization(0, 1000);
    }

    #[test]
    fn to_and_from_json() {
        // let mut p = Project::new("Test Project");
        // p.add_defaults();

        let file_contents =
            std::fs::read_to_string("/Users/matthewferraro/Downloads/first_project.cadmium")
                .unwrap();

        let p2 = Project::from_json(&file_contents);
        println!("{:?}", p2);
    }

    #[test]
    fn circle_crashing() {
        // let mut p = Project::new("Test Project");
        // p.add_defaults();

        let file_contents =
            std::fs::read_to_string("/Users/matthewferraro/Downloads/circle_crashing_2.cadmium")
                .unwrap();

        let p2 = Project::from_json(&file_contents);

        let realization = p2.get_realization(0, 1000);
        println!("{:?}", realization);
    }
}
