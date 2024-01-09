use std::collections::VecDeque;

use crate::sketch::{Arc2, Circle2, IncrementingMap, Line2, Point2, Sketch};
use itertools::Itertools;
use std::f64::consts::{PI, TAU};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle(Circle2),
    Arc(Arc2),
    Line(Line2),
}

impl Shape {
    pub fn split_at_point_id(&self, new_point_id: u64) -> (Shape, Shape) {
        match self {
            Shape::Line(line) => {
                let new_line_1 = Line2 {
                    start: line.start,
                    end: new_point_id,
                };
                let new_line_2 = Line2 {
                    start: new_point_id,
                    end: line.end,
                };
                (Shape::Line(new_line_1), Shape::Line(new_line_2))
            }
            Shape::Circle(circle) => todo!(),
            Shape::Arc(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Intersection {
    None,
    OnePoint(Point2, bool),
    TwoPoints(Point2, bool, Point2, bool),
    Line(Point2, Point2),
    Arc(Arc2),
    Circle(Circle2),
}

impl Sketch {
    pub fn shape_intersection(&self, shape_a: &Shape, shape_b: &Shape) -> Intersection {
        match (shape_a, shape_b) {
            (Shape::Line(line_a), Shape::Line(line_b)) => {
                self.line_line_intersection(&line_a, &line_b)
            }
            (Shape::Circle(circle_a), Shape::Circle(circle_b)) => {
                self.circle_circle_intersection(&circle_a, &circle_b)
            }
            (Shape::Circle(circle), Shape::Arc(arc)) => self.circle_arc_intersection(&circle, &arc),
            _ => Intersection::None,
        }
    }

    pub fn split_intersections(&self) -> Self {
        let mut temp_sketch = self.clone();

        // First put all segments: Arcs, Lines, Circles into one big collection
        let mut all_shapes: IncrementingMap<Shape> = IncrementingMap::new();

        let line_ids: Vec<u64> = temp_sketch.line_segments.keys().cloned().sorted().collect();
        for line_id in line_ids {
            let line = temp_sketch.line_segments.get(&line_id).unwrap();
            all_shapes.add_item(Shape::Line(line.clone()));
        }

        let circle_ids: Vec<u64> = temp_sketch.circles.keys().cloned().sorted().collect();
        for circle_id in circle_ids {
            let circle = temp_sketch.circles.get(&circle_id).unwrap();
            all_shapes.add_item(Shape::Circle(circle.clone()));
        }

        let arc_ids: Vec<u64> = temp_sketch.arcs.keys().cloned().sorted().collect();
        for arc_id in arc_ids {
            let arc = temp_sketch.arcs.get(&arc_id).unwrap();
            all_shapes.add_item(Shape::Arc(arc.clone()));
        }

        // println!("JUST debugging 0 vs 5");
        // let intersection = temp_sketch.shape_intersection(
        //     all_shapes.items.get(&0).unwrap(),
        //     all_shapes.items.get(&5).unwrap(),
        // );
        // println!("Intersection: {:?}", intersection);

        // return temp_sketch;

        // then compare each one against each other one (once, skipping itself)
        // let mut all_intersections: Vec<(u64, u64, Intersection)> = vec![];
        let mut all_intersections: VecDeque<(u64, u64, Intersection)> = VecDeque::new();
        for (shape_a_id, shape_a) in all_shapes.items.iter() {
            for (shape_b_id, shape_b) in all_shapes.items.iter() {
                if (shape_a_id >= shape_b_id) {
                    continue;
                }

                let intersection = temp_sketch.shape_intersection(shape_a, shape_b);

                // if *shape_a_id == 11 && *shape_b_id == 13 {
                //     println!("What's up at 11, 13?");
                //     println!("{:?}", intersection);
                // }

                match intersection {
                    Intersection::None => {}
                    Intersection::OnePoint(_, true) => {}
                    _ => {
                        println!("Got a real intersection");
                        all_intersections.push_back((*shape_a_id, *shape_b_id, intersection));
                    }
                }
            }
        }

        println!("\nHave intersections, now splitting shapes");
        for i in all_intersections.iter() {
            println!("{:?}", i);
        }
        loop {
            if all_intersections.len() == 0 {
                break;
            }
            let intersection_tuple = all_intersections.pop_front().unwrap();

            println!(
                "Now considering this intersection:\n{:?}",
                intersection_tuple
            );

            println!("Leaving the following:");
            for inters in all_intersections.iter() {
                println!("{:?}", inters);
            }

            let (shape_a_id, shape_b_id, intersection) = intersection_tuple;
            let shape_a = all_shapes.get_item(shape_a_id).unwrap().clone();
            let shape_b = all_shapes.get_item(shape_b_id).unwrap().clone();

            match intersection {
                Intersection::None => {}
                Intersection::OnePoint(point, _) => {
                    // this code currently assumes line vs line intersection. need to
                    // move this under a match statement
                    let new_point_id = temp_sketch.add_point(point.x, point.y);
                    println!("Add an intersection point with ID: {:?}", new_point_id);

                    let (new_shape_one, new_shape_two) = shape_a.split_at_point_id(new_point_id);
                    let (new_shape_three, new_shape_four) = shape_b.split_at_point_id(new_point_id);

                    let mut new_shapes: Vec<u64> = vec![];
                    new_shapes.push(all_shapes.add_item(new_shape_one.clone()));
                    new_shapes.push(all_shapes.add_item(new_shape_two.clone()));
                    new_shapes.push(all_shapes.add_item(new_shape_three.clone()));
                    new_shapes.push(all_shapes.add_item(new_shape_four.clone()));
                    all_shapes.remove_item(shape_a_id);
                    all_shapes.remove_item(shape_b_id);

                    println!(
                        "Adding new shapes: {:?}, {:?}, {:?}, {:?}",
                        new_shapes.get(new_shapes.len() - 4),
                        new_shapes.get(new_shapes.len() - 3),
                        new_shapes.get(new_shapes.len() - 2),
                        new_shapes.get(new_shapes.len() - 1),
                    );

                    println!("Removing shapes: {:?}, {:?}", shape_a_id, shape_b_id);

                    // now we have to sweep through all the existing intersections and remove any that reference the
                    // ids of the shapes we just removed, adding each one to a list of possible new intersections to check for
                    let mut possibilities: Vec<u64> = vec![];
                    let mut indices_to_remove: Vec<usize> = vec![];
                    for (index, (a, b, _)) in all_intersections.iter().enumerate() {
                        if *a == shape_a_id || *a == shape_b_id {
                            possibilities.push(*b);
                            indices_to_remove.push(index);
                        }
                        if *b == shape_a_id || *b == shape_b_id {
                            possibilities.push(*a);
                            indices_to_remove.push(index);
                        }
                    }

                    println!("New possibilities: {:?}", possibilities);
                    println!("Indices to remove: {:?}", indices_to_remove);

                    for index_to_remove in indices_to_remove.iter().rev() {
                        all_intersections.remove(*index_to_remove);
                    }
                    println!("All intersections after removing stale ones:");
                    for existing_intersection in all_intersections.iter() {
                        println!("{:?}", existing_intersection);
                    }
                    println!("that was all of them");

                    for possibility_id in possibilities {
                        for new_line_id in new_shapes.iter() {
                            // println!("Checking {:?} against {:?}", possibility_id, new_line_id);
                            let possibility = all_shapes.get_item(possibility_id).unwrap();
                            let new_line = all_shapes.get_item(*new_line_id).unwrap();
                            let intersection =
                                temp_sketch.shape_intersection(possibility, new_line);

                            match intersection {
                                Intersection::None => {}
                                Intersection::OnePoint(_, true) => {}
                                _ => {
                                    all_intersections.push_back((
                                        possibility_id,
                                        *new_line_id,
                                        intersection,
                                    ));
                                    // println!("  hit!");
                                }
                            }
                        }
                    }
                }
                Intersection::Line(point_a, point_b) => {
                    println!("Intersection line: {:?} {:?}", point_a, point_b);
                }
                Intersection::TwoPoints(point_a, false, point_b, false) => {
                    println!("Shape A: {:?}", shape_a);
                    println!("Shape B: {:?}", shape_b);

                    match (shape_a, shape_b) {
                        (Shape::Circle(circle_a), Shape::Circle(circle_b)) => {
                            // we need to add two new points, one for each of these intersections
                            let new_point_0 = temp_sketch.add_point(point_a.x, point_a.y);
                            let new_point_1 = temp_sketch.add_point(point_b.x, point_b.y);

                            // then break each circle up into two arcs:
                            let arc_a_0 = Arc2 {
                                center: circle_a.center,
                                start: new_point_0,
                                end: new_point_1,
                                clockwise: false,
                            };
                            let arc_a_1 = Arc2 {
                                center: circle_a.center,
                                start: new_point_1,
                                end: new_point_0,
                                clockwise: false,
                            };
                            let arc_b_0 = Arc2 {
                                center: circle_b.center,
                                start: new_point_0,
                                end: new_point_1,
                                clockwise: false,
                            };
                            let arc_b_1 = Arc2 {
                                center: circle_b.center,
                                start: new_point_1,
                                end: new_point_0,
                                clockwise: false,
                            };

                            // add these four new arcs
                            let mut new_shapes: Vec<u64> = vec![];
                            new_shapes.push(all_shapes.add_item(Shape::Arc(arc_a_0)));
                            new_shapes.push(all_shapes.add_item(Shape::Arc(arc_a_1)));
                            new_shapes.push(all_shapes.add_item(Shape::Arc(arc_b_0)));
                            new_shapes.push(all_shapes.add_item(Shape::Arc(arc_b_1)));

                            // remove the two circles
                            all_shapes.remove_item(shape_a_id);
                            all_shapes.remove_item(shape_b_id);

                            // now we have to sweep through all the existing intersections and remove any that reference the
                            // ids of the circles we just removed, adding each one to a list of possible new intersections to check for
                            let mut possibilities: Vec<u64> = vec![];
                            let mut indices_to_remove: Vec<usize> = vec![];
                            for (index, (a, b, _)) in all_intersections.iter().enumerate() {
                                if *a == shape_a_id || *a == shape_b_id {
                                    possibilities.push(*b);
                                    indices_to_remove.push(index);
                                }
                                if *b == shape_a_id || *b == shape_b_id {
                                    possibilities.push(*a);
                                    indices_to_remove.push(index);
                                }
                            }

                            println!("New possibilities: {:?}", possibilities);
                            println!("Indices to remove: {:?}", indices_to_remove);

                            for index_to_remove in indices_to_remove.iter().rev() {
                                all_intersections.remove(*index_to_remove);
                            }
                            println!("All intersections after removing stale ones:");
                            for existing_intersection in all_intersections.iter() {
                                println!("{:?}", existing_intersection);
                            }
                            println!("that was all of them");

                            for possibility_id in possibilities {
                                for new_arc_id in new_shapes.iter() {
                                    println!(
                                        "Checking {:?} against {:?}",
                                        possibility_id, new_arc_id
                                    );
                                    let possibility = all_shapes.get_item(possibility_id).unwrap();
                                    let new_line = all_shapes.get_item(*new_arc_id).unwrap();
                                    let intersection =
                                        temp_sketch.shape_intersection(possibility, new_line);

                                    println!("intersection: {:?}", intersection);
                                    let intersection_clone = intersection.clone();

                                    match intersection {
                                        Intersection::None => {}
                                        Intersection::TwoPoints(p_a, p_a_d, p_b, p_b_d) => {
                                            all_intersections.push_back((
                                                possibility_id,
                                                *new_arc_id,
                                                intersection_clone,
                                            ));
                                        }
                                        _ => todo!(),
                                    }
                                }
                            }
                        }
                        (Shape::Circle(circle), Shape::Arc(arc)) => {
                            println!("Circle on Arc collision!");
                        }
                        (_, _) => todo!(),
                    }
                }
                Intersection::TwoPoints(point_a, _, point_b, _) => todo!(),
                Intersection::Arc(_) => todo!(),
                Intersection::Circle(_) => todo!(),
            }

            if all_intersections.len() == 0 {
                break;
            }
        }

        // println!("All Shapes at the end: {:?}", all_shapes.items);

        let mut final_sketch = Sketch::new();
        final_sketch.points = temp_sketch.points;
        final_sketch.highest_point_id = temp_sketch.highest_point_id;
        for shape in all_shapes.items.iter() {
            match shape {
                (id, Shape::Line(line)) => {
                    final_sketch.add_segment(line.start, line.end);
                }
                (id, Shape::Circle(circle)) => {
                    final_sketch.add_circle(circle.center, circle.radius);
                }
                (id, Shape::Arc(arc)) => {
                    final_sketch.add_arc(arc.center, arc.start, arc.end, arc.clockwise);
                }
                _ => {}
            }
        }

        println!("So, in summary I've generated these shapes:");
        for shape in all_shapes.items.iter() {
            println!("{:?}", shape);
        }

        final_sketch
    }

    pub fn line_line_intersection(&self, line_a: &Line2, line_b: &Line2) -> Intersection {
        let a_start = self.points.get(&line_a.start).unwrap();
        let a_end = self.points.get(&line_a.end).unwrap();
        let b_start = self.points.get(&line_b.start).unwrap();
        let b_end = self.points.get(&line_b.end).unwrap();

        // println!("a start: {:?}", a_start);
        // println!("a end: {:?}", a_end);
        // println!("b start: {:?}", b_start);
        // println!("b end: {:?}", b_end);

        // Start by handling the degenerate cases
        if line_a.start == line_b.start || line_a.start == line_b.end {
            // println!("A.start is degen");

            if line_a.end == line_b.start || line_a.end == line_b.end {
                // println!("AND A.end is degen");
                // these lines are equal or reverse equal
                return Intersection::Line(a_start.clone(), a_end.clone());
            }

            return Intersection::OnePoint(self.points.get(&line_a.start).unwrap().clone(), true);
        }
        if line_a.end == line_b.start || line_a.end == line_b.end {
            // println!("A.end is degen");
            return Intersection::OnePoint(self.points.get(&line_a.end).unwrap().clone(), true);
        }

        // println!("Was not degenerate");

        fn normal_form(start: &Point2, end: &Point2) -> (f64, f64, f64) {
            let a = start.y - end.y;
            let b = end.x - start.x;
            let c = (start.x - end.x) * start.y + (end.y - start.y) * start.x;
            return (a, b, c);
        }

        let (a1, b1, c1) = normal_form(&a_start, &a_end);
        let (a2, b2, c2) = normal_form(&b_start, &b_end);

        let x_intercept = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
        let y_intercept = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);

        // println!(
        //     "Computed X and Y intercept: {}, {}",
        //     x_intercept, y_intercept
        // );

        if x_intercept.is_infinite() || y_intercept.is_infinite() {
            // println!("Infinite intercept, so lines are parallel");
            return Intersection::None;
        }

        fn within_range(x: f64, a: f64, b: f64, epsilon: f64) -> bool {
            if a < b {
                return x >= a - epsilon && x <= b + epsilon;
            } else {
                return x >= b - epsilon && x <= a + epsilon;
            }
        }

        if x_intercept.is_nan() && y_intercept.is_nan() {
            // println!("NaN intercept, so either there is line overlap or they are disjoint");

            let mut special_points: Vec<Point2> = vec![];

            if within_range(a_start.x, b_start.x, b_end.x, 0.0)
                && within_range(a_start.y, b_start.y, b_end.y, 0.0)
            {
                special_points.push(a_start.clone());
            }

            if within_range(a_end.x, b_start.x, b_end.x, 0.0)
                && within_range(a_end.y, b_start.y, b_end.y, 0.0)
            {
                special_points.push(a_end.clone());
            }

            if within_range(b_start.x, a_start.x, a_end.x, 0.0)
                && within_range(b_start.y, a_start.y, a_end.y, 0.0)
            {
                special_points.push(b_start.clone());
            }

            if within_range(b_end.x, a_start.x, a_end.x, 0.0)
                && within_range(b_end.y, a_start.y, a_end.y, 0.0)
            {
                special_points.push(b_end.clone());
            }

            match special_points.len() {
                0 => return Intersection::None,
                2 => {
                    return Intersection::Line(special_points[0].clone(), special_points[1].clone())
                }
                _ => panic!(
                    "Unexpected number of special points: {}",
                    special_points.len()
                ),
            }
        }

        // it only counts as an intersection if it falls within both the segments
        // Check that the x-intercept is within the x-range of the first segment

        // println!("X intercept: {}", x_intercept);
        // println!("{}", a_start.x.min(a_end.x));
        // println!("{}", a_start.x.max(a_end.x));

        // if x_intercept >= a_start.x.min(a_end.x) && x_intercept <= a_start.x.max(a_end.x) {
        let epsilon = 1e-12;
        if within_range(x_intercept, a_start.x, a_end.x, epsilon)
            && within_range(y_intercept, a_start.y, a_end.y, epsilon)
        {
            // println!("Fulfilled x range on a");
            // check that the x-intercept is within the x-range of the second segment
            // if x_intercept >= b_start.x.min(b_end.x) && x_intercept <= b_start.x.max(b_end.x) {
            if within_range(x_intercept, b_start.x, b_end.x, epsilon)
                && within_range(y_intercept, b_start.y, b_end.y, epsilon)
            {
                // println!("Fulfilled x range on b");
                return Intersection::OnePoint(Point2::new(x_intercept, y_intercept), false);
            }
        }

        // println!("Did not fulfill x range somehow");

        Intersection::None
    }

    pub fn circle_circle_intersection(
        &self,
        circle_a: &Circle2,
        circle_b: &Circle2,
    ) -> Intersection {
        let center_a = self.points.get(&circle_a.center).unwrap();
        let center_b = self.points.get(&circle_b.center).unwrap();
        let r_a = circle_a.radius;
        let r_b = circle_b.radius;

        // compute distance between centers
        let center_dx = center_b.x - center_a.x;
        let center_dy = center_b.y - center_a.y;
        let center_dist = center_dx.hypot(center_dy);

        // if the circles are too far away OR too close, they don't intersect
        if center_dist > r_a + r_b {
            return Intersection::None;
        }
        if center_dist < (r_a - r_b).abs() {
            return Intersection::None;
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

        Intersection::TwoPoints(Point2::new(ix1, iy1), false, Point2::new(ix2, iy2), false)
    }

    pub fn circle_arc_intersection(&self, circle: &Circle2, arc: &Arc2) -> Intersection {
        // treat this is circle/circle intersection, then just do some checks
        // afterwards to make sure the intersection points really do fall within
        // the bounds of the arc
        let arc_center = self.points.get(&arc.center).unwrap();
        let arc_start = self.points.get(&arc.start).unwrap();
        let arc_dx = arc_center.x - arc_start.x;
        let arc_dy = arc_center.y - arc_start.y;
        let arc_radius = arc_dx.hypot(arc_dy);
        let fake_circle = Circle2 {
            center: arc.center,
            radius: arc_radius,
            top: arc.start,
        };

        let fake_intersection = self.circle_circle_intersection(circle, &fake_circle);
        println!("Fake intersection: {:?}", fake_intersection);

        match fake_intersection {
            Intersection::None => Intersection::None,
            Intersection::OnePoint(_, _) => todo!(),
            Intersection::TwoPoints(point_a, is_degenerate_a, point_b, is_degenerate_b) => {
                // check to make sure that both points fall within the arc. If only one
                // of them does, just return that one. if none do, return none.
                // if both do, return both.
                let point_a_good = self.point_within_arc(arc, &point_a);
                let point_b_good = self.point_within_arc(arc, &point_b);

                match (point_a_good, point_b_good) {
                    (true, true) => {
                        Intersection::TwoPoints(point_a, is_degenerate_a, point_b, is_degenerate_b)
                    }
                    (true, false) => Intersection::OnePoint(point_a, is_degenerate_a),
                    (false, true) => Intersection::OnePoint(point_b, is_degenerate_b),
                    (false, false) => Intersection::None,
                }
            }
            Intersection::Line(_, _) => todo!(),
            Intersection::Arc(_) => todo!(),
            Intersection::Circle(_) => todo!(),
        }
    }

    pub fn point_within_arc(&self, arc: &Arc2, point: &Point2) -> bool {
        let center = self.points.get(&arc.center).unwrap();
        let mut start = self.points.get(&arc.start).unwrap();
        let mut end = self.points.get(&arc.end).unwrap();

        // clockwise arcs are weird and unconventional. Within this function, pretend all arcs are CCW.
        // doing this destroys 1 bit of information about the arc, but it's irrelevant for the purposes of this function
        if arc.clockwise {
            (start, end) = (end, start);
        }

        // cool, so you only have to imagine this math working for CCW arcs
        let start_dx = start.x - center.x;
        let start_dy = start.y - center.y;
        let start_angle = start_dy.atan2(start_dx);

        let end_dx = end.x - center.x;
        let end_dy = end.y - center.y;
        let mut end_angle = end_dy.atan2(end_dx);

        if end_angle <= start_angle {
            end_angle += TAU;
        }

        let point_dx = point.x - center.x;
        let point_dy = point.y - center.y;
        let mut point_angle = point_dy.atan2(point_dx);

        if point_angle < start_angle {
            point_angle += TAU;
        }

        if point_angle >= start_angle && point_angle <= end_angle {
            // okay the angles work out, but we gotta run one last check:
            // make sure the point is the right distance from center!
            let arc_radius = start_dy.hypot(start_dx);
            let point_radius = point_dy.hypot(point_dx);
            let radius_diff = (arc_radius - point_radius).abs();

            // floats are never really *equal*, just nearly equal
            radius_diff < 1e-10
        } else {
            false
        }
    }

    pub fn arc_arc_intersection(&self, arc_a: &Arc2, arc_b: &Arc2) -> Intersection {
        // treat this is circle/circle intersection, then just do some checks
        // afterwards to make sure the intersection points really do fall within
        // the bounds of the arcs

        Intersection::None
    }
}

#[cfg(test)]
mod tests {
    use crate::project::Project;

    use super::*;

    #[test]
    fn line_through_rectangle() {
        let contents =
            std::fs::read_to_string("src/test_inputs/line_through_rectangle.cadmium").unwrap();
        let p: Project = serde_json::from_str(&contents).unwrap();
        // println!("{:?}", p);

        let realized = p.get_realization(0, 1000);
        let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();
        println!("Sketch: {:?}", sketch_split);
        println!("Faces: {:?}", sketch_split.faces);
        println!("Number of faces: {:?}", sketch_split.faces.len());
        assert_eq!(sketch_split.faces.len(), 2);
    }

    #[test]
    fn line_through_many_rectangles() {
        let contents =
            std::fs::read_to_string("src/test_inputs/line_through_many_rectangles.cadmium")
                .unwrap();
        let p: Project = serde_json::from_str(&contents).unwrap();
        // println!("{:?}", p);

        let realized = p.get_realization(0, 1000);
        let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();
        // println!("Sketch: {:?}", sketch_split);
        // println!("Faces: {:?}", sketch_split.faces);
        println!("Number of faces: {:?}", sketch_split.faces.len());
        assert_eq!(sketch_split.faces.len(), 8);
    }

    #[test]
    fn two_circles_two_intersections() {
        // two intersecting circles should yield 3 extrudable faces
        let contents = std::fs::read_to_string(
            "src/test_inputs/sketches/circle_circle/two_circles_two_intersections.cadmium",
        )
        .unwrap();
        let p: Project = serde_json::from_str(&contents).unwrap();

        let realized = p.get_realization(0, 1000);
        let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();

        println!("Number of faces: {:?}", sketch_split.faces.len());
        assert_eq!(sketch_split.faces.len(), 3);
    }

    #[test]
    fn three_circles() {
        // three intersecting circles should yield 5 extrudable faces
        let contents =
            std::fs::read_to_string("src/test_inputs/sketches/circle_circle/three_circles.cadmium")
                .unwrap();
        let p: Project = serde_json::from_str(&contents).unwrap();

        let realized = p.get_realization(0, 1000);
        let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();

        println!("Number of faces: {:?}", sketch_split.faces.len());
        assert_eq!(sketch_split.faces.len(), 3);
    }

    #[test]
    fn points_are_in_arcs() {
        let mut sketch = Sketch::new();

        let origin = sketch.add_point(0.0, 0.0);
        let right = sketch.add_point(1.0, 0.0);
        let left = sketch.add_point(-1.0, 0.0);
        let arc_top = Arc2 {
            center: origin,
            start: right,
            end: left,
            clockwise: false,
        };
        let arc_bottom = Arc2 {
            center: origin,
            start: left,
            end: right,
            clockwise: false,
        };
        let arc_top_cw = Arc2 {
            center: origin,
            start: left,
            end: right,
            clockwise: true,
        };
        let arc_bottom_cw = Arc2 {
            center: origin,
            start: right,
            end: left,
            clockwise: true,
        };

        let up_top = Point2::new(0.0, 1.0);
        let down_low = Point2::new(0.0, -1.0);

        // counterclockwise, as god intended
        assert_eq!(sketch.point_within_arc(&arc_top, &up_top), true);
        assert_eq!(sketch.point_within_arc(&arc_top, &down_low), false);

        assert_eq!(sketch.point_within_arc(&arc_bottom, &up_top), false);
        assert_eq!(sketch.point_within_arc(&arc_bottom, &down_low), true);

        // clockwise, like a hooligan
        assert_eq!(sketch.point_within_arc(&arc_top_cw, &up_top), true);
        assert_eq!(sketch.point_within_arc(&arc_top_cw, &down_low), false);

        assert_eq!(sketch.point_within_arc(&arc_bottom_cw, &up_top), false);
        assert_eq!(sketch.point_within_arc(&arc_bottom_cw, &down_low), true);

        let way_up_top = Point2::new(0.0, 100.0);
        assert_eq!(sketch.point_within_arc(&arc_top, &way_up_top), false);
    }

    #[test]
    fn circle_circle_intersection() {
        let mut sketch = Sketch::new();

        // two touching normally
        println!("two circles touching normally");
        let a_radius = 1.0;
        let a = sketch.add_point(0.0, 0.0);
        let a_top = sketch.add_point(0.0, a_radius);
        let b_radius = 1.0;
        let b = sketch.add_point(1.0, 0.0);
        let b_top = sketch.add_point(1.0, b_radius);
        let circle_a = Circle2 {
            center: a,
            radius: a_radius,
            top: a_top,
        };
        let circle_b = Circle2 {
            center: b,
            radius: b_radius,
            top: b_top,
        };
        let intersection = sketch.circle_circle_intersection(&circle_a, &circle_b);
        assert_eq!(
            intersection,
            Intersection::TwoPoints(
                Point2::new(0.5, -0.8660254037844386),
                false,
                Point2::new(0.5, 0.8660254037844386),
                false
            )
        );
    }

    #[test]
    fn line_line_intersection() {
        let mut sketch = Sketch::new();

        // simple cross
        println!("simple cross");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(0.0, -1.0);
        let d = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(
            intersection,
            Intersection::OnePoint(Point2::new(0.0, 0.0), false)
        );

        // a T
        println!("a T");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(0.0, 0.0);
        let d = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(
            intersection,
            Intersection::OnePoint(Point2::new(0.0, 0.0), false)
        );

        // parallel horizontal
        println!("parallel horizontal");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(-1.0, 1.0);
        let d = sketch.add_point(1.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(intersection, Intersection::None);

        // parallel vertical
        println!("parallel vertical");
        let a = sketch.add_point(0.0, -1.0);
        let b = sketch.add_point(0.0, 1.0);
        let c = sketch.add_point(1.0, -1.0);
        let d = sketch.add_point(1.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(intersection, Intersection::None);

        // perpendicular but not intersecting
        println!("perpendicular but not intersecting");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(3.0, 0.0);
        let d = sketch.add_point(3.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(intersection, Intersection::None);

        // share 1 point but only in the === sense not the == sense
        println!("share 1 point but only in the === sense not the == sense");
        let a = sketch.add_point(-1.0, 1.0);
        let b = sketch.add_point(0.0, 0.0);
        let c = sketch.add_point(0.0, 0.0);
        let d = sketch.add_point(1.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(
            intersection,
            Intersection::OnePoint(Point2::new(0.0, 0.0), false)
        );

        // share 1 point in the == sense
        println!("share 1 point in the == sense");
        let a = sketch.add_point(-1.0, 1.0);
        let b = sketch.add_point(0.0, 0.0);
        let d = sketch.add_point(1.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: b, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(
            intersection,
            Intersection::OnePoint(Point2::new(0.0, 0.0), true)
        );

        // colinear, horizontal no intersection
        println!("colinear horizontal no intersection");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(0.0, 0.0);
        let c = sketch.add_point(1.0, 0.0);
        let d = sketch.add_point(2.0, 0.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(intersection, Intersection::None);

        // colinear, vertical no intersection
        println!("colinear vertical no intersection");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(0.0, 1.0);
        let c = sketch.add_point(0.0, 2.0);
        let d = sketch.add_point(0.0, 3.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(intersection, Intersection::None);

        // Lines are exactly equal
        println!("Exactly equal");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let intersection = sketch.line_line_intersection(&line_ab, &line_ab);
        assert_eq!(
            intersection,
            Intersection::Line(Point2::new(0.0, 0.0), Point2::new(0.0, 1.0))
        );

        println!("\nLine Overlap:");
        // lines overlap somewhat, both vertical
        println!("lines overlap somewhat, both vertical");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(0.0, 2.0);
        let c = sketch.add_point(0.0, 1.0);
        let d = sketch.add_point(0.0, 3.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(
            intersection,
            Intersection::Line(Point2::new(0.0, 2.0), Point2::new(0.0, 1.0))
        );
        // for future reference: the ordering of points here and for all of the tests below is inconsequential
        // Feel free to swap the order here if the implementation changes. Maybe these should always come
        // in a canonical order?

        // lines overlap somewhat, both horizontal
        println!("lines overlap somewhat, both horizontal");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(2.0, 0.0);
        let c = sketch.add_point(1.0, 0.0);
        let d = sketch.add_point(3.0, 0.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(
            intersection,
            Intersection::Line(Point2::new(2.0, 0.0), Point2::new(1.0, 0.0))
        );

        // lines overlap somewhat, both angled
        println!("lines overlap somewhat, both angled");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(2.0, 2.0);
        let c = sketch.add_point(1.0, 1.0);
        let d = sketch.add_point(3.0, 3.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(
            intersection,
            Intersection::Line(Point2::new(2.0, 2.0), Point2::new(1.0, 1.0))
        );

        // one line engulfs the other, both angled
        println!("one line engulfs the other, both angled");
        let a = sketch.add_point(1.0, 1.0);
        let b = sketch.add_point(2.0, 2.0);
        let c = sketch.add_point(0.0, 0.0);
        let d = sketch.add_point(3.0, 3.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let intersection = sketch.line_line_intersection(&line_ab, &line_cd);
        assert_eq!(
            intersection,
            Intersection::Line(Point2::new(1.0, 1.0), Point2::new(2.0, 2.0))
        );
    }
}
