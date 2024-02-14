#![allow(dead_code, unused)]
// use cadmium::project::Project;

use std::ops::{Sub, SubAssign};

use truck_meshalgo::filters::OptimizingFilter;
use truck_meshalgo::tessellation::{MeshableShape, MeshedShape};
use truck_modeling::builder::{translated, tsweep, vertex};
use truck_modeling::{Plane, Point3, Surface, Vector3};
use truck_polymesh::{obj, InnerSpace, Invertible, ParametricSurface, Tolerance};
use truck_shapeops::{and, or, ShapeOpsCurve, ShapeOpsSurface};
use truck_topology::Solid;

fn main() {
    let point_a = vertex(Point3::new(0.0, 0.0, 0.0));
    let line_a = tsweep(&point_a, Vector3::new(1.0, 0.0, 0.0));
    let square_a = tsweep(&line_a, Vector3::new(0.0, 1.0, 0.0));
    let cube_a = tsweep(&square_a, Vector3::new(0.0, 0.0, 1.0));

    let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    let line_b = tsweep(&point_b, Vector3::new(0.2, 0.0, 0.0));
    let square_b = tsweep(&line_b, Vector3::new(0.0, 0.2, 0.0));
    let cube_b: Solid<
        truck_meshalgo::prelude::cgmath::Point3<f64>,
        truck_modeling::Curve,
        truck_modeling::Surface,
    > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

    // extend the cube to be just 0.01 longer than it needs to be
    // let cube_b = tsweep(&square_b, Vector3::new(0.0, 0.0, 1.01));
    // let bad_volume = tsweep(&square_b, Vector3::new(0.0, 0.0, -0.01));
    // then translate it down
    // let cube_b = translated(&cube_b, Vector3::new(0.0, 0.0, -0.01));
    // let combined_big = or(&cube_a, &cube_b, 0.01).unwrap();

    // let combined = or(&cube_a, &cube_b, 0.01).unwrap();
    let combined = fuse(&cube_a, &cube_b);

    // println!(
    //     "combined_cube_or has {:?} shell boundaries",
    //     combined.boundaries().len()
    // );

    // let mut mesh = combined.triangulation(0.01).to_polygon();
    // mesh.put_together_same_attrs();
    // let file = std::fs::File::create("combined_cube.obj").unwrap();
    // obj::write(&mesh, file).unwrap();
}

pub fn fuse<C: ShapeOpsCurve<S> + std::fmt::Debug, S: ShapeOpsSurface + std::fmt::Debug>(
    solid0: &Solid<Point3, C, Surface>,
    solid1: &Solid<Point3, C, Surface>,
) -> Option<Solid<Point3, C, S>> {
    println!("Okay let's fuse!");

    let solid0_boundaries = solid0.boundaries();
    let solid1_boundaries = solid1.boundaries();
    assert!(solid0_boundaries.len() == 1);
    assert!(solid1_boundaries.len() == 1);

    for (face_0_idx, face_0) in solid0_boundaries[0].face_iter().enumerate() {
        println!("face 0: {:?}", face_0_idx);
        let surface_0 = face_0.oriented_surface();

        match surface_0 {
            Surface::Plane(p0) => {
                for (face_1_idx, face_1) in solid1_boundaries[0].face_iter().enumerate() {
                    print!("\tface 1: {:?}", face_1_idx);
                    let surface_1 = face_1.oriented_surface().inverse();

                    match surface_1 {
                        Surface::Plane(p1) => {
                            if are_coplanar(p0, p1) {
                                println!(" coplanar!");
                            } else {
                                println!(" not coplanar!");
                            }
                            // print!(" {:?} vs {:?}", normal0, normal1);
                            // println!("{}", normal0.near(&normal1));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    None
}

fn are_coplanar(p0: Plane, p1: Plane) -> bool {
    let normal0 = p0.normal();
    let normal1 = p1.normal();

    if !normal0.near(&normal1) {
        return false;
    }

    let difference = p0.origin() - p1.origin();
    let dot = normal0.dot(difference);
    dot.abs() < 0.0001
}
