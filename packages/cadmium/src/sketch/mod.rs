#![allow(unused)]
use geo::line_intersection::{line_intersection, LineIntersection};
use geo::Line;
use geo::{point, Contains};
use geo::{within, Intersects};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use truck_polymesh::stl::PolygonMeshStlFaceIterator;
use tsify::Tsify;

use core::panic;
use geo::LineString;
use geo::Polygon;
use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::f64::consts::{PI, TAU};
use std::hash::{Hash, Hasher};

use crate::archetypes::{Circle3, Plane};
use crate::error::CADmiumError;
use crate::project::{Project, RealSketch};

pub(crate) mod constraints;
mod intersections;
mod svg;

use crate::sketch::constraints::Constraint;

#[derive(strum::Display, Debug, Serialize, Deserialize)]
pub enum SketchFeatureType {
    Point,
    Line,
    Circle,
    Arc,
    Constraint,
}

#[serde_as]
#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Sketch {
    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub points: HashMap<u64, Point2>,
    pub highest_point_id: u64,

    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub line_segments: HashMap<u64, Line2>,
    pub highest_line_segment_id: u64,

    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub circles: HashMap<u64, Circle2>,
    pub highest_circle_id: u64,

    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub arcs: HashMap<u64, Arc2>,
    pub highest_arc_id: u64,

    #[serde_as(as = "HashMap<DisplayFromStr, _>")]
    pub constraints: HashMap<u64, Constraint>,
    pub highest_constraint_id: u64,
}

impl Sketch {
    pub fn new() -> Self {
        Sketch {
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
        }
    }

