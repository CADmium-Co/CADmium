#![allow(unused)]
use geo::line_intersection::{line_intersection, LineIntersection};
use geo::Contains;
use geo::Intersects;
use geo::Line;

use core::panic;
use geo::LineString;
use geo::Polygon;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::f64::consts::{PI, TAU};
use std::sync::Arc;
use svg::node::element::path::Data;
// use svg::node::element::Circle;
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
    virtual_points: HashMap<u64, Point2>,
    highest_virtual_point_id: u64,
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
            virtual_points: HashMap::new(),
            highest_virtual_point_id: 0,
        }
    }

    pub fn arc_angle(&self, arc: &Arc2) -> f64 {
        let center = self.points.get(&arc.center).unwrap();
        let start = self.points.get(&arc.start).unwrap();
        let end = self.points.get(&arc.end).unwrap();

        angle(start, center, end)
    }

    pub fn as_polygon(&self, ring: &Ring) -> Polygon {
        match ring {
            Ring::Circle(circle) => {
                let mut b: Vec<(f64, f64)> = vec![];
                let center = self.points.get(&circle.center).unwrap();

                let num_pts = 10;
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
                // TODO: figure out how to better support arcs here! At least
                // approximate them with a few line segments!
                for segment in segments {
                    let start = segment.get_start();
                    let start_pt = self.points.get(&start).unwrap();
                    let start_tuple = (start_pt.x, start_pt.y);
                    b.push(start_tuple);
                }
                let polygon = Polygon::new(LineString::from(b), vec![]);
                polygon
            }
        }
    }

    pub fn signed_area(&self, ring: &Ring) -> f64 {
        match ring {
            Ring::Circle(circle) => circle.radius * circle.radius * std::f64::consts::PI,
            Ring::Segments(segments) => {
                let mut area: f64 = 0.0;
                for segment in segments {
                    let end = self.points.get(&segment.get_end()).unwrap();
                    let start = self.points.get(&segment.get_start()).unwrap();
                    area += (end.x - start.x) * (end.y + start.y);
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

    pub fn add_fixed_point(&mut self, x: f64, y: f64) -> u64 {
        let id = self.highest_point_id + 1;
        self.points.insert(id, Point2::new_fixed(x, y));
        self.highest_point_id += 1;
        id
    }

    pub fn add_arc(&mut self, center_id: u64, start_id: u64, end_id: u64) -> u64 {
        let a = Arc2 {
            center: center_id,
            start: start_id,
            end: end_id,
        };
        let id = self.highest_arc_id + 1;
        self.arcs.insert(id, a);
        self.highest_arc_id += 1;
        id
    }

    pub fn add_circle(&mut self, point_id: u64, radius: f64) -> u64 {
        let c = Circle2 {
            center: point_id,
            radius,
        };
        let id = self.highest_circle_id + 1;
        self.circles.insert(id, c);
        self.highest_circle_id += 1;
        id
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
        for (_point_id, point) in self.points.iter_mut() {
            point.reset_forces();
        }
        for (_spring_id, spring) in self.springs.iter() {
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
        for (_point_id, point) in self.points.iter_mut() {
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

    pub fn save_svg(&self, filename: &str) {
        // Find the maximum extent of the points so we can set a viewport
        let mut extended_points: Vec<Point2> = self.points.values().map(|p| p.clone()).collect();

        for (_circle_id, circle) in self.circles.iter() {
            let center = self.points.get(&circle.center).unwrap();
            let left = Point2::new(center.x - circle.radius, center.y);
            let right = Point2::new(center.x + circle.radius, center.y);
            let top = Point2::new(center.x, center.y + circle.radius);
            let bottom = Point2::new(center.x, center.y - circle.radius);
            extended_points.extend(vec![left, right, top, bottom]);
        }

        for (_arc_id, arc) in self.arcs.iter() {
            let center = self.points.get(&arc.center).unwrap();
            let start = self.points.get(&arc.start).unwrap();
            let r = (center.x - start.x).hypot(center.y - start.y);
            let left = Point2::new(center.x - r, center.y);
            let right = Point2::new(center.x + r, center.y);
            let top = Point2::new(center.x, center.y + r);
            let bottom = Point2::new(center.x, center.y - r);
            extended_points.extend(vec![left, right, top, bottom]);
        }

        if extended_points.len() == 0 {
            extended_points.push(Point2::new(0.0, 0.0));
            extended_points.push(Point2::new(1.0, 1.0));
        }
        let point0 = &extended_points[0];
        let mut min_x = point0.x;
        let mut min_y = point0.y;
        let mut max_x = point0.x;
        let mut max_y = point0.y;
        for point in extended_points {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y > max_y {
                max_y = point.y;
            }
        }

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let buffer_percent = 10.0;
        let buffer_x = dx * buffer_percent / 100.0;
        let buffer_y = dy * buffer_percent / 100.0;

        let mut document = Document::new().set(
            "viewBox",
            (
                min_x - buffer_x,
                -(max_y + buffer_y),
                dx + buffer_x * 2.0,
                dy + buffer_y * 2.0,
            ),
        );

        // Start by creating shapes for each face
        let (faces, unused_segments) = self.find_faces();
        for face in faces.iter() {
            let exterior = &face.exterior;

            let mut data = self.ring_to_data(exterior, Data::new());

            for hole in face.holes.iter() {
                data = self.ring_to_data(hole, data);
            }

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.01)
                .set("fill-rule", "evenodd")
                .set("d", data);

            document = document.add(path);
        }

        for segment in unused_segments.iter() {
            let mut data = Data::new();

            match segment {
                Segment::Line(line) => {
                    let start = self.points.get(&line.start).unwrap();
                    let end = self.points.get(&line.end).unwrap();
                    data = data.move_to((start.x, -start.y));
                    data = data.line_to((end.x, -end.y));
                }
                Segment::Arc(arc) => {
                    let center = self.points.get(&arc.center).unwrap();
                    let start = self.points.get(&arc.start).unwrap();
                    let end = self.points.get(&arc.end).unwrap();

                    let r = (center.x - start.x).hypot(center.y - start.y);

                    data = data.move_to((start.x, -start.y));

                    let arc_angle_degrees = self.arc_angle(arc) * 180.0 / PI;
                    println!("arc_angle: {}", arc_angle_degrees);
                    if arc_angle_degrees > 180.0 {
                        println!("large arc flag!");
                        //A rx ry x-axis-rotation large-arc-flag sweep-flag x y
                        data = data.elliptical_arc_to((r, r, 0.0, 1, 0, end.x, -end.y));
                    } else {
                        //A rx ry x-axis-rotation large-arc-flag sweep-flag x y
                        data = data.elliptical_arc_to((r, r, 0.0, 0, 0, end.x, -end.y));
                    }
                }
            }

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.01)
                .set("d", data);

            document = document.add(path);
        }

        // for (_circle_id, circle) in self.circles.iter() {
        //     let center = self.points.get(&circle.center).unwrap();

        //     let svg_circle = Circle::new()
        //         .set("cx", center.x)
        //         .set("cy", -center.y)
        //         .set("r", circle.radius)
        //         .set("fill", "none")
        //         .set("stroke", "black")
        //         .set("stroke-width", 0.01);

        //     document = document.add(svg_circle);
        // }

        svg::save(filename, &document).unwrap();
    }

    pub fn split_intersections(&self) -> Self {
        let mut temp_sketch = self.clone();

        // First compare every line segment against every line segment to see if they intersect
        let mut count = 0;
        let mut intersections: Vec<(u64, u64, Point2)> = vec![];

        for (i, (line_a_id, line_a)) in temp_sketch.line_segments.iter().enumerate() {
            for (j, (line_b_id, line_b)) in temp_sketch.line_segments.iter().enumerate() {
                // we only need to compare each line segment to each other line segment once
                // so we can skip indices where i > j.
                // Also, every line intersects itself so no need to check that.
                if i >= j {
                    continue;
                }

                // line segments which share a point do intersect, but there's nothing to be done
                // so let's just skip
                if line_a.start == line_b.start
                    || line_a.start == line_b.end
                    || line_a.end == line_b.end
                    || line_a.end == line_b.start
                {
                    continue;
                }

                count += 1;

                let intersection = temp_sketch.line_intersection(line_a, line_b);
                match intersection {
                    None => {}
                    Some(point) => {
                        intersections.push((*line_a_id, *line_b_id, point));
                    }
                }
            }
        }

        for (line_a_id, line_b_id, point) in intersections {
            let line_a = temp_sketch.line_segments.get(&line_a_id).unwrap().clone();
            let line_b = temp_sketch.line_segments.get(&line_b_id).unwrap().clone();

            let new_point_id = temp_sketch.add_point(point.x, point.y);
            temp_sketch.add_segment(line_a.start, new_point_id);
            temp_sketch.add_segment(new_point_id, line_a.end);
            temp_sketch.add_segment(line_b.start, new_point_id);
            temp_sketch.add_segment(new_point_id, line_b.end);

            temp_sketch.line_segments.remove(&line_a_id);
            temp_sketch.line_segments.remove(&line_b_id);
        }

        // Second, compare every circle against every other circle to see if they intersect
        let mut circle_intersections: Vec<(u64, u64, Vec<Point2>)> = vec![];
        for (i, (circle_a_id, circle_a)) in temp_sketch.circles.iter().enumerate() {
            for (j, (circle_b_id, circle_b)) in temp_sketch.circles.iter().enumerate() {
                // we only need to compare each circle to each other circle once
                // so we can skip indices where i > j.
                // Also, every line intersects itself so no need to check that.
                if i >= j {
                    continue;
                }

                // circles which share a point do intersect, but there's nothing to be done
                // so let's just skip
                if circle_a.center == circle_b.center {
                    continue;
                }

                count += 1;

                let intersection = temp_sketch.circle_intersection(circle_a, circle_b);
                circle_intersections.push((*circle_a_id, *circle_b_id, intersection));
            }
        }

        for (circle_a_id, circle_b_id, points) in circle_intersections {
            // TODO: check for duplicates! Things get hairy if 3 circles intersect at the same point!
            let circle_a = temp_sketch.circles.get(&circle_a_id).unwrap().clone();
            let circle_b = temp_sketch.circles.get(&circle_b_id).unwrap().clone();

            let center_a = temp_sketch.points.get(&circle_a.center).unwrap().clone();
            let center_b = temp_sketch.points.get(&circle_b.center).unwrap().clone();

            println!(
                "Circle A: {:?} centered: at {}, {}",
                circle_a, center_a.x, center_a.y
            );
            println!(
                "Circle B: {:?} centered: at {}, {}",
                circle_b, center_b.x, center_b.y
            );

            let new_point_0 = temp_sketch.add_point(points[0].x, points[0].y);
            let new_point_1 = temp_sketch.add_point(points[1].x, points[1].y);

            temp_sketch.add_arc(circle_a.center, new_point_0, new_point_1);
            temp_sketch.add_arc(circle_a.center, new_point_1, new_point_0);

            temp_sketch.add_arc(circle_b.center, new_point_0, new_point_1);
            temp_sketch.add_arc(circle_b.center, new_point_1, new_point_0);

            // temp_sketch.circles.remove(&circle_a_id);
            // temp_sketch.circles.remove(&circle_b_id);
        }

        temp_sketch
    }

    pub fn find_faces(&self) -> (Vec<Face>, Vec<Segment>) {
        let mut segments_overall: Vec<Segment> = vec![];
        for line in self.line_segments.values() {
            segments_overall.push(Segment::Line(line.clone()));
        }
        for arc in self.arcs.values() {
            segments_overall.push(Segment::Arc(arc.clone()));
        }

        let (rings, unused_segments) = self.find_rings(segments_overall, false);
        let mut faces: Vec<Face> = rings.iter().map(|r| Face::from_ring(r)).collect();

        if rings.len() == 0 {
            return (faces, unused_segments);
        }

        // this next block of code converts everything to Polygons just so we can
        // determine what faces contain which other faces. It's a bit of a waste
        // because geo is a relatively heavy dependency and we don't need all of
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

        if debug {
            println!(
                "Overall: {:?} segments including reversals",
                segments_overall.len()
            );
        }

        // keep track of every index we've already used--each segment can only be used once
        let mut used_indices: Vec<usize> = vec![];
        // this is the output variable
        let mut new_rings: Vec<Vec<usize>> = vec![];

        for (seg_idx, s) in segments_overall.iter().enumerate() {
            if debug {
                println!("Starting a loop with segment: {:?}", s);
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
                let actual_segment = segments_overall.get(*segment_index).unwrap();
                this_ring.push(actual_segment.clone());
            }
            all_rings.push(Ring::Segments(this_ring));
        }

        // Circles are trivially rings!
        for (_circle_id, circle) in self.circles.iter() {
            all_rings.push(Ring::Circle(circle.clone()));
        }

        all_rings.sort_by(|r1, r2| {
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

        (all_rings, unused_segments)
    }

    pub fn find_next_segment_index(
        &self,
        segments: &Vec<Segment>,
        starting_segment: &Segment,
        used_indices: &Vec<usize>,
        debug: bool,
    ) -> Option<usize> {
        let mut matches: Vec<usize> = vec![];
        for (idx, s2) in segments.iter().enumerate() {
            if used_indices.contains(&idx) {
                continue;
            }
            if s2.continues(&starting_segment) && !s2.equals_or_reverse_equals(&starting_segment) {
                matches.push(idx);
            }
        }

        if matches.len() == 0 {
            None
        } else if matches.len() == 1 {
            Some(matches[0])
        } else {
            if debug {
                println!("\tMultiple options! Deciding which one to take...");
            }
            let point_a = starting_segment.get_start();
            let point_b = starting_segment.get_end();

            let mut best_option: usize = 0;
            let mut biggest_angle: f64 = 0.0;
            for option in matches {
                let point_c = segments[option].get_end();
                let ang = angle(
                    &self.points[&point_a],
                    &self.points[&point_b],
                    &self.points[&point_c],
                );
                if debug {
                    println!(
                        "\tAngle from {} to {} to {}: {}",
                        point_a,
                        point_b,
                        point_c,
                        ang * 180.0 / 3.1415926
                    );
                }
                if ang >= biggest_angle {
                    biggest_angle = ang;
                    best_option = option;
                }
            }

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

    pub fn ring_to_data(&self, ring: &Ring, mut data: Data) -> Data {
        match ring {
            Ring::Circle(circle) => {
                let center = self.points.get(&circle.center).unwrap();
                let radius = circle.radius;
                data = data.move_to((center.x, -center.y + radius)); // starts at bottom
                data = data.elliptical_arc_to((
                    radius,
                    radius,
                    0.0,
                    0,
                    0,
                    center.x,
                    -center.y - radius,
                )); // arc to top

                data = data.elliptical_arc_to((
                    radius,
                    radius,
                    0.0,
                    0,
                    0,
                    center.x,
                    -center.y + radius,
                )); // arc back to bottom

                data
            }
            Ring::Segments(segments) => {
                let mut first = true;
                for segment in segments {
                    match segment {
                        Segment::Line(line) => {
                            let start = self.points.get(&line.start).unwrap();
                            let end = self.points.get(&line.end).unwrap();

                            if first {
                                data = data.move_to((start.x, -start.y));
                                first = false;
                            }
                            data = data.line_to((end.x, -end.y));
                        }
                        Segment::Arc(arc) => {
                            let center = self.points.get(&arc.center).unwrap();
                            let start = self.points.get(&arc.start).unwrap();
                            let end = self.points.get(&arc.end).unwrap();

                            let r = (center.x - start.x).hypot(center.y - start.y);

                            if first {
                                data = data.move_to((start.x, -start.y));
                                first = false;
                            }

                            let arc_angle = self.arc_angle(arc);
                            println!("Ring has an arc with arc_angle: {}", arc_angle);

                            //A rx ry x-axis-rotation large-arc-flag sweep-flag x y
                            data = data.elliptical_arc_to((r, r, 0.0, 0, 0, end.x, -end.y));
                        }
                    }
                }
                data
            }
        }
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

    fn distance_to(&self, other: &Point2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx.hypot(dy)
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Arc2 {
    center: u64,
    start: u64,
    end: u64,
}

impl Arc2 {
    pub fn reverse(&self) -> Self {
        Arc2 {
            center: self.center,
            start: self.end,
            end: self.start,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Line2 {
    start: u64,
    end: u64,
}

impl Line2 {
    pub fn reverse(&self) -> Self {
        Line2 {
            start: self.end,
            end: self.start,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ring {
    Circle(Circle2),
    Segments(Vec<Segment>),
}

impl Ring {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    exterior: Ring,
    holes: Vec<Ring>,
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
        return path_a;
    } else {
        return path_b;
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

    return naive_diff;
}
