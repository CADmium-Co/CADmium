#![allow(dead_code, unused)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use truck_polymesh::SearchNearestParameterD1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[wasm_bindgen]
pub fn subtract(a: usize, b: usize) -> usize {
    a - b
}

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
}

impl Workbench {
    pub fn new(name: &str) -> Self {
        Workbench {
            name: name.to_owned(),
            history: vec![],
        }
    }

    pub fn add_defaults(&mut self) {
        self.history.push(Step::new_plane("Top", Plane::top()));
        self.history.push(Step::new_plane("Front", Plane::front()));
        self.history.push(Step::new_plane("Right", Plane::right()));
    }

    pub fn add_sketch(&mut self, name: &str, plane_name: &str) {
        self.history.push(Step::new_sketch(name, plane_name));
    }

    pub fn realize(&self, max_steps: u32) -> Realization {
        let mut realized = Realization::new();
        let max_steps = max_steps as usize; // just coerce the type once

        for (step_n, step) in self.history.iter().enumerate() {
            // println!("{:?}", step);
            if step_n >= max_steps {
                break;
            }

            let step_data = &step.data;
            // println!("{:?}", step_data);
            match step_data {
                StepData::Plane {
                    plane,
                    width,
                    height,
                } => {
                    let rp = RealPlane {
                        plane: plane.clone(),
                        width: *width,
                        height: *height,
                    };
                    realized.planes.insert(step.name.to_owned(), rp);
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
}

impl Realization {
    pub fn new() -> Self {
        Realization {
            planes: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    name: String,
    suppressed: bool,
    data: StepData,
}

impl Step {
    pub fn new_plane(name: &str, plane: Plane) -> Self {
        Step {
            name: name.to_owned(),
            suppressed: false,
            data: StepData::Plane {
                plane,
                height: 1.0,
                width: 1.0,
            },
        }
    }

    pub fn new_sketch(name: &str, plane_name: &str) -> Self {
        Step {
            name: name.to_owned(),
            suppressed: false,
            data: StepData::Sketch {
                plane_name: plane_name.to_owned(),
                width: 0.5,
                height: 0.5,
                sketch: Sketch::new(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StepData {
    Plane {
        plane: Plane,
        width: f64,
        height: f64,
    },
    Sketch {
        plane_name: String,
        width: f64,
        height: f64,
        sketch: Sketch,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    origin: Point3,
    primary: Vector3,
    secondary: Vector3,
    tertiary: Vector3, // aka Normal
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 { x, y, z }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealPlane {
    pub plane: Plane,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpringKind {
    Torsion,
    Length,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Spring {
    kp: f64,   // kp is the proportional gain, the spring constant
    kd: f64,   // kd is the derivative gain, the damping constant
    rest: f64, // length is the rest length of the spring
    start_id: u64,
    end_id: u64,
    kind: SpringKind,
}

impl Spring {
    fn new(start_id: u64, end_id: u64, rest: f64, kind: SpringKind) -> Self {
        Spring {
            kp: 2.0,
            kd: 0.3,
            rest,
            start_id,
            end_id,
            kind,
        }
    }

    fn compute_forces(&self, point_a: &Point2, point_b: &Point2) -> Vec<f64> {
        match self.kind {
            SpringKind::Length => self.compute_length_forces(point_a, point_b),
            SpringKind::Torsion => self.compute_torsion_forces(point_a, point_b),
        }
    }

    fn compute_torsion_forces(&self, point_a: &Point2, point_b: &Point2) -> Vec<f64> {
        let dt = 0.01;

        let angle = (point_b.y - point_a.y).atan2(point_b.x - point_a.x);

        // println!("current angle: {}", angle);
        // println!("rest angle: {}", self.resto);
        let err = self.rest - angle;

        let point_a_stepped = point_a.step(dt);
        let point_b_stepped = point_b.step(dt);
        let angle_stepped =
            (point_b_stepped.1 - point_a_stepped.1).atan2(point_b_stepped.0 - point_a_stepped.0);
        // println!("angle_stepped: {}", angle_stepped);
        let d_angle = (angle_stepped - angle) / dt;
        // println!("d_angle: {}", d_angle);
        // let torque = self.kp * err + self.kd * d_angle;
        let torque = self.kp * err - self.kd * d_angle;

        let dx = point_b.x - point_a.x;
        let dy = point_b.y - point_a.y;
        let dist = dx.hypot(dy);
        // println!("dist: {}", dist);

        let f_mag = torque / dist;
        // println!("f_mag: {}", f_mag);

        let fx = f_mag * dy;
        let fy = -f_mag * dx;
        // println!("fx: {}", fx);
        // println!("fy: {}", fy);

        vec![fx, fy, -fx, -fy]
    }

    fn compute_length_forces(&self, point_a: &Point2, point_b: &Point2) -> Vec<f64> {
        let dx = point_b.x - point_a.x;
        let dy = point_b.y - point_a.y;
        let dist = (dx * dx + dy * dy).sqrt();
        let err = dist - self.rest;

        let relative_dx = point_b.dx - point_a.dx;
        let relative_dy = point_b.dy - point_a.dy;

        // project the relative velocity onto the vector between the points
        // a is the velocity
        // b is the vector between the points
        // a dot b / |b|
        let closing_velocity = (relative_dx * dx + relative_dy * dy) / dist;

        let f = self.kp * err + self.kd * closing_velocity;
        let fx = f * dx / dist;
        let fy = f * dy / dist;

        // [fx_a, fy_a, fx_b, fy_b]
        vec![fx, fy, -fx, -fy]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sketch {
    points: Vec<Point2>,
    springs: Vec<Spring>,
}

impl Sketch {
    pub fn new() -> Self {
        Sketch {
            points: vec![],
            springs: vec![],
        }
    }

    fn add_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.points.len();
        self.points.push(Point2::new(x, y));
        id as u64
    }

    fn add_spring(&mut self, start_id: u64, end_id: u64, length: f64) -> u64 {
        let id = self.springs.len();
        let s = Spring::new(start_id, end_id, length, SpringKind::Length);
        self.springs.push(s);
        id as u64
    }

    fn add_torsion(&mut self, start_id: u64, end_id: u64, angle: f64) -> u64 {
        let id = self.springs.len();
        self.springs
            .push(Spring::new(start_id, end_id, angle, SpringKind::Torsion));
        id as u64
    }

    fn solve(&mut self, steps: u64) {
        self.print_state();
        for _ in 0..steps {
            self.step();
            self.print_state();
        }
    }

    fn step(&mut self) {
        let dt = 0.04;
        for point in self.points.iter_mut() {
            point.reset_forces();
        }
        for spring in self.springs.iter() {
            let point_a = &self.points[spring.start_id as usize];
            let point_b = &self.points[spring.end_id as usize];
            let forces = spring.compute_forces(point_a, point_b);

            self.points[spring.start_id as usize].fx += forces[0];
            self.points[spring.start_id as usize].fy += forces[1];
            self.points[spring.end_id as usize].fx += forces[2];
            self.points[spring.end_id as usize].fy += forces[3];
        }
        for point in self.points.iter_mut() {
            let ax = point.fx / point.m;
            let ay = point.fy / point.m;
            point.dx += ax;
            point.dy += ay;
            point.x += 0.5 * ax * dt * dt + point.dx * dt;
            point.y += 0.5 * ay * dt * dt + point.dy * dt;
        }
    }

    fn print_state(&self) {
        let mut data = vec![];
        for point in self.points.iter() {
            data.push(point.x);
            data.push(point.y);
            data.push(point.dx);
            data.push(point.dy);
            data.push(point.fx);
            data.push(point.fy);
        }
        let mut strings = data.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        println!("{}", strings.join(","));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2 {
    x: f64,
    y: f64,
    m: f64,
    dx: f64,
    dy: f64,
    fx: f64,
    fy: f64,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Point2 {
            x,
            y,
            m: 1.0,
            dx: 0.0,
            dy: 0.0,
            fx: 0.0,
            fy: 0.0,
        }
    }

    fn reset_forces(&mut self) {
        self.fx = 0.0;
        self.fy = 0.0;
    }

    fn step(&self, dt: f64) -> (f64, f64) {
        (self.x + self.dx * dt, self.y + self.dy * dt)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle2 {
    center: Point2,
    radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arc2 {
    center: Point2,
    start: Point2,
    end: Point2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line2 {
    start: Point2,
    end: Point2,
}