    pub fn from_faces(faces: &Vec<Face>, real_sketch: &RealSketch) -> Self {
        let mut new_sketch = Sketch::new();

        // println!("Creating a sketch just from faces");
        // for face in faces {
        //     println!("Face: {:?}", face);
        // }

        new_sketch.points = real_sketch.points_2d.clone();

        let mut circles: HashMap<String, Circle2> = HashMap::new();
        let mut line_segments: HashMap<String, Line2> = HashMap::new();
        let mut arcs: HashMap<String, Arc2> = HashMap::new();

        fn include_ring(
            ring: &Ring,
            circles: &mut HashMap<String, Circle2>,
            line_segments: &mut HashMap<String, Line2>,
            arcs: &mut HashMap<String, Arc2>,
        ) {
            match ring {
                Ring::Circle(circle) => {
                    let cs = circle.canonical_string();
                    let search_result = circles.get(&cs);
                    match search_result {
                        Some(existing_circle) => {
                            circles.remove(&cs);
                        }
                        None => {
                            circles.insert(cs.clone(), circle.clone());
                        }
                    }
                }
                Ring::Segments(segments) => {
                    for segment in segments {
                        match segment {
                            Segment::Line(line) => {
                                let cs = line.canonical_string();
                                let search_result = line_segments.get(&cs);

                                match search_result {
                                    Some(existing_line) => {
                                        line_segments.remove(&cs);
                                    }
                                    None => {
                                        line_segments.insert(cs.clone(), line.clone());
                                    }
                                }
                            }
                            Segment::Arc(arc) => {
                                let cs = arc.canonical_string();
                                let search_result = arcs.get(&cs);

                                match search_result {
                                    Some(existing_arc) => {
                                        arcs.remove(&cs);
                                    }
                                    None => {
                                        arcs.insert(cs.clone(), arc.clone());
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        for face in faces {
            include_ring(&face.exterior, &mut circles, &mut line_segments, &mut arcs);
            for ring in &face.holes {
                include_ring(ring, &mut circles, &mut line_segments, &mut arcs)
            }
        }

        for (index, circle) in circles.values().enumerate() {
            new_sketch.circles.insert(index as u64, circle.clone());
        }

        for (index, line) in line_segments.values().enumerate() {
            new_sketch.line_segments.insert(index as u64, line.clone());
        }

        for (index, arc) in arcs.values().enumerate() {
            new_sketch.arcs.insert(index as u64, arc.clone());
        }

        new_sketch
    }

    pub fn arc_angle(&self, arc: &Arc2) -> f64 {
        let center = self.points.get(&arc.center).unwrap();
        let start = self.points.get(&arc.start).unwrap();
        let end = self.points.get(&arc.end).unwrap();

        match arc.clockwise {
            false => angle(start, center, end),
            true => TAU - angle(start, center, end),
        }
    }

    pub fn arc_end_angle(&self, arc: &Arc2) -> f64 {
        let center = self.points.get(&arc.center).unwrap();
        let end = self.points.get(&arc.end).unwrap();

        let dx = end.x - center.x;
        let dy = end.y - center.y;

        if arc.clockwise {
            dy.atan2(dx) - PI / 2.0
        } else {
            dy.atan2(dx) + PI / 2.0
        }
    }

    pub fn arc_start_angle(&self, arc: &Arc2) -> f64 {
        let center = self.points.get(&arc.center).unwrap();
        let start = self.points.get(&arc.start).unwrap();

        let dx = start.x - center.x;
        let dy = start.y - center.y;

        if arc.clockwise {
            dy.atan2(dx) - PI / 2.0
        } else {
            dy.atan2(dx) + PI / 2.0
        }
    }

    pub fn line_start_angle(&self, line: &Line2) -> f64 {
        let start = self.points.get(&line.start).unwrap();
        let end = self.points.get(&line.end).unwrap();

        let dx = end.x - start.x;
        let dy = end.y - start.y;

        dy.atan2(dx)
    }

    pub fn line_end_angle(&self, line: &Line2) -> f64 {
        self.line_start_angle(line)
    }

    pub fn pretty_print_arc(&self, arc: &Arc2) {
        let center = self.points.get(&arc.center).unwrap();
        let start = self.points.get(&arc.start).unwrap();
        let end = self.points.get(&arc.end).unwrap();

        println!(
            "Arc: center: {}: ({}, {}), start: {}: ({}, {}), end: {}: ({}, {}) CW: {}",
            arc.center,
            center.x,
            center.y,
            arc.start,
            start.x,
            start.y,
            arc.end,
            end.x,
            end.y,
            arc.clockwise
        );
        println!("Start angle:\t{}", self.arc_start_angle(arc) * 180.0 / PI);
        println!("End angle:  \t{}", self.arc_end_angle(arc) * 180.0 / PI);
        println!("Angle:      \t{}", self.arc_angle(arc) * 180.0 / PI);
    }

    pub fn face_as_polygon(&self, face: &Face) -> Polygon {
        let binding = self.as_polygon(&face.exterior);
        let exterior = binding.exterior();

        let mut interiors: Vec<LineString<f64>> = vec![];
        for ring in &face.holes {
            interiors.push(self.as_polygon(ring).exterior().clone());
        }

        Polygon::new(exterior.clone(), interiors)
    }

    pub fn as_polygon(&self, ring: &Ring) -> Polygon {
        match ring {
            Ring::Circle(circle) => {
                let mut b: Vec<(f64, f64)> = vec![];
                let center = self.points.get(&circle.center).unwrap();

                let num_pts = 36;
                for i in 0..num_pts {
                    let angle = i as f64 / num_pts as f64 * TAU;
                    let x = center.x + circle.radius * angle.cos();
                    let y = center.y + circle.radius * angle.sin();
                    b.push((x, y));
                }

                let polygon = Polygon::new(LineString::from(b), vec![]);
                polygon
            }
            Ring::Segments(segments) => {
                let mut b: Vec<(f64, f64)> = vec![];
                for segment in segments {
                    match segment {
                        Segment::Line(line) => {
                            // we only ever push the start point. Imagine what happens for a closed
                            // square--the final closing segment is inferred.
                            let start = self.points.get(&segment.get_start()).unwrap();
                            b.push((start.x, start.y));
                        }
                        Segment::Arc(arc) => {
                            // similarly, we push all the points except the final one. The final
                            // segment is inferred.
                            let points = self.arc_to_points(arc);
                            for point in points {
                                b.push((point.x, point.y));
                            }
                            b.pop();
                        }
                    }
                }
                let polygon = Polygon::new(LineString::from(b), vec![]);
                polygon
            }
        }
    }

    pub fn arc_to_points(&self, arc: &Arc2) -> Vec<Point2> {
        // println!("An arc to points: {:?}", arc);
        let center = self.points.get(&arc.center).unwrap();
        let start = self.points.get(&arc.start).unwrap();
        let end = self.points.get(&arc.end).unwrap();
        let clockwise = arc.clockwise;

        arc_to_points(start, end, center, clockwise)
    }

    pub fn signed_area(&self, ring: &Ring) -> f64 {
        match ring {
            Ring::Circle(circle) => circle.radius * circle.radius * std::f64::consts::PI,
            Ring::Segments(segments) => {
                let mut area: f64 = 0.0;

                for segment in segments {
                    match segment {
                        Segment::Line(line) => {
                            let end = self.points.get(&segment.get_end()).unwrap();
                            let start = self.points.get(&segment.get_start()).unwrap();
                            area += (end.x - start.x) * (end.y + start.y);
                        }
                        Segment::Arc(arc) => {
                            let points = self.arc_to_points(arc);
                            for i in 0..points.len() - 1 {
                                let end = &points[i + 1];
                                let start = &points[i];
                                area += (end.x - start.x) * (end.y + start.y);
                            }
                        }
                    }
                }
                return area / -2.0;
            }
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.highest_point_id + 1;
        self.points.insert(id, Point2::new(x, y));
        self.highest_point_id += 1;
        id
    }

    pub fn add_hidden_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.highest_point_id + 1;
        self.points.insert(id, Point2::new_hidden(x, y));
        self.highest_point_id += 1;
        id
    }

    pub fn add_point_with_id(&mut self, x: f64, y: f64, id0: u64) -> Result<(), CADmiumError> {
        if self.points.contains_key(&id0) {
            return Err(CADmiumError::SketchFeatureAlreadyExists(
                SketchFeatureType::Point,
                id0,
            ));
        }
        if self.highest_point_id >= id0 {
            return Err(CADmiumError::SketchFeatureIDTooLow(
                SketchFeatureType::Point,
                id0,
            ));
        }
        self.points.insert(id0, Point2::new(x, y));
        self.highest_point_id = id0;
        Ok(())
    }

    pub fn add_fixed_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.highest_point_id + 1;
        self.points.insert(id, Point2::new_fixed(x, y));
        self.highest_point_id += 1;
        id
    }

    pub fn add_arc(&mut self, center_id: u64, start_id: u64, end_id: u64, clockwise: bool) -> u64 {
        let a = Arc2 {
            center: center_id,
            start: start_id,
            end: end_id,
            clockwise,
        };
        let id = self.highest_arc_id + 1;
        self.arcs.insert(id, a);
        self.highest_arc_id += 1;
        id
    }

    pub fn add_circle(&mut self, point_id: u64, radius: f64) -> u64 {
        let center_pt = self.points.get(&point_id).unwrap();
        let top = self.add_point(center_pt.x, center_pt.y + radius);
        let top_point = self.points.get_mut(&top).unwrap();
        top_point.hidden = true; // sneaky!
        let c = Circle2 {
            center: point_id,
            radius,
            top,
        };
        let id = self.highest_circle_id + 1;
        self.circles.insert(id, c);
        self.highest_circle_id += 1;
        id
    }

    pub fn add_circle_between_points(&mut self, center_id: u64, edge_id: u64) -> u64 {
        let center_pt = self.points.get(&center_id).unwrap();
        let edge_pt = self.points.get(&edge_id).unwrap();
        let radius = center_pt.distance_to(edge_pt);
        let c = Circle2 {
            center: center_id,
            radius,
            top: edge_id,
        };
        let id = self.highest_circle_id + 1;
        self.circles.insert(id, c);
        self.highest_circle_id += 1;
        id
    }

    pub fn add_rectangle_between_points(
        &mut self,
        start_id: u64,
        end_id: u64,
    ) -> (Vec<u64>, Vec<u64>) {
        let start = self.points.get(&start_id).unwrap();
        let end = self.points.get(&end_id).unwrap();

        let dx = end.x - start.x;
        let dy = end.y - start.y;

        let mut points = vec![];
        let mut segments = vec![];

        // create the two missing points
        let p0 = {
            let start_point = self.points.get(&start_id).unwrap();
            self.add_point(start_point.x + dx, start_point.y)
        };
        let p1 = {
            let start_point = self.points.get(&start_id).unwrap();
            self.add_point(start_point.x, start_point.y + dy)
        };

        points.push(p0);
        points.push(p1);

        let s0 = self.add_segment(start_id, p1);
        let s1 = self.add_segment(p1, end_id);
        let s2 = self.add_segment(end_id, p0);
        let s3 = self.add_segment(p0, start_id);

        segments.push(s0);
        segments.push(s1);
        segments.push(s2);
        segments.push(s3);

        (points, segments)
    }

    pub fn add_segment(&mut self, id0: u64, id1: u64) -> u64 {
        let l = Line2 {
            start: id0,
            end: id1,
        };
        let id = self.highest_line_segment_id + 1;
        self.line_segments.insert(id, l);
        self.highest_line_segment_id += 1;
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

    pub fn delete_circle(&mut self, id: u64) {
        let center_point_id = self.circles.get(&id).unwrap().center;
        let top_point_id = self.circles.get(&id).unwrap().top;
        let mut center_is_safe = false;
        let mut top_is_safe = false;

        for (line_id, line) in self.line_segments.iter() {
            if line.start == center_point_id || line.end == center_point_id {
                center_is_safe = true;
            }
            if line.start == top_point_id || line.end == top_point_id {
                top_is_safe = true;
            }
        }

        for (arc_id, arc) in self.arcs.iter() {
            if arc.start == center_point_id
                || arc.end == center_point_id
                || arc.center == center_point_id
            {
                center_is_safe = true;
            }
            if arc.start == top_point_id || arc.end == top_point_id || arc.center == top_point_id {
                top_is_safe = true;
            }
        }

        for (circle_id, circle) in self.circles.iter() {
            if *circle_id != id {
                if circle.center == center_point_id || circle.top == center_point_id {
                    center_is_safe = true;
                }
                if circle.center == top_point_id || circle.top == top_point_id {
                    top_is_safe = true;
                }
            }
        }

        if !center_is_safe {
            self.points.remove(&center_point_id);
        }
        if !top_is_safe {
            self.points.remove(&top_point_id);
        }

        self.circles.remove(&id);
    }

    pub fn delete_arc(&mut self, id: u64) {
        // TODO: return a result instead of crashing if the arc doesn't exist
        // TODO: remove any constraints that reference this arc
        let start_point_id = self.arcs.get(&id).unwrap().start;
        let end_point_id = self.arcs.get(&id).unwrap().end;
        let center_point_id = self.arcs.get(&id).unwrap().center;
        let mut start_is_safe = false;
        let mut end_is_safe = false;
        let mut center_is_safe = false;

        for (line_id, line) in self.line_segments.iter() {
            if line.start == start_point_id || line.end == start_point_id {
                start_is_safe = true;
            }
            if line.start == end_point_id || line.end == end_point_id {
                end_is_safe = true;
            }
            if line.start == center_point_id || line.end == center_point_id {
                center_is_safe = true;
            }
        }
        for (arc_id, arc) in self.arcs.iter() {
            if (*arc_id != id) {
                if arc.start == start_point_id
                    || arc.end == start_point_id
                    || arc.center == start_point_id
                {
                    start_is_safe = true;
                }
                if arc.start == end_point_id
                    || arc.end == end_point_id
                    || arc.center == end_point_id
                {
                    end_is_safe = true;
                }
                if arc.start == center_point_id
                    || arc.end == center_point_id
                    || arc.center == center_point_id
                {
                    center_is_safe = true;
                }
            }
        }
        for (circle_id, circle) in self.circles.iter() {
            if circle.center == start_point_id || circle.top == start_point_id {
                start_is_safe = true;
            }
            if circle.center == end_point_id || circle.top == end_point_id {
                end_is_safe = true;
            }
            if circle.center == center_point_id || circle.top == center_point_id {
                center_is_safe = true;
            }
        }
        if !start_is_safe {
            self.points.remove(&start_point_id);
        }
        if !end_is_safe {
            self.points.remove(&end_point_id);
        }
        if !center_is_safe {
            self.points.remove(&center_point_id);
        }

        self.arcs.remove(&id);
    }

    pub fn delete_line_segment(&mut self, id: u64) {
        // TODO: return a result instead of crashing if the line segment doesn't exist
        // TODO: remove any constraints that reference this line segment
        let start_point_id = self.line_segments.get(&id).unwrap().start;
        let end_point_id = self.line_segments.get(&id).unwrap().end;
        let mut start_is_safe = false;
        let mut end_is_safe = false;
        for (line_id, line) in self.line_segments.iter() {
            if *line_id != id {
                if line.start == start_point_id || line.end == start_point_id {
                    start_is_safe = true;
                }
                if line.start == end_point_id || line.end == end_point_id {
                    end_is_safe = true;
                }
            }
        }
        for (arc_id, arc) in self.arcs.iter() {
            if arc.start == start_point_id
                || arc.end == start_point_id
                || arc.center == start_point_id
            {
                start_is_safe = true;
            }
            if arc.start == end_point_id || arc.end == end_point_id || arc.center == end_point_id {
                end_is_safe = true;
            }
        }
        for (circle_id, circle) in self.circles.iter() {
            if circle.center == start_point_id || circle.top == start_point_id {
                start_is_safe = true;
            }
            if circle.center == end_point_id || circle.top == end_point_id {
                end_is_safe = true;
            }
        }
        if !start_is_safe {
            self.points.remove(&start_point_id);
        }
        if !end_is_safe {
            self.points.remove(&end_point_id);
        }

        self.line_segments.remove(&id);
    }

    pub fn add_line_with_id(
        &mut self,
        start_id: u64,
        end_id: u64,
        id: u64,
    ) -> Result<(), CADmiumError> {
        if self.line_segments.contains_key(&id) {
            return Err(CADmiumError::SketchFeatureAlreadyExists(
                SketchFeatureType::Line,
                id,
            ));
        }
        if self.highest_line_segment_id >= id {
            return Err(CADmiumError::SketchFeatureIDTooLow(
                SketchFeatureType::Line,
                id,
            ));
        }
        if !self.points.contains_key(&start_id) {
            return Err(CADmiumError::SketchFeatureMissingStart(
                SketchFeatureType::Line,
                id,
            ));
        }
        if !self.points.contains_key(&end_id) {
            return Err(CADmiumError::SketchFeatureMissingEnd(
                SketchFeatureType::Line,
                id,
            ));
        }

        let l = Line2 {
            start: start_id,
            end: end_id,
        };
        self.line_segments.insert(id, l);
        self.highest_line_segment_id = id;
        Ok(())
    }

    pub fn segment_length(&self, segment_id: u64) -> f64 {
        let segment = self.line_segments.get(&segment_id).unwrap();
        let start = self.points.get(&segment.start).unwrap();
        let end = self.points.get(&segment.end).unwrap();
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        dx.hypot(dy)
    }

    pub fn segment_angle(&self, segment_id: u64) -> f64 {
        let segment = self.line_segments.get(&segment_id).unwrap();
        let start = self.points.get(&segment.start).unwrap();
        let end = self.points.get(&segment.end).unwrap();
        start.angle_to(end)
    }

    fn apply_length_forces(
        &mut self,
        point_a_id: u64,
        point_b_id: u64,
        rest: f64,
        kp: f64,
        kd: f64,
    ) {
        let mut fx = 0.0;
        let mut fy = 0.0;
        let mut pa_hidden = false;
        let mut pb_hidden = false;
        {
            let point_a = self.points.get(&point_a_id).unwrap();
            let point_b = self.points.get(&point_b_id).unwrap();

            let dx = point_b.x - point_a.x;
            let dy = point_b.y - point_a.y;
            let dist = dx.hypot(dy);
            let err = dist - rest;

            let relative_dx = point_b.dx - point_a.dx;
            let relative_dy = point_b.dy - point_a.dy;

            // project the relative velocity onto the vector between the points
            // a is the velocity
            // b is the vector between the points
            // a dot b / |b|
            let closing_velocity = (relative_dx * dx + relative_dy * dy) / dist;

            let f = kp * err + kd * closing_velocity;
            fx = f * dx / dist;
            fy = f * dy / dist;

            pa_hidden = point_a.hidden;
            pb_hidden = point_b.hidden;
        }

        // if a point is hidden, it feels forces but does not exert them
        if !pa_hidden {
            let point_b = self.points.get_mut(&point_b_id).unwrap();
            point_b.fx -= fx;
            point_b.fy -= fy;
        }
        if !pb_hidden {
            let point_a = self.points.get_mut(&point_a_id).unwrap();
            point_a.fx += fx;
            point_a.fy += fy;
        }
    }

    fn apply_torsion_forces(
        &mut self,
        point_a_id: u64,
        point_b_id: u64,
        rest: f64,
        kp: f64,
        kd: f64,
    ) {
        let mut fx = 0.0;
        let mut fy = 0.0;
        {
            let point_a = self.points.get(&point_a_id).unwrap();
            let point_b = self.points.get(&point_b_id).unwrap();

            let dt = 0.01;

            let angle = (point_b.y - point_a.y).atan2(point_b.x - point_a.x);

            let mut err = rest - angle;
            // println!("Err: {}", err);
            if err > PI {
                err = err - PI * 2.0;
            }
            if err < -PI {
                err = err + PI * 2.0;
            }

            let point_a_stepped = point_a.step(dt);
            let point_b_stepped = point_b.step(dt);
            let angle_stepped = (point_b_stepped.1 - point_a_stepped.1)
                .atan2(point_b_stepped.0 - point_a_stepped.0);
            let mut angle_change = angle_stepped - angle;
            // println!("Dangle: {}", angle_change);

            if angle_change > PI {
                angle_change = angle_change - PI * 2.0;
            }
            if angle_change < -PI {
                angle_change = angle_change + PI * 2.0;
            }

            let d_angle = angle_change / dt;
            let torque = kp * err - kd * d_angle;

            let dx = point_b.x - point_a.x;
            let dy = point_b.y - point_a.y;
            let dist = dx.hypot(dy);

            let f_mag = torque / dist;

            fx = f_mag * dy;
            fy = -f_mag * dx;
        }

        let point_a = self.points.get_mut(&point_a_id).unwrap();
        point_a.fx += fx;
        point_a.fy += fy;

        let point_b = self.points.get_mut(&point_b_id).unwrap();
        point_b.fx -= fx;
        point_b.fy -= fy;
    }

    pub fn solve(&mut self, steps: u64) -> bool {
        let tolerance = 1e-12;

        for _ in 0..steps {
            let retval = self.take_a_step();
            if retval < tolerance {
                return true;
            }
        }
        return false;
    }

    pub fn apply_forces(&mut self, constraint_id: u64) {
        let constraint = self.constraints.get(&constraint_id).unwrap().clone();

        match constraint {
            Constraint::SegmentsEqual {
                segment_a_id,
                segment_b_id,
                kp,
                kd,
                ..
            } => {
                let a = self.line_segments.get(&segment_a_id).unwrap();
                let b = self.line_segments.get(&segment_b_id).unwrap();

                // TODO: is there a better way to satisfy the borrow checker?
                let mut average_length = 0.0;
                let mut a_start = 0;
                let mut b_start = 0;
                let mut a_end = 0;
                let mut b_end = 0;
                {
                    average_length = (self.segment_length(segment_a_id)
                        + self.segment_length(segment_b_id))
                        / 2.0;
                    a_start = a.start;
                    b_start = b.start;
                    a_end = a.end;
                    b_end = b.end;
                }
                self.apply_length_forces(a_start, a_end, average_length, kp, kd);
                self.apply_length_forces(b_start, b_end, average_length, kp, kd);
            }
            Constraint::SegmentLength {
                segment_id,
                length,
                kp,
                kd,
                ..
            } => {
                let segment = self.line_segments.get(&segment_id).unwrap();
                self.apply_length_forces(segment.start, segment.end, length, kp, kd)
            }
            Constraint::CircleDiameter {
                circle_id,
                diameter,
                kp,
                kd,
                ..
            } => {
                let circle = self.circles.get(&circle_id).unwrap();
                let center = self.points.get(&circle.center).unwrap();
                let top = self.points.get(&circle.top).unwrap();
                let radius = center.distance_to(top);

                self.apply_length_forces(circle.center, circle.top, diameter / 2.0, kp, kd)
            }
            Constraint::SegmentAngle {
                segment_id,
                angle,
                kp,
                kd,
                ..
            } => {
                let segment = self.line_segments.get(&segment_id).unwrap();
                self.apply_torsion_forces(segment.start, segment.end, angle, kp, kd);
            }
        }
    }

    pub fn take_a_step(&mut self) -> f64 {
        let dt = 0.02; // at 0.04 the system can be unstable! especially manual_rectangle()
                       // TODO: switch to RK4?
        let mut biggest_change = 0.0;
        for (_point_id, point) in self.points.iter_mut() {
            point.reset_forces();
        }

        let constraint_keys = self
            .constraints
            .keys()
            .sorted()
            .map(|k| k.clone())
            .collect::<Vec<_>>();
        for constraint_id in constraint_keys {
            self.apply_forces(constraint_id);
        }

        for point in self.points.values_mut() {
            point.apply_drag_force();
        }

        for (point_id, point) in self.points.iter_mut() {
            if point.fixed {
                continue;
            }
            let ax = point.fx / point.m;
            let ay = point.fy / point.m;
            point.dx += ax;
            point.dy += ay;
            let delta_x = 0.5 * ax * dt * dt + point.dx * dt;
            let delta_y = 0.5 * ay * dt * dt + point.dy * dt;

            if delta_x.abs() > biggest_change {
                biggest_change = delta_x.abs();
            }
            if delta_y.abs() > biggest_change {
                biggest_change = delta_y.abs();
            }

            point.x += delta_x;
            point.y += delta_y;
        }

        // update any circles whose radii might have changed!
        for (_circle_id, circle) in self.circles.iter_mut() {
            let center = self.points.get(&circle.center).unwrap();
            let top = self.points.get(&circle.top).unwrap();
            circle.radius = center.distance_to(top);
        }

        biggest_change
    }

    pub fn print_state_minimal(&self) {
        let mut data = vec![];
        for (point_id, point) in self.points.iter().sorted_by_key(|(id, _)| *id) {
            data.push(*point_id as f64);
            data.push(point.x);
            data.push(point.y);
        }
        let strings = data.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        println!("{}", strings.join(","));
    }

    pub fn print_state(&self) {
        let mut data = vec![];
        for (_point_id, point) in self.points.iter() {
            data.push(point.x);
            data.push(point.y);
            data.push(point.dx);
            data.push(point.dy);
            data.push(point.fx);
            data.push(point.fy);
        }
        let strings = data.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        println!("{}", strings.join(","));
    }

    pub fn find_faces(&self) -> (Vec<Face>, Vec<Segment>) {
        let mut segments_overall: Vec<Segment> = vec![];

        for line_id in self.line_segments.keys().sorted() {
            let line = self.line_segments.get(line_id).unwrap();
            segments_overall.push(Segment::Line(line.clone()));
        }
        for arc_id in self.arcs.keys().sorted() {
            let arc = self.arcs.get(arc_id).unwrap();
            segments_overall.push(Segment::Arc(arc.clone()));
        }

        let (rings, unused_segments) = self.find_rings(segments_overall, false);
        // println!("Found {} rings", rings.len());
        // for ring in &rings {
        //     println!("{:?}", ring);
        // }
        // println!("Found {} unused segments", unused_segments.len());
        let mut faces: Vec<Face> = rings.iter().map(|r| Face::from_ring(r)).collect();

        if rings.len() == 0 {
            return (faces, unused_segments);
        }

        // this next block of code converts everything to Polygons just so we can
        // determine what faces contain which other faces. It's a bit of a waste
        // because geo is a relatively heavy dependency and we don't need all of it
        let polygons: Vec<Polygon> = rings.iter().map(|r| self.as_polygon(r)).collect();
        // they are already sorted from smallest to largest area - self.find_rings does this
        let mut what_contains_what: Vec<(usize, usize)> = vec![];

        for smaller_polygon_index in 0..polygons.len() - 1 {
            let smaller_polygon = &polygons[smaller_polygon_index];

            for bigger_polygon_index in smaller_polygon_index + 1..polygons.len() {
                let bigger_polygon = &polygons[bigger_polygon_index];
                let inside = bigger_polygon.contains(smaller_polygon);

                if inside {
                    what_contains_what.push((bigger_polygon_index, smaller_polygon_index));
                    break;
                }
            }
        }

        // cool, now we know what faces contain which other faces. Let's just add the holes
        for (bigger_index, smaller_index) in what_contains_what {
            let smaller_face = &faces[smaller_index].clone();
            faces[bigger_index].add_hole(smaller_face)
        }

        // let faces: Vec<Face> = polygons.iter().map(|p| Face::from_polygon(p)).collect();
        (faces, unused_segments)
    }

    pub fn find_rings(&self, segments: Vec<Segment>, debug: bool) -> (Vec<Ring>, Vec<Segment>) {
        // We are handed all of the segments to consider
        let mut segments_overall = segments.clone();
        let num_segments = segments_overall.len();

        // Let's double it by reversing each one and adding it to the list of
        // segments to consider
        let segments_reversed: Vec<Segment> =
            segments_overall.iter().map(|s| s.reverse()).collect();
        segments_overall.extend(segments_reversed);

        // keep track of every index we've already used--each segment can only be used once
        let mut used_indices: Vec<usize> = vec![];
        // this is the output variable
        let mut new_rings: Vec<Vec<usize>> = vec![];

        for (seg_idx, s) in segments_overall.iter().enumerate() {
            if debug {
                // println!("Starting a loop with segment: {:?}", s);
                match s {
                    Segment::Line(line) => {
                        println!(
                            "Line: ({}, {}) to ({}, {})",
                            self.points.get(&line.start).unwrap().x,
                            self.points.get(&line.start).unwrap().y,
                            self.points.get(&line.end).unwrap().x,
                            self.points.get(&line.end).unwrap().y
                        );
                    }
                    Segment::Arc(arc) => {
                        println!(
                            "Arc: center: ({}, {}), start: ({}, {}), end: ({}, {})",
                            self.points.get(&arc.center).unwrap().x,
                            self.points.get(&arc.center).unwrap().y,
                            self.points.get(&arc.start).unwrap().x,
                            self.points.get(&arc.start).unwrap().y,
                            self.points.get(&arc.end).unwrap().x,
                            self.points.get(&arc.end).unwrap().y
                        );
                    }
                }
            }
            if used_indices.contains(&seg_idx) {
                if debug {
                    println!("Skipping because it's been used");
                }
                continue;
            }
            // this variable will accumulate the indices of our new ring
            let mut new_ring_indices: Vec<usize> = vec![];
            let starting_point = s.get_start();
            if debug {
                println!("Starting point: {:?}", starting_point);
            }

            let mut next_segment_index: usize = seg_idx;
            for _i in 1..segments_overall.len() {
                let next_segment = segments_overall.get(next_segment_index).unwrap();
                if debug {
                    println!("next segment: {:?}", next_segment);
                }
                new_ring_indices.push(next_segment_index);

                match self.find_next_segment_index(
                    &segments_overall,
                    next_segment,
                    &used_indices,
                    debug,
                ) {
                    None => {
                        if debug {
                            println!("\tno viable next segments!");
                        }
                        break;
                    }
                    Some(idx) => next_segment_index = idx,
                }
                if next_segment.get_end() == starting_point {
                    if debug {
                        println!("\tomg finished!");
                        println!("\tring indices: {:?}", new_ring_indices);
                    }
                    new_rings.push(new_ring_indices.clone());
                    used_indices.extend(new_ring_indices);
                    break;
                }
            }
        }

        let used_indices_set = used_indices.iter().cloned().collect::<HashSet<_>>();
        let all_indices_set = (0..segments_overall.len()).collect::<HashSet<_>>();

        let unused_indices_set = all_indices_set
            .difference(&used_indices_set)
            .collect::<HashSet<_>>();
        let unused_indices = unused_indices_set
            .iter()
            .cloned()
            .filter(|idx| return *idx < &num_segments)
            .collect::<Vec<_>>();
        let unused_segments = unused_indices
            .iter()
            .cloned()
            .map(|idx| segments_overall.get(*idx).unwrap().clone())
            .collect::<Vec<_>>();

        let mut all_rings: Vec<Ring> = vec![];
        for ring_indices in new_rings.iter() {
            // let mut this_ring: Ring = Ring::Segments(vec![]);
            let mut this_ring: Vec<Segment> = vec![];
            for segment_index in ring_indices {
                let actual_segment: &Segment = segments_overall.get(*segment_index).unwrap();
                this_ring.push(actual_segment.clone());
            }
            all_rings.push(Ring::Segments(this_ring));
        }

        // println!("--Found {} rings", all_rings.len());

        // Circles are trivially rings!
        for (_circle_id, circle) in self.circles.iter() {
            all_rings.push(Ring::Circle(circle.clone()));
        }

        all_rings.sort_by(|r1, r2| {
            // TODO: implement signed_area for a ring which is made of arcs
            self.signed_area(r1)
                .partial_cmp(&self.signed_area(r2))
                .unwrap()
        });

        // filter out to only the positive-valued ones
        all_rings = all_rings
            .iter()
            .filter(|r| self.signed_area(r) > 0.0)
            .cloned()
            .collect();

        // println!("--Found {} rings", all_rings.len());

        (all_rings, unused_segments)
    }

    pub fn find_next_segment_index(
        &self,
        segments: &Vec<Segment>,
        starting_segment: &Segment,
        used_indices: &Vec<usize>,
        debug: bool,
    ) -> Option<usize> {
        // println!("Finding next segment index");
        let mut matches: Vec<(usize, f64, f64)> = vec![];
        let mut this_segment_end_angle = match starting_segment {
            Segment::Line(line) => self.line_end_angle(line),
            Segment::Arc(arc) => self.arc_end_angle(arc),
        };
        this_segment_end_angle = (this_segment_end_angle + PI) % (2.0 * PI);

        for (idx, s2) in segments.iter().enumerate() {
            if used_indices.contains(&idx) {
                continue;
            }
            if s2.continues(&starting_segment) && !s2.equals_or_reverse_equals(&starting_segment) {
                let starting_angle = match s2 {
                    Segment::Line(line) => self.line_start_angle(line),
                    Segment::Arc(arc) => self.arc_start_angle(arc),
                };
                let angle_diff = angle_difference(this_segment_end_angle, starting_angle);
                matches.push((idx, starting_angle, angle_diff));
                // angle_diff measures how hard you'd have to turn left to continue the path from
                // starting_segment to s2, where a straight line would be 180, a left turn 270, a right turn 90.
                // This is important later because to make the smallest loops possible, we always want to be
                // turning left as hard as possible when finding rings.
            }
        }

        if matches.len() == 0 {
            None
        } else if matches.len() == 1 {
            Some(matches[0].0)
        } else {
            if debug {
                println!("\tMultiple options! Deciding which one to take...");
            }

            let mut best_option = 0;
            let mut hardest_left_turn = 0.0;
            for o in matches.iter() {
                // println!("Option: {:?}", segments.get(o.0).unwrap());
                // println!("Option: {} angle {}", o.0, o.1 * 180.0 / PI);
                // println!("Option: {}", o.2 * 180.0 / PI);
                // println!();

                if o.2 > hardest_left_turn {
                    hardest_left_turn = o.2;
                    best_option = o.0;
                }
            }
            // println!("Best option: {}", best_option);
            Some(best_option)
        }
    }

    pub fn circle_intersection(&self, circle_a: &Circle2, circle_b: &Circle2) -> Vec<Point2> {
        // See https://math.stackexchange.com/questions/256100/how-can-i-find-the-points-at-which-two-circles-intersect#comment4306998_1367732
        // See https://gist.github.com/jupdike/bfe5eb23d1c395d8a0a1a4ddd94882ac
        let center_a = self.points.get(&circle_a.center).unwrap();
        let center_b = self.points.get(&circle_b.center).unwrap();
        let r_a = circle_a.radius;
        let r_b = circle_b.radius;

        let center_dx = center_b.x - center_a.x;
        let center_dy = center_b.y - center_a.y;
        let center_dist = center_dx.hypot(center_dy);

        if !(center_dist <= r_a + r_b && center_dist >= r_a - r_b) {
            return vec![];
        }

        let r_2 = center_dist * center_dist;
        let r_4 = r_2 * r_2;
        let a = (r_a * r_a - r_b * r_b) / (2.0 * r_2);
        let r_2_r_2 = r_a * r_a - r_b * r_b;
        let c = (2.0 * (r_a * r_a + r_b * r_b) / r_2 - r_2_r_2 * r_2_r_2 / r_4 - 1.0).sqrt();

        let fx = (center_a.x + center_b.x) / 2.0 + a * (center_b.x - center_a.x);
        let gx = c * (center_b.y - center_a.y) / 2.0;
        let ix1 = fx + gx;
        let ix2 = fx - gx;

        let fy = (center_a.y + center_b.y) / 2.0 + a * (center_b.y - center_a.y);
        let gy = c * (center_a.x - center_b.x) / 2.0;
        let iy1 = fy + gy;
        let iy2 = fy - gy;

        vec![Point2::new(ix1, iy1), Point2::new(ix2, iy2)]
    }

    pub fn line_intersection(&self, line_a: &Line2, line_b: &Line2) -> Option<Point2> {
        let start_a = self.points.get(&line_a.start).unwrap();
        let end_a = self.points.get(&line_a.end).unwrap();
        let start_b = self.points.get(&line_b.start).unwrap();
        let end_b = self.points.get(&line_b.end).unwrap();

        let line_a = Line::new(
            geo::Coord {
                x: start_a.x,
                y: start_a.y,
            },
            geo::Coord {
                x: end_a.x,
                y: end_a.y,
            },
        );
        let line_b = Line::new(
            geo::Coord {
                x: start_b.x,
                y: start_b.y,
            },
            geo::Coord {
                x: end_b.x,
                y: end_b.y,
            },
        );

        let intersection = line_intersection(line_a, line_b);

        match intersection {
            Some(line_intersection) => match line_intersection {
                LineIntersection::SinglePoint {
                    intersection,
                    is_proper,
                } => Some(Point2::new(intersection.x, intersection.y)),
                LineIntersection::Collinear { intersection } => panic!("Collinear!"),
            },
            None => None,
        }
    }
}

pub fn arc_to_points(
    start: &Point2,
    end: &Point2,
    center: &Point2,
    clockwise: bool,
) -> Vec<Point2> {
    let r = (center.x - start.x).hypot(center.y - start.y);
    let circle_tolerance: f64 = 0.001; // in meters
    let k = circle_tolerance / r;
    let mut n = (PI / (2.0 * k).sqrt()).ceil() as i64;

    let segment_angle = (2.0 * PI) / n as f64;
    let segment_length = r * segment_angle;
    let start_angle = (start.y - center.y).atan2(start.x - center.x);

    let mut line_vertices: Vec<Point2> = vec![];
    line_vertices.push(Point2::new(start.x, start.y));

    if clockwise {
        n = -n;
    }

    for i in 1..n.abs() {
        let theta = ((2.0 * PI) / n as f64) * i as f64 + start_angle;
        let x_component = r * theta.cos();
        let y_component = r * theta.sin();
        let point = Point2::new(x_component + center.x, y_component + center.y);
        line_vertices.push(point.clone());

        let distance_to_end = point.distance_to(end);
        if (distance_to_end <= segment_length) {
            line_vertices.push(Point2::new(end.x, end.y));
            break;
        }
    }

    line_vertices
}

pub struct IncrementingMap<T> {
    pub items: IndexMap<u64, T>,
    next_id: u64,
}

impl<T> IncrementingMap<T> {
    pub fn new() -> Self {
        IncrementingMap {
            items: IndexMap::new(),
            next_id: 0,
        }
    }

    pub fn add_item(&mut self, item: T) -> u64 {
        let id = self.next_id;
        self.items.insert(id, item);
        self.next_id += 1;
        id
    }

    pub fn remove_item(&mut self, id: u64) -> u64 {
        self.items.remove(&id);
        id
    }

    pub fn get_item(&self, id: u64) -> Option<&T> {
        self.items.get(&id)
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
    m: f64,
    dx: f64,
    dy: f64,
    fx: f64,
    fy: f64,
    fixed: bool,
    pub hidden: bool,
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
            hidden: false,
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
            hidden: false,
        }
    }

    pub fn new_hidden(x: f64, y: f64) -> Self {
        Point2 {
            x,
            y,
            m: 1.0,
            dx: 0.0,
            dy: 0.0,
            fx: 0.0,
            fy: 0.0,
            fixed: false,
            hidden: true,
        }
    }

    fn reset_forces(&mut self) {
        self.fx = 0.0;
        self.fy = 0.0;
    }

    pub fn apply_drag_force(&mut self) {
        let drag_coefficient = 0.1;
        let drag_force = -drag_coefficient * self.dx;
        self.fx += drag_force;
        let drag_force = -drag_coefficient * self.dy;
        self.fy += drag_force;
    }

    fn step(&self, dt: f64) -> (f64, f64) {
        (self.x + self.dx * dt, self.y + self.dy * dt)
    }

    fn distance_to(&self, other: &Point2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx.hypot(dy)
    }

    fn angle_to(&self, other: &Point2) -> f64 {
        (other.y - self.y).atan2(other.x - self.x)
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Circle2 {
    pub center: u64,
    pub radius: f64,
    pub top: u64,
}

impl Circle2 {
    pub fn equals(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }

    pub fn canonical_string(&self) -> String {
        format!("{}-{}", self.center, self.radius)
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Arc2 {
    pub center: u64,
    pub start: u64,
    pub end: u64,
    pub clockwise: bool,
}

impl Arc2 {
    pub fn reverse(&self) -> Self {
        Arc2 {
            center: self.center,
            start: self.end,
            end: self.start,
            clockwise: !self.clockwise,
        }
    }

    pub fn canonical_string(&self) -> String {
        if self.start < self.end {
            format!(
                "{}-{}-{}-{}",
                self.start, self.end, self.center, self.clockwise
            )
        } else {
            self.reverse().canonical_string()
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Line2 {
    pub start: u64,
    pub end: u64,
}

impl Line2 {
    pub fn reverse(&self) -> Self {
        Line2 {
            start: self.end,
            end: self.start,
        }
    }

    pub fn canonical_string(&self) -> String {
        if self.start < self.end {
            format!("{}-{}", self.start, self.end)
        } else {
            format!("{}-{}", self.end, self.start)
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Segment {
    Line(Line2),
    Arc(Arc2),
}

impl Segment {
    pub fn reverse(&self) -> Self {
        match self {
            Segment::Line(line) => Segment::Line(line.reverse()),
            Segment::Arc(arc) => Segment::Arc(arc.reverse()),
        }
    }

    pub fn get_start(&self) -> u64 {
        match self {
            Segment::Line(line) => line.start,
            Segment::Arc(arc) => arc.start,
        }
    }

    pub fn get_end(&self) -> u64 {
        match self {
            Segment::Line(line) => line.end,
            Segment::Arc(arc) => arc.end,
        }
    }

    pub fn continues(&self, prior_segment: &Segment) -> bool {
        // determines if this segment continues the prior segment
        prior_segment.get_end() == self.get_start()
    }

    pub fn equals_or_reverse_equals(&self, other: &Self) -> bool {
        self == other || self == &other.reverse()
    }

    pub fn reverse_equals(&self, other: &Self) -> bool {
        self == &other.reverse()
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Ring {
    Circle(Circle2),
    Segments(Vec<Segment>),
}

impl Ring {
    pub fn adjacent_edges(&self, other: &Self) -> Option<(Vec<usize>, Vec<usize>)> {
        match (self, other) {
            (Ring::Segments(segments_a), Ring::Segments(segments_b)) => {
                let mut edge_indices_a: Vec<usize> = vec![];
                let mut edge_indices_b: Vec<usize> = vec![];
                for (index_a, segment_a) in segments_a.iter().enumerate() {
                    for (index_b, segment_b) in segments_b.iter().enumerate() {
                        if segment_a.reverse_equals(segment_b) {
                            edge_indices_a.push(index_a);
                            edge_indices_b.push(index_b);
                        }
                    }
                }
                if edge_indices_a.len() == 0 {
                    return None;
                } else {
                    Some((edge_indices_a, edge_indices_b))
                }
            }
            _ => None,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Ring::Circle(circle_a), Ring::Circle(circle_b)) => circle_a.equals(circle_b),
            (Ring::Segments(segments_a), Ring::Segments(segments_b)) => {
                segments_a.len() == segments_b.len()
                    && segments_a
                        .iter()
                        .zip(segments_b.iter())
                        .all(|(a, b)| a == b)
            }
            _ => false,
        }
    }

    pub fn canonical_form(&self) -> Self {
        // sort the segments in order by first finding the segment with the smallest start point
        // and then rotating the list so that that segment is first
        match self {
            Ring::Circle(circle) => Ring::Circle(circle.clone()),
            Ring::Segments(segments) => {
                let mut canonical_segments: Vec<Segment> = vec![];
                let mut min_index = 0;
                let mut min_segment = segments.get(0).unwrap();
                for (i, segment) in segments.iter().enumerate() {
                    if segment.get_start() < min_segment.get_start() {
                        min_index = i;
                        min_segment = segment;
                    }
                }

                for i in 0..segments.len() {
                    canonical_segments.push(
                        segments
                            .get((i + min_index) % segments.len())
                            .unwrap()
                            .clone(),
                    );
                }

                Ring::Segments(canonical_segments)
            }
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Ring::Circle(circle) => Ring::Circle(circle.clone()),
            Ring::Segments(segments) => {
                let mut reversed_segments: Vec<Segment> = vec![];
                for segment in segments.iter().rev() {
                    reversed_segments.push(segment.reverse());
                }
                Ring::Segments(reversed_segments)
            }
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Face {
    pub exterior: Ring,
    pub holes: Vec<Ring>,
}

impl Face {
    pub fn from_ring(ring: &Ring) -> Face {
        Face {
            exterior: ring.clone(),
            holes: vec![],
        }
    }

    pub fn add_hole(&mut self, hole: &Face) {
        self.holes.push(hole.exterior.clone());
    }
}

pub fn angle(a: &Point2, b: &Point2, c: &Point2) -> f64 {
    // output range is (0, 2*PI]
    let ba_dx: f64 = a.x - b.x;
    let ba_dy: f64 = a.y - b.y;
    let ba_angle: f64 = ba_dy.atan2(ba_dx);

    let bc_dx = c.x - b.x;
    let bc_dy = c.y - b.y;
    let bc_angle = bc_dy.atan2(bc_dx);

    let mut naive_angle = bc_angle - ba_angle;
    if naive_angle <= 0.0 {
        naive_angle += TAU;
    }
    naive_angle
}

pub fn min_angle_diff(a0: f64, a1: f64) -> f64 {
    let path_a = angle_difference(a0, a1);
    let path_b = angle_difference(a1, a0);
    if path_a < path_b {
        path_a
    } else {
        path_b
    }
}

pub fn angle_difference(mut a0: f64, mut a1: f64) -> f64 {
    if a0 > TAU {
        a0 -= TAU;
    }
    if a0 < 0.0 {
        a0 += TAU;
    }

    if a1 > TAU {
        a1 -= TAU;
    }
    if a1 < 0.0 {
        a1 += TAU;
    }

    let mut naive_diff = a1 - a0;
    if naive_diff > TAU {
        naive_diff -= TAU;
    }
    if naive_diff < 0.0 {
        naive_diff += TAU;
    }

    naive_diff
}

#[cfg(test)]
mod tests {
    use crate::project::Project;

    use super::*;

    #[test]
    fn arc_to_points_90() {
        let mut sketch = Sketch::new();
        let center = sketch.add_point(0.0, 0.0);
        let start = sketch.add_point(1.0, 0.0);
        let end = sketch.add_point(0.0, 1.0);
        let arc_id = sketch.add_arc(center, start, end, false);
        let arc = sketch.arcs.get(&arc_id).unwrap();

        let points = sketch.arc_to_points(&arc);
        assert_eq!(points.len(), 19);
    }

    #[test]
    fn arc_to_points_neg_90() {
        let mut sketch = Sketch::new();
        let center = sketch.add_point(0.0, 0.0);
        let start = sketch.add_point(0.0, 1.0);
        let end = sketch.add_point(1.0, 0.0);
        let arc_id = sketch.add_arc(center, start, end, true);
        let arc = sketch.arcs.get(&arc_id).unwrap();

        let points = sketch.arc_to_points(&arc);
        assert_eq!(points.len(), 19);

        for point in points {
            println!("Point: ({}, {})", point.x, point.y);
        }
    }

    #[test]
    fn arc_to_points_180() {
        let mut sketch = Sketch::new();
        let center = sketch.add_point(0.0, 0.0);
        let start = sketch.add_point(1.0, 0.0);
        let end = sketch.add_point(-1.0, 0.0);
        let arc_id = sketch.add_arc(center, start, end, false);
        let arc = sketch.arcs.get(&arc_id).unwrap();

        let points = sketch.arc_to_points(&arc);
        assert_eq!(points.len(), 37);
    }

    #[test]
    fn arc_to_points70() {
        let mut sketch = Sketch::new();
        let center = sketch.add_point(0.0, 0.0);
        let start = sketch.add_point(1.0, 0.0);
        let end = sketch.add_point(0.0, -1.0);
        let arc_id = sketch.add_arc(center, start, end, false);
        let arc = sketch.arcs.get(&arc_id).unwrap();

        let points = sketch.arc_to_points(&arc);
        assert_eq!(points.len(), 55);
    }

    #[test]
    fn delete_lines() {
        let mut sketch = Sketch::new();

        let a = sketch.add_fixed_point(0.0, 0.0);
        let b = sketch.add_point(1.0, -0.1);
        let c = sketch.add_point(1.1, 0.9);
        let d = sketch.add_point(-0.1, 0.9);

        let segment_ab = sketch.add_segment(a, b);
        let segment_bc = sketch.add_segment(b, c);
        let segment_cd = sketch.add_segment(c, d);
        let segment_da = sketch.add_segment(d, a);

        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 4);

        sketch.delete_line_segment(segment_ab);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 4);

        sketch.delete_line_segment(segment_bc);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 3);

        sketch.delete_line_segment(segment_cd);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 2);

        sketch.delete_line_segment(segment_da);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 0);
    }

    #[test]
    fn delete_arcs() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(1.0, 0.0);
        let b = sketch.add_point(2.0, 1.0);
        let c = sketch.add_point(1.0, 2.0);
        let d = sketch.add_point(0.0, 1.0);
        let center = sketch.add_point(1.0, 1.0);

        let arc_ab = sketch.add_arc(center, a, b, false);
        let arc_bc = sketch.add_arc(center, b, c, false);
        let arc_cd = sketch.add_arc(center, c, d, false);
        let arc_da = sketch.add_arc(center, d, a, false);

        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 5);

        sketch.delete_arc(arc_ab);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 5);

        sketch.delete_arc(arc_bc);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 4);

        sketch.delete_arc(arc_cd);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 3);

        sketch.delete_arc(arc_da);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 0);
    }

    #[test]
    fn delete_circles() {
        let mut sketch = Sketch::new();

        let center = sketch.add_point(1.0, 1.0);
        let circle_a = sketch.add_circle(center, 1.0);
        let circle_b = sketch.add_circle(center, 2.0);

        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 3);

        sketch.delete_circle(circle_a);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 2);

        sketch.delete_circle(circle_b);
        println!("points: {:?}", sketch.points.len());
        assert_eq!(sketch.points.len(), 0);
    }
}
