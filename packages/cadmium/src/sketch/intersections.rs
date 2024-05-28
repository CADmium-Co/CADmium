use std::{collections::VecDeque, f32::EPSILON};

use crate::sketch::{Arc2, Circle2, IncrementingMap, Line2, Point2, Sketch};
use itertools::Itertools;
use std::f64::consts::{PI, TAU};

use serde::{Deserialize, Serialize};
use tsify::Tsify;


#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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

#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Collision {
    point: Point2,
    shape_a: u64,
    shape_b: u64,
    shape_a_degeneracy: Degeneracy,
    shape_b_degeneracy: Degeneracy,
}

impl Collision {
    pub fn new(point: Point2, shape_a: u64, shape_b: u64) -> Self {
        Collision {
            point,
            shape_a,
            shape_b,
            shape_a_degeneracy: Degeneracy::None,
            shape_b_degeneracy: Degeneracy::None,
        }
    }
    pub fn degenerate(shape_a: u64, shape_b: u64) -> Self {
        Collision {
            point: Point2::new(f64::NAN, f64::NAN),
            shape_a,
            shape_b,
            shape_a_degeneracy: Degeneracy::Complete,
            shape_b_degeneracy: Degeneracy::Complete,
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Degeneracy {
    None,
    IsStart,
    IsEnd,
    Complete,
}

use Degeneracy::*;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Intersection {
    None,
    OnePoint(Point2, bool),
    TwoPoints(Point2, bool, Point2, bool),
    Line(Point2, Point2),
    Arc(Arc2),
    Circle(Circle2),
}

impl Sketch {
    pub fn identify_collisions(
        &self,
        temp_sketch: &Sketch,
        all_shapes: &IncrementingMap<Shape>,
        shape_a_id: u64,
        shape_b_id: u64,
        debug: bool,
    ) -> Vec<Collision> {
        let shape_a = all_shapes.items.get(&(shape_a_id)).unwrap();
        let shape_b = all_shapes.items.get(&(shape_b_id)).unwrap();

        if (debug) {
            println!("Shape A ({}): {:?}", shape_a_id, shape_a);
            println!("Shape B ({}): {:?}", shape_b_id, shape_b);
        }

        match (shape_a, shape_b) {
            (Shape::Circle(circle_a), Shape::Circle(circle_b)) => {
                temp_sketch.circle_circle_collisions(circle_a, shape_a_id, circle_b, shape_b_id)
            }
            (Shape::Circle(circle_a), Shape::Arc(arc_b)) => {
                temp_sketch.circle_arc_collisions(circle_a, shape_a_id, arc_b, shape_b_id)
            }
            (Shape::Circle(circle_a), Shape::Line(line_b)) => {
                temp_sketch.line_circle_collisions(line_b, shape_b_id, circle_a, shape_a_id)
            }
            (Shape::Arc(arc_a), Shape::Circle(circle_b)) => {
                temp_sketch.circle_arc_collisions(circle_b, shape_b_id, arc_a, shape_a_id)
            }
            (Shape::Arc(arc_a), Shape::Arc(arc_b)) => {
                temp_sketch.arc_arc_collisions(arc_a, shape_a_id, arc_b, shape_b_id)
            }
            (Shape::Arc(arc_a), Shape::Line(line_b)) => {
                temp_sketch.line_arc_collisions(line_b, shape_b_id, arc_a, shape_a_id)
            }
            (Shape::Line(line_a), Shape::Circle(circle_b)) => {
                temp_sketch.line_circle_collisions(line_a, shape_a_id, circle_b, shape_b_id)
            }
            (Shape::Line(line_a), Shape::Arc(arc_b)) => {
                temp_sketch.line_arc_collisions(line_a, shape_a_id, arc_b, shape_b_id)
            }
            (Shape::Line(line_a), Shape::Line(line_b)) => {
                temp_sketch.line_line_collisions(line_a, shape_a_id, line_b, shape_b_id, false)
            }
        }
    }

    pub fn process_collision(
        &self,
        temp_sketch: &mut Sketch,
        all_shapes: &mut IncrementingMap<Shape>,
        possible_shape_collisions: &mut Vec<u64>,
        new_shapes: &mut Vec<u64>,
        recently_deleted: &mut Vec<u64>,
        collision: Collision,
        debug: bool,
    ) {
        if (debug) {
            println!("Processing collision: {:?}", collision);
        }

        let shape_a_id = collision.shape_a;
        let shape_b_id = collision.shape_b;
        let point = collision.point;

        let shape_a = all_shapes.get_item(shape_a_id).unwrap().clone();
        let shape_b = all_shapes.get_item(shape_b_id).unwrap().clone();

        match (shape_a, shape_b) {
            (Shape::Circle(circle_a), Shape::Circle(circle_b)) => {
                let new_point_id = temp_sketch.add_point(point.x, point.y);

                let arc_a = temp_sketch.split_circle_at_point(&circle_a, &new_point_id, &point);
                let arc_b = temp_sketch.split_circle_at_point(&circle_b, &new_point_id, &point);

                // this is a unique case. We're making substitutions here, not deleting or creating
                all_shapes.items.insert(shape_a_id, Shape::Arc(arc_a));
                all_shapes.items.insert(shape_b_id, Shape::Arc(arc_b));

                if (debug) {
                    println!("Replaced two circles with two arcs, keeping the same IDs");
                }
            }
            (Shape::Circle(circle_a), Shape::Arc(arc_b)) => {
                let new_point_id = temp_sketch.add_point(point.x, point.y);

                // the circle can be converted to an arc
                let arc_a = temp_sketch.split_circle_at_point(&circle_a, &new_point_id, &point);
                // the arc must be split into two arcs
                let (arc_b1, arc_b2) =
                    temp_sketch.split_arc_at_point(&arc_b, &new_point_id, &point);

                // the circle -> arc amounts to a substitution not a deletion + creation
                all_shapes.items.insert(shape_a_id, Shape::Arc(arc_a));

                // but the arc -> 2 arcs is a deletion + creation
                let new_arc_b1_id = all_shapes.add_item(Shape::Arc(arc_b1));
                new_shapes.push(new_arc_b1_id);
                let new_arc_b2_id = all_shapes.add_item(Shape::Arc(arc_b2));
                new_shapes.push(new_arc_b2_id);
                recently_deleted.push(all_shapes.remove_item(shape_b_id));

                if (debug) {
                    println!("Replaced a circle with an arc ({})", shape_a_id);
                    println!(
                        "AND replaced an arc ({}) with 2 arcs ({}), ({})",
                        shape_b_id, new_arc_b1_id, new_arc_b2_id
                    );
                }
            }
            (Shape::Circle(_), Shape::Line(_)) => todo!(),
            (Shape::Arc(_), Shape::Circle(_)) => todo!(),
            (Shape::Arc(arc_a), Shape::Arc(arc_b)) => {
                let new_point_id = temp_sketch.add_point(point.x, point.y);

                let (arc_a1, arc_a2) =
                    temp_sketch.split_arc_at_point(&arc_a, &new_point_id, &point);
                let (arc_b1, arc_b2) =
                    temp_sketch.split_arc_at_point(&arc_b, &new_point_id, &point);

                new_shapes.push(all_shapes.add_item(Shape::Arc(arc_a1)));
                new_shapes.push(all_shapes.add_item(Shape::Arc(arc_a2)));
                new_shapes.push(all_shapes.add_item(Shape::Arc(arc_b1)));
                new_shapes.push(all_shapes.add_item(Shape::Arc(arc_b2)));

                recently_deleted.push(all_shapes.remove_item(shape_a_id));
                recently_deleted.push(all_shapes.remove_item(shape_b_id));

                if (debug) {
                    println!("replaced two arcs with four arcs");
                    println!(
                        "Replaced arc {} with arcs {} and {}",
                        shape_a_id, new_shapes[0], new_shapes[1]
                    );

                    println!(
                        "Replaced arc {} with arcs {} and {}",
                        shape_b_id, new_shapes[2], new_shapes[3]
                    );
                }
            }
            (Shape::Arc(_), Shape::Line(_)) => todo!(),
            (Shape::Line(line_a), Shape::Circle(circle_b)) => {
                let new_point_id = temp_sketch.add_point(point.x, point.y);

                let (line_a1, line_a2) =
                    temp_sketch.split_line_at_point(&line_a, &new_point_id, &point);
                let arc_b = temp_sketch.split_circle_at_point(&circle_b, &new_point_id, &point);

                // convert the circle into an arc in place
                all_shapes.items.insert(shape_b_id, Shape::Arc(arc_b));

                // delete the old line and replace with two smaller lines
                new_shapes.push(all_shapes.add_item(Shape::Line(line_a1)));
                new_shapes.push(all_shapes.add_item(Shape::Line(line_a2)));
                recently_deleted.push(all_shapes.remove_item(shape_a_id));

                if (debug) {
                    println!(
                        "Broke line {} into {} and {}",
                        shape_a_id, new_shapes[0], new_shapes[1]
                    );
                    println!("Replaced circle {} with arc in place", shape_b_id);
                }
            }
            (Shape::Line(line_a), Shape::Arc(arc_b)) => {
                let new_point_id = temp_sketch.add_point(point.x, point.y);

                let (line_a1, line_a2) =
                    temp_sketch.split_line_at_point(&line_a, &new_point_id, &point);
                let (arc_b1, arc_b2) =
                    temp_sketch.split_arc_at_point(&arc_b, &new_point_id, &point);

                new_shapes.push(all_shapes.add_item(Shape::Line(line_a1)));
                new_shapes.push(all_shapes.add_item(Shape::Line(line_a2)));
                new_shapes.push(all_shapes.add_item(Shape::Arc(arc_b1)));
                new_shapes.push(all_shapes.add_item(Shape::Arc(arc_b2)));

                recently_deleted.push(all_shapes.remove_item(shape_a_id));
                recently_deleted.push(all_shapes.remove_item(shape_b_id));

                if (debug) {
                    println!(
                        "Replaced line {} with {} and {}",
                        recently_deleted[0], new_shapes[0], new_shapes[1]
                    );
                    println!(
                        "Replaced arc {} with {} and {}",
                        recently_deleted[1], new_shapes[2], new_shapes[3]
                    );
                }
            }
            (Shape::Line(line_a), Shape::Line(line_b)) => {
                let collision_point_id =
                    match (&collision.shape_a_degeneracy, &collision.shape_b_degeneracy) {
                        (IsStart, None) => line_a.start,
                        (IsEnd, None) => line_a.end,
                        (None, IsStart) => line_b.start,
                        (None, IsEnd) => line_b.end,
                        (None, None) => temp_sketch.add_point(point.x, point.y),
                        (Complete, Complete) => {
                            if (debug) {
                                println!("COMPLETE degeneracy found. Removing line {}", shape_b_id);
                            }
                            all_shapes.remove_item(shape_b_id);

                            recently_deleted.push(shape_b_id);

                            return;
                        }
                        (_, _) => {
                            if (debug) {
                                println!("One line continues the other. Nothing to be done!");
                            }
                            return;
                        }
                    };

                if collision.shape_a_degeneracy == None {
                    let (line_a1, line_a2) =
                        temp_sketch.split_line_at_point(&line_a, &collision_point_id, &point);

                    new_shapes.push(all_shapes.add_item(Shape::Line(line_a1)));
                    new_shapes.push(all_shapes.add_item(Shape::Line(line_a2)));
                    recently_deleted.push(all_shapes.remove_item(shape_a_id));
                    if (debug) {
                        println!(
                            "Replaced line {} with lines {} and {}",
                            shape_a_id, new_shapes[0], new_shapes[1]
                        );
                    }
                }

                if collision.shape_b_degeneracy == None {
                    let (line_b1, line_b2) =
                        temp_sketch.split_line_at_point(&line_b, &collision_point_id, &point);

                    new_shapes.push(all_shapes.add_item(Shape::Line(line_b1)));
                    new_shapes.push(all_shapes.add_item(Shape::Line(line_b2)));
                    recently_deleted.push(all_shapes.remove_item(shape_b_id));
                    if (debug) {
                        println!(
                            "Replaced line {} with lines {} and {}",
                            shape_b_id,
                            new_shapes[new_shapes.len() - 2],
                            new_shapes[new_shapes.len() - 1]
                        );
                    }
                }

                if collision.shape_a_degeneracy == IsEnd || collision.shape_a_degeneracy == IsStart
                {
                    if !possible_shape_collisions.contains(&collision.shape_a) {
                        possible_shape_collisions.push(collision.shape_a);
                    }
                }

                if collision.shape_b_degeneracy == IsEnd || collision.shape_b_degeneracy == IsStart
                {
                    if !possible_shape_collisions.contains(&collision.shape_b) {
                        possible_shape_collisions.push(collision.shape_b);
                    }
                }
            }
        }
    }

    pub fn step_process(
        &self,
        temp_sketch: &mut Sketch,
        all_shapes: &mut IncrementingMap<Shape>,
        pairs_to_check: &mut VecDeque<(u64, u64)>,
        collisions: &mut VecDeque<Collision>,
        possible_shape_collisions: &mut Vec<u64>,
        new_shapes: &mut Vec<u64>,
        recently_deleted: &mut Vec<u64>,
        debug: bool,
    ) -> bool {
        // the bool we return indicates whether any work was done

        if (debug) {
            println!("----- Okay let's process:");
        }

        if !recently_deleted.is_empty() {
            println!("Something was recently deleted, let's fix it");
            // any collisions with the old shape are simply deleted
            let mut indices_to_delete: Vec<usize> = vec![];
            for (i, c) in collisions.iter().enumerate() {
                if recently_deleted.contains(&c.shape_a) {
                    indices_to_delete.push(i);

                    if !possible_shape_collisions.contains(&c.shape_b) {
                        possible_shape_collisions.push(c.shape_b);
                        println!("Pushed a possible shape collision against {}", c.shape_b);
                    }
                }

                if recently_deleted.contains(&c.shape_b) {
                    indices_to_delete.push(i);

                    if !possible_shape_collisions.contains(&c.shape_a) {
                        possible_shape_collisions.push(c.shape_a);
                        println!("Pushed a possible shape collision against {}", c.shape_a);
                    }
                }
            }
            for i in indices_to_delete.iter().rev() {
                collisions.remove(*i);
            }
            println!("I removed {} collisions", indices_to_delete.len());

            // We also need to comb through pairs_to_check to remove anything that
            // references these recently removed shape IDs
            indices_to_delete.clear();
            for (i, (shape_a_id, shape_b_id)) in pairs_to_check.iter().enumerate() {
                let shape_a_deleted = recently_deleted.contains(shape_a_id);
                let shape_b_deleted = recently_deleted.contains(shape_b_id);

                if !shape_a_deleted && !shape_b_deleted {
                    //nothing to do, move on
                    continue;
                }

                if shape_a_deleted && !shape_b_deleted {
                    // shape A was deleted but B wasn't, we we'll need to consider whether any of the
                    // new shapes are colliding with shape B
                    if !possible_shape_collisions.contains(shape_b_id) {
                        possible_shape_collisions.push(*shape_b_id);
                    }
                } else if shape_b_deleted && !shape_a_deleted {
                    if !possible_shape_collisions.contains(shape_a_id) {
                        possible_shape_collisions.push(*shape_a_id);
                    }
                } else if shape_a_deleted && shape_b_deleted {
                    panic!("I didn't think both shapes could be deleted at once!");
                }
                indices_to_delete.push(i);
            }
            for i in indices_to_delete.iter().rev() {
                pairs_to_check.remove(*i);
            }
            println!("I removed {} pairs to check", indices_to_delete.len());

            recently_deleted.clear();
            return true;
        }

        if !possible_shape_collisions.is_empty() || !new_shapes.is_empty() {
            if (debug) {
                println!("Possible shape collisions + new shapes! Let's add some pairs to check");
            }
            for possible_shape_id in possible_shape_collisions.iter() {
                for new_shape_id in new_shapes.iter() {
                    pairs_to_check.push_front((*possible_shape_id, *new_shape_id));
                }
            }
            possible_shape_collisions.clear();
            new_shapes.clear();
            return true;
        }

        if !collisions.is_empty() {
            if (debug) {
                println!("We have a collision to process");
            }
            let collision = collisions.pop_front().unwrap();
            self.process_collision(
                temp_sketch,
                all_shapes,
                possible_shape_collisions,
                new_shapes,
                recently_deleted,
                collision,
                debug,
            );

            return true;
        }

        if !pairs_to_check.is_empty() {
            if (debug) {
                println!("We have pairs to check!");
            }
            let (shape_a_id, shape_b_id) = pairs_to_check.pop_front().unwrap();

            let new_collisions =
                self.identify_collisions(temp_sketch, all_shapes, shape_a_id, shape_b_id, debug);

            if (debug) {
                println!("Adding {} collisions", new_collisions.len());
            }
            for c in new_collisions {
                // println!("Adding a collision");
                collisions.push_back(c);
            }
            return true;
        }

        if (debug) {
            println!("There was nothing to do!\n");
        }
        return false;
    }

    pub fn split_intersections(&self, debug: bool) -> Self {
        let mut temp_sketch = self.clone();

        // set up the necessary data structures:
        // First put all segments: Arcs, Lines, Circles into one big collection called all_shapes
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

        let mut pairs_to_check: VecDeque<(u64, u64)> = VecDeque::new();
        let mut collisions: VecDeque<Collision> = VecDeque::new();
        let mut possible_shape_collisions: Vec<u64> = vec![];
        let mut new_shapes: Vec<u64> = vec![];
        let mut recently_deleted: Vec<u64> = vec![];

        // inject all the pairs of shapes that need to be checked:
        for shape_id_a in all_shapes.items.keys() {
            for shape_id_b in all_shapes.items.keys() {
                if shape_id_a < shape_id_b {
                    pairs_to_check.push_back((*shape_id_a, *shape_id_b))
                }
            }
        }

        // While there's anything to do, step the process forward
        let mut count = 0;
        loop {
            if (debug) {
                println!("\nstep: {} Here's the setup:", count);
            }
            count += 1;

            if (debug) {
                println!("Pairs to check: {:?}", pairs_to_check);
                println!("Collisions: {:?}", collisions);
                println!("Possible shape collisions: {:?}", possible_shape_collisions);
                println!("New shapes: {:?}", new_shapes);
                println!("Recently deleted: {:?}", recently_deleted);
            }
            // let mut input = String::new();
            // io::stdin()
            //     .read_line(&mut input)
            //     .expect("error: unable to read user input");
            // println!("{}", input);

            let result = self.step_process(
                &mut temp_sketch,
                &mut all_shapes,
                &mut pairs_to_check,
                &mut collisions,
                &mut possible_shape_collisions,
                &mut new_shapes,
                &mut recently_deleted,
                debug,
            );
            // let mut input = String::new();
            // io::stdin()
            //     .read_line(&mut input)
            //     .expect("error: unable to read user input");
            // println!("{}", input);
            if result == false {
                break;
            }
        }

        // Lastly, consolidate all the shapes into a final sketch and return it
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
        if (debug) {
            println!("So, in summary I've generated these shapes:");
            for shape in all_shapes.items.iter() {
                println!("{:?}", shape);
            }
        }
        final_sketch
    }

    pub fn line_line_collisions(
        &self,
        line_a: &Line2,
        line_a_id: u64,
        line_b: &Line2,
        line_b_id: u64,
        debug: bool,
    ) -> Vec<Collision> {
        // catch the case where the lines are completely degenerate-their start and end points are the same
        if line_a.start == line_b.start || line_a.start == line_b.end {
            if line_a.end == line_b.start || line_a.end == line_b.end {
                return vec![Collision::degenerate(line_a_id, line_b_id)];
            }
        }

        let a_start = self.points.get(&line_a.start).unwrap();
        let a_end = self.points.get(&line_a.end).unwrap();
        let b_start = self.points.get(&line_b.start).unwrap();
        let b_end = self.points.get(&line_b.end).unwrap();

        fn normal_form(start: &Point2, end: &Point2) -> (f64, f64, f64) {
            let a = start.y - end.y;
            let b = end.x - start.x;
            let c = (start.x - end.x) * start.y + (end.y - start.y) * start.x;
            return (a, b, c);
        }

        let (a1, b1, c1) = normal_form(&a_start, &a_end);
        let (a2, b2, c2) = normal_form(&b_start, &b_end);

        if (debug) {
            println!("a1, b1, c1: {} {} {}", a1, b1, c1);
            println!("a2, b2, c2: {} {} {}", a2, b2, c2);
        }

        let x_intercept = (b1 * c2 - b2 * c1) / (a1 * b2 - a2 * b1);
        let y_intercept = (c1 * a2 - c2 * a1) / (a1 * b2 - a2 * b1);

        if (debug) {
            println!("Intercept: {} {}", x_intercept, y_intercept);
        }

        if x_intercept.is_nan() || y_intercept.is_nan() {
            // something degenerate happened. The lines may be parallel.
            if (debug) {
                println!("NAN intercept, so lines are be parallel");
            }

            let tol = 1e-8;
            let a_start_colinear_with_b = (a2 * a_start.x + b2 * a_start.y + c2).abs() < tol;
            let a_end_colinear_with_b = (a2 * a_end.x + b2 * a_end.y + c2).abs() < tol;

            if a_start_colinear_with_b && a_end_colinear_with_b {
                if (debug) {
                    println!("The lines are perfectly colinear!");
                }

                let mut collisions: Vec<Collision> = vec![];

                // it is possible these lines overlap somewhat.
                if within_range(a_start.x, b_start.x, b_end.x, -tol)
                    && within_range(a_start.y, b_start.y, b_end.y, -tol)
                {
                    // a_start is WITHIN b!
                    if (debug) {
                        println!("A start is within B");
                    }
                    let mut collision = Collision::new(a_start.clone(), line_a_id, line_b_id);
                    collision.shape_a_degeneracy = IsStart;
                    collisions.push(collision);
                }
                if within_range(a_end.x, b_start.x, b_end.x, -tol)
                    && within_range(a_end.y, b_start.y, b_end.y, -tol)
                {
                    // a_end is WITHIN b!
                    if (debug) {
                        println!("A end is within B");
                    }
                    let mut collision = Collision::new(a_end.clone(), line_a_id, line_b_id);
                    collision.shape_a_degeneracy = IsEnd;
                    collisions.push(collision);
                }

                if within_range(b_start.x, a_start.x, a_end.x, -tol)
                    && within_range(b_start.y, a_start.y, a_end.y, -tol)
                {
                    // b_start is WITHIN a!
                    if (debug) {
                        println!("B start is within A");
                    }
                    let mut collision = Collision::new(b_start.clone(), line_a_id, line_b_id);
                    collision.shape_b_degeneracy = IsStart;
                    collisions.push(collision);
                }
                if within_range(b_end.x, a_start.x, a_end.x, -tol)
                    && within_range(b_end.y, a_start.y, a_end.y, -tol)
                {
                    // b_end is WITHIN a!
                    if (debug) {
                        println!("B end is within A");
                    }
                    let mut collision = Collision::new(b_end.clone(), line_a_id, line_b_id);
                    collision.shape_b_degeneracy = IsEnd;
                    collisions.push(collision);
                }

                return collisions;
            } else {
                // If the lines are parallel but not colinear, then they can have no collisions
                return vec![];
            }
        }

        if x_intercept.is_infinite() || y_intercept.is_infinite() {
            if (debug) {
                println!("Infinite intercept, so lines are parallel");
            }
            return vec![];
        }

        // it only counts as an intersection if it falls within both the segments
        // Check that the x-intercept is within the x-range of the first segment
        let epsilon = 1e-12;
        if within_range(x_intercept, a_start.x, a_end.x, epsilon)
            && within_range(y_intercept, a_start.y, a_end.y, epsilon)
        {
            if within_range(x_intercept, b_start.x, b_end.x, epsilon)
                && within_range(y_intercept, b_start.y, b_end.y, epsilon)
            {
                let collision_point = Point2::new(x_intercept, y_intercept);
                let mut collision = Collision::new(collision_point.clone(), line_a_id, line_b_id);
                if points_almost_equal(&a_start, &collision_point) {
                    collision.shape_a_degeneracy = Degeneracy::IsStart;
                }
                if points_almost_equal(&a_end, &collision_point) {
                    collision.shape_a_degeneracy = Degeneracy::IsEnd;
                }
                if points_almost_equal(&b_start, &collision_point) {
                    collision.shape_b_degeneracy = Degeneracy::IsStart;
                }
                if points_almost_equal(&b_end, &collision_point) {
                    collision.shape_b_degeneracy = Degeneracy::IsEnd;
                }
                return vec![collision];
            }
        }

        vec![]
    }

    pub fn line_arc_collisions(
        &self,
        line_a: &Line2,
        line_a_id: u64,
        arc_b: &Arc2,
        arc_b_id: u64,
    ) -> Vec<Collision> {
        // treat this is circle/circle collision, then just do some checks
        // afterwards to make sure the collision points really do fall within
        // the bounds of the arc
        println!("LINE AGAINST ARC");
        let arc_center = self.points.get(&arc_b.center).unwrap();
        // println!("Getting arc start: {}", &arc.start);
        let arc_start = self.points.get(&arc_b.start).unwrap();
        let arc_dx = arc_center.x - arc_start.x;
        let arc_dy = arc_center.y - arc_start.y;
        let arc_radius = arc_dx.hypot(arc_dy);
        let fake_circle = Circle2 {
            center: arc_b.center,
            radius: arc_radius,
            top: arc_b.start,
        };

        println!("Fake circle: {:?}", &fake_circle);
        println!("Line Details:");
        let line_start = &self.points[&line_a.start];
        println!("start: {:?}", line_start);
        let line_end = &self.points[&line_a.end];
        println!("end: {:?}", line_end);

        let fake_collisions =
            self.line_circle_collisions(line_a, line_a_id, &fake_circle, arc_b_id);

        println!("Fake collisions: {:?}", fake_collisions);

        let mut real_collisions: Vec<Collision> = vec![];

        for c in fake_collisions {
            // check to make sure the point falls within the arc.
            if self.point_within_arc(arc_b, &c.point) {
                real_collisions.push(c);
            }
        }

        real_collisions
    }

    pub fn line_circle_collisions(
        &self,
        line_a: &Line2,
        line_a_id: u64,
        circle_b: &Circle2,
        circle_b_id: u64,
    ) -> Vec<Collision> {
        // to make the math easier, let's assume the circle's center point is the origin
        // let's translate the line's start and end points
        let mut start = self.points[&line_a.start].clone();
        let mut end = self.points[&line_a.end].clone();
        let center = self.points[&circle_b.center].clone();
        let r = circle_b.radius;
        println!("Radius as reported by circle: {}", r);
        start.x -= center.x;
        start.y -= center.y;
        end.x -= center.x;
        end.y -= center.y;

        // get the line in normal form
        let (a, b, c) = normal_form(&start, &end);

        println!("Line from {:?} to {:?}", &start, &end);
        println!("In normal form: {} {} {}", a, b, c);
        let (mut y1, mut y2, mut x1, mut x2);

        let dy = (end.y - start.y).abs();

        if a == 0.0 || dy < 1e-10 {
            println!("It's a horizontal line");
            // oh, it's a horizontal line! that makes the math easier
            y1 = -c / b;
            y2 = -c / b;

            println!("Y1 and Y2: {} {}", y1, y2);

            x1 = (r * r - y1 * y1).sqrt();
            x2 = -(r * r - y1 * y1).sqrt();

            // println!("X1 and X2: {} {}", x1, x2);
        } else {
            println!("It's not a special case");
            let det = a * a + b * b;
            let d = (a * a * ((r * r * det) - c * c)).sqrt();

            y1 = (-d - b * c) / det;
            // println!("y1 {}", y1);
            y2 = (d - b * c) / det;
            // println!("y2 {}", y2);

            x1 = -(b * y1 + c) / a;
            // println!("x1 {}", x1);
            x2 = -(b * y2 + c) / a;
            // println!("x2 {}", x2);
        }

        println!("In shifted coordinates:");
        println!("x1 y1 {} {}", x1, y1);
        println!("x2 y2 {} {}", x2, y2);

        if (x1.hypot(y1) - r).abs() > 1e-10 {
            panic!(
                "Somehow the radius is not equal! Got {} expected {}",
                x1.hypot(y1),
                r
            );
        }

        if (x2.hypot(y2) - r).abs() > 1e-10 {
            panic!(
                "Somehow the radius is not equal! Got {} expected {}",
                x2.hypot(y2),
                r
            );
        }

        y1 += center.y;
        y2 += center.y;
        x1 += center.x;
        x2 += center.x;
        start.x += center.x;
        start.y += center.y;
        end.x += center.x;
        end.y += center.y;

        println!("In real coordinates:");
        println!("x1 y1 {} {}", x1, y1);
        println!("x2 y2 {} {}", x2, y2);

        // println!("X1 Y1: {} {}", x1, y1);
        // println!("X2 Y2: {} {}", x2, y2);

        let mut valid_collisions: Vec<Collision> = vec![];

        let start = &self.points[&line_a.start];
        let end = &self.points[&line_a.end];

        let epsilon = 1e-10;

        println!("Checking that X {} is within {} {}", x1, start.x, end.x);
        println!("Checking that Y {} is within {} {}", y1, start.y, end.y);

        if within_range(x1, start.x, end.x, epsilon) && within_range(y1, start.y, end.y, epsilon) {
            println!("Added that point!");
            valid_collisions.push(Collision::new(Point2::new(x1, y1), line_a_id, circle_b_id));
        }

        println!("Checking that X {} is within {} {}", x2, start.x, end.x);
        println!("Checking that Y {} is within {} {}", y2, start.y, end.y);
        if within_range(x2, start.x, end.x, epsilon) && within_range(y2, start.y, end.y, epsilon) {
            println!("Added that point!");
            valid_collisions.push(Collision::new(Point2::new(x2, y2), line_a_id, circle_b_id));
        }

        valid_collisions
    }

    pub fn circle_circle_collisions(
        &self,
        circle_a: &Circle2,
        circle_a_id: u64,
        circle_b: &Circle2,
        circle_b_id: u64,
    ) -> Vec<Collision> {
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
            return vec![];
        }
        if center_dist < (r_a - r_b).abs() {
            return vec![];
        }

        let epsilon = 1e-10;
        if center_dist > r_a + r_b - epsilon && center_dist < r_a + r_b + epsilon {
            // draw a straight line from a to b, of length r_a
            let collision = Collision::new(
                Point2::new(
                    center_a.x + r_a * center_dx / center_dist,
                    center_a.y + r_a * center_dy / center_dist,
                ),
                circle_a_id,
                circle_b_id,
            );
            return vec![collision];
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

        let collision_a = Collision::new(Point2::new(ix1, iy1), circle_a_id, circle_b_id);

        let collision_b = Collision::new(Point2::new(ix2, iy2), circle_a_id, circle_b_id);

        return vec![collision_a, collision_b];
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

    pub fn circle_arc_collisions(
        &self,
        circle: &Circle2,
        circle_id: u64,
        arc: &Arc2,
        arc_id: u64,
    ) -> Vec<Collision> {
        // treat this is circle/circle collision, then just do some checks
        // afterwards to make sure the collision points really do fall within
        // the bounds of the arc
        let arc_center = self.points.get(&arc.center).unwrap();
        // println!("Getting arc start: {}", &arc.start);
        let arc_start = self.points.get(&arc.start).unwrap();
        let arc_dx = arc_center.x - arc_start.x;
        let arc_dy = arc_center.y - arc_start.y;
        let arc_radius = arc_dx.hypot(arc_dy);
        let fake_circle = Circle2 {
            center: arc.center,
            radius: arc_radius,
            top: arc.start,
        };

        let fake_collisions: Vec<Collision> =
            self.circle_circle_collisions(circle, circle_id, &fake_circle, arc_id);
        println!("Fake collision: {:?}", fake_collisions);

        let mut real_collisions: Vec<Collision> = vec![];

        for c in fake_collisions {
            // check to make sure the point falls within the arc.
            if self.point_within_arc(arc, &c.point) {
                real_collisions.push(c);
            }
        }

        real_collisions
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

    pub fn arc_arc_collisions(
        &self,
        arc_a: &Arc2,
        arc_a_id: u64,
        arc_b: &Arc2,
        arc_b_id: u64,
    ) -> Vec<Collision> {
        // treat this is circle/circle collision, then just do some checks
        // afterwards to make sure the collision points really do fall within
        // the bounds of the arc
        let arc_a_center = self.points.get(&arc_a.center).unwrap();
        // println!("Getting arc start: {}", &arc.start);
        let arc_a_start = self.points.get(&arc_a.start).unwrap();
        let arc_a_end = self.points.get(&arc_a.end).unwrap();
        let arc_a_dx = arc_a_center.x - arc_a_start.x;
        let arc_a_dy = arc_a_center.y - arc_a_start.y;
        let arc_a_radius = arc_a_dx.hypot(arc_a_dy);
        let fake_circle_a = Circle2 {
            center: arc_a.center,
            radius: arc_a_radius,
            top: arc_a.start,
        };

        let arc_b_center = self.points.get(&arc_b.center).unwrap();
        // println!("Getting arc start: {}", &arc.start);
        let arc_b_start = self.points.get(&arc_b.start).unwrap();
        let arc_b_end = self.points.get(&arc_b.end).unwrap();
        let arc_b_dx = arc_b_center.x - arc_b_start.x;
        let arc_b_dy = arc_b_center.y - arc_b_start.y;
        let arc_b_radius = arc_b_dx.hypot(arc_b_dy);
        let fake_circle_b = Circle2 {
            center: arc_b.center,
            radius: arc_b_radius,
            top: arc_b.start,
        };

        let mut forbidden_points: Vec<Point2> = vec![];
        if arc_a.start == arc_b.start || arc_a.start == arc_b.end {
            forbidden_points.push(arc_a_start.clone());
        }
        if arc_a.end == arc_b.end || arc_a.end == arc_b.start {
            forbidden_points.push(arc_a_end.clone());
        }

        let fake_collisions: Vec<Collision> =
            self.circle_circle_collisions(&fake_circle_a, arc_a_id, &fake_circle_b, arc_b_id);
        println!("Fake collisions: {:?}", fake_collisions);

        let mut real_collisions: Vec<Collision> = vec![];

        for c in fake_collisions {
            // check to make sure the point falls within both arcs.
            if self.point_within_arc(arc_a, &c.point) && self.point_within_arc(arc_b, &c.point) {
                // check to make sure the collision point is not approximately equal to any of
                // the start or end points

                let mut point_was_forbidden = false;
                for forbidden_point in forbidden_points.iter() {
                    if points_almost_equal(&c.point, forbidden_point) {
                        point_was_forbidden = true;
                        break;
                    }
                }

                if !point_was_forbidden {
                    real_collisions.push(c);
                } else {
                    println!("A point was forbidden! {:?}", &c.point);
                }
            }
        }

        real_collisions
    }

    pub fn split_circle_at_point(&self, circle: &Circle2, point_id: &u64, point: &Point2) -> Arc2 {
        // this converts a single circle into a single arc
        let new_arc = Arc2 {
            center: circle.center,
            start: *point_id,
            end: *point_id,
            clockwise: false,
        };

        new_arc
    }

    pub fn split_arc_at_point(&self, arc: &Arc2, point_id: &u64, point: &Point2) -> (Arc2, Arc2) {
        // this converts a single arc into a two arcs
        let new_arc_1 = Arc2 {
            center: arc.center,
            start: arc.start,
            end: *point_id,
            clockwise: arc.clockwise,
        };

        let new_arc_2 = Arc2 {
            center: arc.center,
            start: *point_id,
            end: arc.end,
            clockwise: arc.clockwise,
        };

        (new_arc_1, new_arc_2)
    }

    pub fn split_line_at_point(
        &self,
        line: &Line2,
        point_id: &u64,
        point: &Point2,
    ) -> (Line2, Line2) {
        let new_line_1 = Line2 {
            start: line.start,
            end: *point_id,
        };

        let new_line_2 = Line2 {
            start: *point_id,
            end: line.end,
        };

        (new_line_1, new_line_2)
    }
}

pub fn points_almost_equal(point_a: &Point2, point_b: &Point2) -> bool {
    let dx = (point_b.x - point_a.x).abs();
    let dy = (point_b.y - point_a.y).abs();
    dx < 1e-10 && dy < 1e-10
}

fn normal_form(start: &Point2, end: &Point2) -> (f64, f64, f64) {
    // finds the normal form of a line:
    // ax + by + c = 0
    let a = start.y - end.y;
    let b = end.x - start.x;
    let c = (start.x - end.x) * start.y + (end.y - start.y) * start.x;
    return (a, b, c);
}

fn within_range(x: f64, a: f64, b: f64, epsilon: f64) -> bool {
    if a == b && b == x {
        return true;
    }
    let mut retval;
    if a < b {
        retval = x >= a - epsilon && x <= b + epsilon;
    } else {
        retval = x >= b - epsilon && x <= a + epsilon;
    }
    retval
}

#[cfg(test)]
mod tests {
    use crate::archetypes::Plane;
    use crate::project::Project;
    use crate::workbench::Workbench;

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
    fn four_circles() {
        // three intersecting circles should yield 5 extrudable faces
        let contents = std::fs::read_to_string(
            "src/test_inputs/sketches/circle_circle/four_circles_chained.cadmium",
        )
        .unwrap();
        let p: Project = serde_json::from_str(&contents).unwrap();

        let realized = p.get_realization(0, 1000);
        let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();

        println!("Number of faces: {:?}", sketch_split.faces.len());
        assert_eq!(sketch_split.faces.len(), 7);
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
        assert_eq!(sketch_split.faces.len(), 5);
    }

    #[test]
    fn circle_rectangle() {
        // a circle that intersects with a rectangle on two lines
        // let contents = std::fs::read_to_string(
        //     "src/test_inputs/sketches/circle_line/circle_rectangle.cadmium",
        // )
        // .unwrap();
        // let p: Project = serde_json::from_str(&contents).unwrap();

        // let realized = p.get_realization(0, 1000);
        // let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();

        // println!("Number of faces: {:?}", sketch_split.faces.len());
        // assert_eq!(sketch_split.faces.len(), 3);

        let contents = std::fs::read_to_string(
            "src/test_inputs/sketches/circle_line/circle_rect_changing_size.cadmium",
        )
        .unwrap();
        let p: Project = serde_json::from_str(&contents).unwrap();

        let realized = p.get_realization(0, 1000);
        let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();

        println!("Number of faces: {:?}", sketch_split.faces.len());
        assert_eq!(sketch_split.faces.len(), 3);
    }

    #[test]
    fn circle_quadrangle() {
        // a circle that intersects with a rectangle on two lines
        let contents = std::fs::read_to_string(
            "src/test_inputs/sketches/circle_line/circle_quadrangle.cadmium",
        )
        .unwrap();
        let p: Project = serde_json::from_str(&contents).unwrap();

        let realized = p.get_realization(0, 1000);
        let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();

        println!("Number of faces: {:?}", sketch_split.faces.len());
        assert_eq!(sketch_split.faces.len(), 3);
    }

    #[test]
    fn circle_rect_circle() {
        // a circle that intersects with a rectangle on two lines
        let contents = std::fs::read_to_string(
            "src/test_inputs/sketches/circle_line/circle_rect_circle.cadmium",
        )
        .unwrap();
        let p: Project = serde_json::from_str(&contents).unwrap();

        let realized = p.get_realization(0, 1000);
        let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();

        println!("Number of faces: {:?}", sketch_split.faces.len());
        assert_eq!(sketch_split.faces.len(), 5);
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
    fn circle_circle_collisions() {
        let mut sketch = Sketch::new();

        // two touching normally
        println!("two circles touching normally at two points");
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
        let collisions = sketch.circle_circle_collisions(&circle_a, 7, &circle_b, 8);
        assert_eq!(
            collisions,
            vec![
                Collision::new(Point2::new(0.5, -0.8660254037844386), 7, 8,),
                Collision::new(Point2::new(0.5, 0.8660254037844386), 7, 8,)
            ]
        );

        println!("Two circles touching at one point");
        let a_radius = 2.0;
        let a = sketch.add_point(0.0, 0.0);
        let a_top = sketch.add_point(0.0, a_radius);
        let b_radius = 3.0;
        let b = sketch.add_point(a_radius + b_radius, 0.0);
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
        let collisions = sketch.circle_circle_collisions(&circle_a, 7, &circle_b, 8);
        assert_eq!(
            collisions,
            vec![Collision::new(Point2::new(2.0, 0.0), 7, 8,)]
        );

        println!("Two circles not touching--too far away");
        let a_radius = 2.0;
        let a = sketch.add_point(0.0, 0.0);
        let a_top = sketch.add_point(0.0, a_radius);
        let b_radius = 3.0;
        let b = sketch.add_point(a_radius + b_radius + 1.0, 0.0);
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
        let collisions = sketch.circle_circle_collisions(&circle_a, 7, &circle_b, 8);
        assert_eq!(collisions, vec![]);

        println!("Two circles not touching--too close");
        let a_radius = 2.0;
        let a = sketch.add_point(0.0, 0.0);
        let a_top = sketch.add_point(0.0, a_radius);
        let b_radius = 3.0;
        let b = sketch.add_point(0.5, 0.0);
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
        let collisions = sketch.circle_circle_collisions(&circle_a, 7, &circle_b, 8);
        assert_eq!(collisions, vec![]);
    }

    #[test]
    fn line_line_overlap_tests() {
        let all_tests = [
            [
                /*
                        |--------|
                        |        |
                |--------|        |
                |        |        |
                |        |--------|
                |        |
                |--------|
                */
                [0.0, 0.0],
                [2.0, 0.0],
                [2.0, 2.0],
                [0.0, 2.0],

                [2.0, 1.0],
                [4.0, 1.0],
                [4.0, 3.0],
                [2.0, 3.0],

                [2.0, 9.0]
            ], [
                /*
                |--------|
                |        |----|
                |        |    |
                |        |----|
                |--------|
                */
                [0.0, 0.0],
                [2.0, 0.0],
                [2.0, 2.0],
                [0.0, 2.0],

                [2.0, 0.5],
                [3.0, 0.5],
                [3.0, 1.5],
                [2.0, 1.5],

                [2.0, 9.0]
            ], [
                /*
                |--------|----|
                |        |    |
                |        |----|
                |        |
                |--------|
                */
                [0.0, 0.0],
                [0.0, -2.0],
                [2.0, -2.0],
                [2.0, 0.0],

                [3.0, 0.0],
                [3.0, -1.0],
                [2.0, -1.0],
                [2.0, 0.0],

                [2.0, 8.0]
            ]
        ];

        for test in all_tests {
            let mut p = Project::new("A");
            let mut w = p.workbenches.get_mut(0).unwrap();

            let top_plane_id = w.get_first_plane_id().unwrap();
            let sketch_id = w.add_sketch_to_plane("Sketch 1", &top_plane_id);
            let sketch = w.get_sketch_mut("Sketch 1").unwrap();

            let a = sketch.add_point(test[0][0], test[0][1]);
            let b = sketch.add_point(test[1][0], test[1][1]);
            let c = sketch.add_point(test[2][0], test[2][1]);
            let d = sketch.add_point(test[3][0], test[3][1]);

            let e = sketch.add_point(test[4][0], test[4][1]);
            let f = sketch.add_point(test[5][0], test[5][1]);
            let g = sketch.add_point(test[6][0], test[6][1]);
            let h = if test[7] == test[3] {
                // For the last test, we want to make sure the line is closed
                d.clone()
            } else {
                sketch.add_point(test[7][0], test[7][1])
            };

            // big one
            sketch.add_segment(a, b);
            sketch.add_segment(b, c);
            sketch.add_segment(c, d);
            sketch.add_segment(d, a);

            // small one
            sketch.add_segment(e, f);
            sketch.add_segment(f, g);
            sketch.add_segment(g, h);
            sketch.add_segment(h, e);

            let realized = p.get_realization(0, 1000);
            let (sketch_unsplit, sketch_split, _) = realized.sketches.get("Sketch-0").unwrap();

            println!("Number of faces: {:?}", sketch_split.faces.len());
            assert_eq!(sketch_split.faces.len(), test[8][0] as usize);

            assert_eq!(sketch_split.line_segments.len(), test[8][1] as usize);
        }
    }

    #[test]
    fn line_circle_collisions() {
        let mut sketch = Sketch::new();

        println!("simple crossing point horizontal line");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(2.0, 0.0);
        let c = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let circle_a = Circle2 {
            center: a,
            radius: 1.0,
            top: c,
        };
        let collisions = sketch.line_circle_collisions(&line_ab, 1, &circle_a, 2);
        assert_eq!(
            collisions,
            vec![Collision::new(Point2::new(1.0, 0.0), 1, 2,)]
        );

        println!("simple crossing point horizontal line but backwards");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(-2.0, 0.0);
        let c = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let circle_a = Circle2 {
            center: a,
            radius: 1.0,
            top: c,
        };
        let collisions = sketch.line_circle_collisions(&line_ab, 1, &circle_a, 2);
        assert_eq!(
            collisions,
            vec![Collision::new(Point2::new(-1.0, 0.0), 1, 2,)]
        );

        println!("simple crossing point 45 degree angle");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(2.0, 2.0);
        let c = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let circle_a = Circle2 {
            center: a,
            radius: 1.0,
            top: c,
        };
        let collisions = sketch.line_circle_collisions(&line_ab, 1, &circle_a, 2);
        assert_eq!(
            collisions,
            vec![Collision::new(
                Point2::new(0.7071067811865476, 0.7071067811865476), // sqrt(2)/2
                1,
                2,
            )]
        );

        println!("simple crossing point 45 degree angle but away from origin");
        let a = sketch.add_point(10.0, 10.0);
        let b = sketch.add_point(12.0, 12.0);
        let c = sketch.add_point(10.0, 11.0);
        let line_ab = Line2 { start: a, end: b };
        let circle_a = Circle2 {
            center: a,
            radius: 1.0,
            top: c,
        };
        let collisions = sketch.line_circle_collisions(&line_ab, 1, &circle_a, 2);
        assert_eq!(
            collisions,
            vec![Collision::new(
                Point2::new(10.7071067811865476, 10.7071067811865476),
                1,
                2,
            )]
        );

        println!("simple crossing point vertical line");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(0.0, 2.0);
        let c = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let circle_a = Circle2 {
            center: a,
            radius: 1.0,
            top: c,
        };
        let collisions = sketch.line_circle_collisions(&line_ab, 1, &circle_a, 2);
        assert_eq!(
            collisions,
            vec![Collision::new(Point2::new(0.0, 1.0), 1, 2,)]
        );
    }

    #[test]
    fn line_line_collisions() {
        let mut sketch = Sketch::new();

        // simple cross
        println!("simple cross");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(0.0, -1.0);
        let d = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        assert_eq!(
            collisions,
            vec![Collision::new(Point2::new(0.0, 0.0), 1, 2,)]
        );

        // a T
        println!("an upside down T");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(0.0, 0.0);
        let d = sketch.add_point(0.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        let mut expected_collision = Collision::new(Point2::new(0.0, 0.0), 1, 2);
        expected_collision.shape_b_degeneracy = IsStart;

        // println!("Got this collision: {:?}", collisions[0]);
        assert_eq!(collisions, vec![expected_collision]);

        // parallel horizontal
        println!("parallel horizontal");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(-1.0, 1.0);
        let d = sketch.add_point(1.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        assert_eq!(collisions, vec![]);

        // parallel vertical
        println!("parallel vertical");
        let a = sketch.add_point(0.0, -1.0);
        let b = sketch.add_point(0.0, 1.0);
        let c = sketch.add_point(1.0, -1.0);
        let d = sketch.add_point(1.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        assert_eq!(collisions, vec![]);

        // perpendicular but not intersecting
        println!("perpendicular but not intersecting");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(3.0, 0.0);
        let d = sketch.add_point(3.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        assert_eq!(collisions, vec![]);

        // share 1 point but only in the === sense not the == sense
        println!("share 1 point but only in the === sense not the == sense");
        let a = sketch.add_point(-1.0, 1.0);
        let b = sketch.add_point(0.0, 0.0);
        let c = sketch.add_point(0.0, 0.0);
        let d = sketch.add_point(1.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        let mut expected_collision = Collision::new(Point2::new(0.0, 0.0), 1, 2);
        expected_collision.shape_a_degeneracy = IsEnd;
        expected_collision.shape_b_degeneracy = IsStart;
        assert_eq!(collisions, vec![expected_collision]);

        // share 1 point in the == sense
        println!("share 1 point in the == sense");
        let a = sketch.add_point(-1.0, 1.0);
        let b = sketch.add_point(0.0, 0.0);
        let d = sketch.add_point(1.0, 1.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: b, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        let mut expected_collision = Collision::new(Point2::new(0.0, 0.0), 1, 2);
        expected_collision.shape_a_degeneracy = IsEnd;
        expected_collision.shape_b_degeneracy = IsStart;
        assert_eq!(collisions, vec![expected_collision]);

        // colinear, horizontal no intersection
        println!("colinear horizontal no intersection");
        let a = sketch.add_point(-1.0, 0.0);
        let b = sketch.add_point(0.0, 0.0);
        let c = sketch.add_point(1.0, 0.0);
        let d = sketch.add_point(2.0, 0.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        assert_eq!(collisions, vec![]);

        // colinear, vertical no intersection
        println!("colinear vertical no intersection");
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(0.0, 1.0);
        let c = sketch.add_point(0.0, 2.0);
        let d = sketch.add_point(0.0, 3.0);
        let line_ab = Line2 { start: a, end: b };
        let line_cd = Line2 { start: c, end: d };
        let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        assert_eq!(collisions, vec![]);

        // Lines are exactly equal
        // println!("Exactly equal");
        // let a = sketch.add_point(0.0, 0.0);
        // let b = sketch.add_point(0.0, 1.0);
        // let line_ab = Line2 { start: a, end: b };
        // let collisions = sketch.line_line_collisions(&line_ab, 1, &line_ab, 2, false);
        // assert_eq!(
        //     collisions,
        //     Intersection::Line(Point2::new(0.0, 0.0), Point2::new(0.0, 1.0))
        // );

        // println!("\nLine Overlap:");
        // // lines overlap somewhat, both vertical
        // println!("lines overlap somewhat, both vertical");
        // let a = sketch.add_point(0.0, 0.0);
        // let b = sketch.add_point(0.0, 2.0);
        // let c = sketch.add_point(0.0, 1.0);
        // let d = sketch.add_point(0.0, 3.0);
        // let line_ab = Line2 { start: a, end: b };
        // let line_cd = Line2 { start: c, end: d };
        // let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2, false);
        // assert_eq!(
        //     collisions,
        //     Intersection::Line(Point2::new(0.0, 2.0), Point2::new(0.0, 1.0))
        // );
        // for future reference: the ordering of points here and for all of the tests below is inconsequential
        // Feel free to swap the order here if the implementation changes. Maybe these should always come
        // in a canonical order?

        // lines overlap somewhat, both horizontal
        // println!("lines overlap somewhat, both horizontal");
        // let a = sketch.add_point(0.0, 0.0);
        // let b = sketch.add_point(2.0, 0.0);
        // let c = sketch.add_point(1.0, 0.0);
        // let d = sketch.add_point(3.0, 0.0);
        // let line_ab = Line2 { start: a, end: b };
        // let line_cd = Line2 { start: c, end: d };
        // let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2);
        // assert_eq!(
        //     collisions,
        //     Intersection::Line(Point2::new(2.0, 0.0), Point2::new(1.0, 0.0))
        // );

        // // lines overlap somewhat, both angled
        // println!("lines overlap somewhat, both angled");
        // let a = sketch.add_point(0.0, 0.0);
        // let b = sketch.add_point(2.0, 2.0);
        // let c = sketch.add_point(1.0, 1.0);
        // let d = sketch.add_point(3.0, 3.0);
        // let line_ab = Line2 { start: a, end: b };
        // let line_cd = Line2 { start: c, end: d };
        // let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2);
        // assert_eq!(
        //     collisions,
        //     Intersection::Line(Point2::new(2.0, 2.0), Point2::new(1.0, 1.0))
        // );

        // one line engulfs the other, both angled
        // println!("one line engulfs the other, both angled");
        // let a = sketch.add_point(1.0, 1.0);
        // let b = sketch.add_point(2.0, 2.0);
        // let c = sketch.add_point(0.0, 0.0);
        // let d = sketch.add_point(3.0, 3.0);
        // let line_ab = Line2 { start: a, end: b };
        // let line_cd = Line2 { start: c, end: d };
        // let collisions = sketch.line_line_collisions(&line_ab, 1, &line_cd, 2);
        // assert_eq!(
        //     collisions,
        //     Intersection::Line(Point2::new(1.0, 1.0), Point2::new(2.0, 2.0))
        // );
    }
}
