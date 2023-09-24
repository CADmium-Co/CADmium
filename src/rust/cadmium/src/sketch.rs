use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sketch {
    points: HashMap<u64, Point2>,
    highest_point_id: u64,
    springs: HashMap<u64, Spring>,
    highest_spring_id: u64,
    line_segments: HashMap<u64, Line2>,
    highest_line_segment_id: u64,
    circles: HashMap<u64, Circle2>,
    highest_circle_id: u64,
    arcs: HashMap<u64, Arc2>,
    highest_arc_id: u64,
}

impl Sketch {
    pub fn new() -> Self {
        Sketch {
            points: HashMap::new(),
            highest_point_id: 0,
            springs: HashMap::new(),
            highest_spring_id: 0,
            line_segments: HashMap::new(),
            highest_line_segment_id: 0,
            circles: HashMap::new(),
            highest_circle_id: 0,
            arcs: HashMap::new(),
            highest_arc_id: 0,
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.highest_point_id + 1;
        self.points.insert(id, Point2::new(x, y));
        self.highest_point_id += 1;
        id
    }

    pub fn add_fixed_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.highest_point_id + 1;
        self.points.insert(id, Point2::new_fixed(x, y));
        self.highest_point_id += 1;
        id
    }

    pub fn add_circle(&mut self, x: f64, y: f64, radius: f64) -> u64 {
        let center_id = self.add_point(x, y);
        let c = Circle2 {
            center: center_id,
            radius,
        };
        let id = self.highest_circle_id + 1;
        self.circles.insert(id, c);
        self.highest_circle_id += 1;
        id
    }

    pub fn add_line_segment(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> u64 {
        let id0 = self.add_point(x0, y0);
        let id1 = self.add_point(x1, y1);
        let l = Line2 {
            start: id0,
            end: id1,
        };
        let id = self.highest_line_segment_id + 1;
        self.line_segments.insert(id, l);
        self.highest_line_segment_id += 1;
        id
    }

    pub fn add_spring(&mut self, start_id: u64, end_id: u64, length: f64) -> u64 {
        let id = self.highest_spring_id + 1;
        let s = Spring::new(start_id, end_id, length, SpringKind::Length);
        self.springs.insert(id, s);
        self.highest_spring_id += 1;
        id
    }

    pub fn add_torsion(&mut self, start_id: u64, end_id: u64, angle: f64) -> u64 {
        let id = self.highest_spring_id + 1;
        self.springs.insert(
            id,
            Spring::new(start_id, end_id, angle, SpringKind::Torsion),
        );
        self.highest_spring_id += 1;
        id
    }

    pub fn solve(&mut self, steps: u64) {
        // self.print_state();
        for _ in 0..steps {
            self.step();
            // self.print_state();
        }
    }

    pub fn step(&mut self) {
        let dt = 0.04;
        for (point_id, point) in self.points.iter_mut() {
            point.reset_forces();
        }
        for (spring_id, spring) in self.springs.iter() {
            let point_a = self.points.get(&spring.start_id).unwrap();
            let point_b = self.points.get(&spring.end_id).unwrap();
            let forces = spring.compute_forces(point_a, point_b);

            {
                let mut point_a = self.points.get_mut(&spring.start_id).unwrap();
                point_a.fx += forces[0];
                point_a.fy += forces[1];
            }
            {
                let mut point_b = self.points.get_mut(&spring.end_id).unwrap();
                point_b.fx += forces[2];
                point_b.fy += forces[3];
            }
        }
        for (point_id, point) in self.points.iter_mut() {
            if point.fixed {
                continue;
            }
            let ax = point.fx / point.m;
            let ay = point.fy / point.m;
            point.dx += ax;
            point.dy += ay;
            point.x += 0.5 * ax * dt * dt + point.dx * dt;
            point.y += 0.5 * ay * dt * dt + point.dy * dt;
        }
    }

    pub fn print_state_minimal(&self) {
        let mut data = vec![];
        for (point_id, point) in self.points.iter().sorted_by_key(|(id, _)| *id) {
            data.push(*point_id as f64);
            data.push(point.x);
            data.push(point.y);
        }
        let mut strings = data.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        println!("{}", strings.join(","));
    }

    pub fn print_state(&self) {
        let mut data = vec![];
        for (point_id, point) in self.points.iter() {
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

pub fn test_svg() {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new().set("viewBox", (0, 0, 70, 70)).add(path);

    svg::save("image.svg", &document).unwrap();
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
pub struct Point2 {
    x: f64,
    y: f64,
    m: f64,
    dx: f64,
    dy: f64,
    fx: f64,
    fy: f64,
    fixed: bool,
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
            fixed: false,
        }
    }

    pub fn new_fixed(x: f64, y: f64) -> Self {
        Point2 {
            x,
            y,
            m: 1.0,
            dx: 0.0,
            dy: 0.0,
            fx: 0.0,
            fy: 0.0,
            fixed: true,
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
    center: u64,
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
    start: u64,
    end: u64,
}
