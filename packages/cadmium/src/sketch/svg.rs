use crate::sketch::{Arc2, Circle2, IncrementingMap, Line2, Point2, Ring, Segment, Sketch};
use std::f64::consts::PI;

use svg::node::element::path::Data;
// use svg::node::element::Circle;
use std::fs;
use svg::node::element::Path;
use svg::Document;

impl Sketch {
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

        // println!("Making SVG. Faces:");
        // for face in faces.iter() {
        //     println!("{:?}", face);
        // }
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

                            let arc_angle_degrees = self.arc_angle(arc) * 180.0 / PI;
                            println!("arc_angle: {}", arc_angle_degrees);

                            // most small simple arcs should have this flag set to 0
                            let mut large_arc_flag = 0;
                            // most arcs are counterclockwise, so this flag is usually 0
                            let mut sweep_flag = 0;

                            if arc_angle_degrees > 180.0 {
                                println!("large arc flag!");
                                large_arc_flag = 1;
                            }

                            if arc.clockwise {
                                sweep_flag = 1;
                            }

                            //A rx ry x-axis-rotation large-arc-flag sweep-flag x y
                            data = data.elliptical_arc_to((
                                r,
                                r,
                                0.0,
                                large_arc_flag,
                                sweep_flag,
                                end.x,
                                -end.y,
                            ));
                        }
                    }
                }
                data
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::project::Project;

    use super::*;

    #[test]
    fn empty_to_svg() {
        let mut sketch = Sketch::new();
        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/empty.svg");
    }

    #[test]
    fn no_rings_to_svg() {
        let mut sketch = Sketch::new();

        let center = sketch.add_point(0.0, 0.0);
        let right = sketch.add_point(1.0, 0.0);
        let top = sketch.add_point(0.0, 1.0);
        let left = sketch.add_point(-1.0, 0.0);
        let bottom = sketch.add_point(0.0, -1.0);

        sketch.add_segment(center, right);
        sketch.add_segment(center, top);
        sketch.add_segment(center, left);
        sketch.add_segment(center, bottom);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/no_rings.svg");
    }

    #[test]
    fn circle_to_svg() {
        let mut sketch = Sketch::new();

        let id0 = sketch.add_point(1.0, 0.0);
        sketch.add_circle(id0, 1.0);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/circle.svg");
    }

    #[test]
    fn square_to_svg() {
        let mut sketch = Sketch::new();

        let id0 = sketch.add_point(0.0, 0.0);
        let id1 = sketch.add_point(1.0, 0.0);
        let id2 = sketch.add_point(1.0, 1.0);
        let id3 = sketch.add_point(0.0, 1.0);

        sketch.add_segment(id0, id1);
        sketch.add_segment(id1, id2);
        sketch.add_segment(id2, id3);
        sketch.add_segment(id3, id0);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/square.svg");
    }

    #[test]
    fn rounded_square_to_svg() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.25, 0.0);
        let b = sketch.add_point(0.75, 0.0);
        let c = sketch.add_point(1.0, 0.25);
        let d = sketch.add_point(1.0, 0.75);
        let e = sketch.add_point(0.75, 1.0);
        let f = sketch.add_point(0.25, 1.0);
        let g = sketch.add_point(0.0, 0.75);
        let h = sketch.add_point(0.0, 0.25);
        let i = sketch.add_point(0.75, 0.25);
        let j = sketch.add_point(0.75, 0.75);
        let k = sketch.add_point(0.25, 0.75);
        let l = sketch.add_point(0.25, 0.25);

        sketch.add_segment(a, b);
        sketch.add_arc(i, b, c, false);
        sketch.add_segment(c, d);
        sketch.add_arc(j, d, e, false);
        sketch.add_segment(e, f);
        sketch.add_arc(k, f, g, false);
        sketch.add_segment(g, h);
        sketch.add_arc(l, h, a, false);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/rounded_square.svg");
    }

    #[test]
    fn square_with_hole_to_svg() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(1.0, 1.0);
        let d = sketch.add_point(0.0, 1.0);

        let e = sketch.add_point(0.25, 0.25);
        let f = sketch.add_point(0.75, 0.25);
        let g = sketch.add_point(0.75, 0.75);
        let h = sketch.add_point(0.25, 0.75);

        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        sketch.add_segment(c, d);
        sketch.add_segment(d, a);

        sketch.add_segment(e, f);
        sketch.add_segment(f, g);
        sketch.add_segment(g, h);
        sketch.add_segment(h, e);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/square_with_hole.svg");
    }

    #[test]
    fn square_with_circular_hole_to_svg() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(1.0, 1.0);
        let d = sketch.add_point(0.0, 1.0);
        let center = sketch.add_point(0.5, 0.5);

        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        sketch.add_segment(c, d);
        sketch.add_segment(d, a);

        sketch.add_circle(center, 0.4);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/square_with_circular_hole.svg");
    }

    #[test]
    fn circle_with_circular_hole_to_svg() {
        let mut sketch = Sketch::new();

        let center = sketch.add_point(0.5, 0.5);

        sketch.add_circle(center, 0.5);
        sketch.add_circle(center, 0.25);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/circle_with_circular_hole.svg");
    }

    #[test]
    fn circle_with_square_hole_to_svg() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(1.0, 1.0);
        let d = sketch.add_point(0.0, 1.0);
        let center = sketch.add_point(0.5, 0.5);

        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        sketch.add_segment(c, d);
        sketch.add_segment(d, a);

        sketch.add_circle(center, 1.0);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/circle_with_square_hole.svg");
    }

    #[test]
    fn two_intersecting_squares_to_svg() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(1.0, 1.0);
        let d = sketch.add_point(0.0, 1.0);
        sketch.add_segment(a, b);
        sketch.add_segment(b, c);
        sketch.add_segment(c, d);
        sketch.add_segment(d, a);

        let e = sketch.add_point(0.5, 0.5);
        let f = sketch.add_point(1.5, 0.5);
        let g = sketch.add_point(1.5, 1.5);
        let h = sketch.add_point(0.5, 1.5);
        sketch.add_segment(e, f);
        sketch.add_segment(f, g);
        sketch.add_segment(g, h);
        sketch.add_segment(h, e);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/two_intersecting_squares_unsplit.svg");

        let sketch = sketch.split_intersections(false);
        sketch.save_svg("test_svgs/two_intersecting_squares_split.svg");
    }

    #[test]
    fn two_intersecting_circles_to_svg() {
        // Create a new sketch
        let mut sketch = Sketch::new();

        // Add two circles which happen to intersect
        let center_a = sketch.add_point(0.0, 0.0);
        sketch.add_circle(center_a, 1.0);
        let center_b = sketch.add_point(1.0, 0.0);
        sketch.add_circle(center_b, 1.0);

        // Save the naive svg: just two circular paths
        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/two_intersecting_circles_unsplit.svg");

        // Split the intersections, creating a new and different sketch
        let sketch = sketch.split_intersections(false);

        // Save this one as an SVG, it will have three non-overlapping paths of two arcs each
        sketch.save_svg("test_svgs/two_intersecting_circles_split.svg");
    }

    #[test]
    fn circle_diameter() {
        let mut sketch = Sketch::new();

        let center = sketch.add_point(0.0, 0.0);
        let circle_id = sketch.add_circle(center, 0.5);
        let constraint_id = sketch.add_circle_diameter_constraint(circle_id, 4.0);

        sketch.solve(4000);

        println!("Value: {}", sketch.constraint_value(constraint_id));
        println!("Error: {}", sketch.constraint_error(constraint_id));
        assert!(sketch.constraint_is_satisfied(constraint_id));

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/constraint_circle_diameter.svg");
    }

    #[test]
    fn segments_equal() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(0.5, 0.0);

        let c = sketch.add_point(0.0, 1.0);
        let d = sketch.add_point(1.0, 1.0);

        let segment_ab = sketch.add_segment(a, b); // length 0.5
        let segment_cd = sketch.add_segment(c, d); // length 1.0

        let constraint_id = sketch.add_segments_equal_constraint(segment_ab, segment_cd);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/equality_constraint_unsolved.svg");
        assert!(sketch.solve(1000));
        sketch.save_svg("test_svgs/equality_constraint_solved.svg");
        println!("equality error: {}", sketch.constraint_error(constraint_id));
    }

    #[test]
    fn triangle_constraint() {
        let mut sketch = Sketch::new();

        // initialized as a right triangle with right angle at origin
        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(0.0, 1.0);

        let segment_ab = sketch.add_segment(a, b);
        let segment_bc = sketch.add_segment(b, c);
        let segment_ca = sketch.add_segment(c, a);

        let constraint_ab = sketch.add_segment_length_constraint(segment_ab, 2.0);
        let constraint_bc = sketch.add_segment_length_constraint(segment_bc, 2.0);
        let constraint_ca = sketch.add_segment_length_constraint(segment_ca, 2.0);

        assert!(sketch.solve(1000));

        assert!(sketch.all_constraints_are_satisfied());

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/constraint_triangle.svg");
    }

    #[test]
    fn two_arcs_in_a_circle_90() {
        let mut sketch = Sketch::new();

        let center = sketch.add_point(0.0, 0.0);
        let top = sketch.add_point(0.0, 1.0);
        let right = sketch.add_point(1.0, 0.0);

        sketch.add_arc(center, right, top, false);
        sketch.add_arc(center, top, right, false);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/two_arcs_in_a_circle_90.svg");
    }

    #[test]
    fn two_arcs_in_a_circle_180() {
        let mut sketch = Sketch::new();

        let center = sketch.add_point(0.0, 0.0);
        let top = sketch.add_point(0.0, 1.0);
        let bottom = sketch.add_point(0.0, -1.0);

        sketch.add_arc(center, bottom, top, false);
        sketch.add_arc(center, top, bottom, false);

        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/two_arcs_in_a_circle_180.svg");
    }

    #[test]
    fn manual_square() {
        let mut sketch = Sketch::new();

        let a = sketch.add_fixed_point(0.0, 0.0);
        let b = sketch.add_point(1.0, -0.1);
        let c = sketch.add_point(1.1, 0.9);
        let d = sketch.add_point(-0.1, 0.9);

        let segment_ab = sketch.add_segment(a, b);
        let segment_bc = sketch.add_segment(b, c);
        let segment_cd = sketch.add_segment(c, d);
        let segment_da = sketch.add_segment(d, a);

        let length = 2.0;
        sketch.add_segment_length_constraint(segment_ab, length);
        sketch.add_segment_length_constraint(segment_bc, length);
        sketch.add_segment_length_constraint(segment_cd, length);
        sketch.add_segment_length_constraint(segment_da, length);

        sketch.add_segment_horizontal_constraint(segment_ab);
        sketch.add_segment_horizontal_constraint(segment_cd);
        sketch.add_segment_vertical_constraint(segment_da);
        sketch.add_segment_vertical_constraint(segment_bc);

        // for i in 0..100 {
        //     sketch.step();
        //     sketch.save_svg(&format!("test_svgs/manual_square/{}.svg", i));
        // }

        sketch.solve(1000);
        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/manual_square_solved.svg");
    }

    #[test]
    fn manual_rectangle() {
        let mut sketch = Sketch::new();

        let a = sketch.add_point(0.0, 0.0);
        let b = sketch.add_point(1.0, 0.0);
        let c = sketch.add_point(1.0, 1.0);
        let d = sketch.add_point(0.0, 1.0);

        let segment_ab = sketch.add_segment(a, b);
        let segment_bc = sketch.add_segment(b, c);
        let segment_cd = sketch.add_segment(c, d);
        let segment_da = sketch.add_segment(d, a);

        sketch.add_segment_horizontal_constraint(segment_ab);
        sketch.add_segment_horizontal_constraint(segment_cd);
        sketch.add_segment_vertical_constraint(segment_da);
        sketch.add_segment_vertical_constraint(segment_bc);

        // fixed width of 1.0
        sketch.add_segment_length_constraint(segment_ab, 1.0);
        sketch.add_segment_length_constraint(segment_cd, 1.0);
        // This should cause it to adjust!
        sketch.add_segment_length_constraint(segment_da, 0.5);

        // for i in 0..800 {
        //     sketch.save_svg(&format!("test_svgs/manual_square/{}.svg", i));
        //     sketch.step();
        // }

        let solved = sketch.solve(1000);
        println!("did solve? {}", solved);
        fs::create_dir_all("test_svgs");
        sketch.save_svg("test_svgs/manual_rectangle_solved.svg");
    }
}
