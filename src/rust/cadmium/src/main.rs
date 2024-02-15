#![allow(dead_code, unused)]

use std::ops::{Sub, SubAssign};

use truck_meshalgo::filters::OptimizingFilter;
use truck_meshalgo::tessellation::{MeshableShape, MeshedShape};
use truck_modeling::builder::{translated, tsweep, vertex};
use truck_modeling::{Plane, Point3, Surface, Vector3};
use truck_polymesh::{obj, InnerSpace, Invertible, ParametricSurface, Tolerance};
use truck_shapeops::{and, or, ShapeOpsCurve, ShapeOpsSurface};
use truck_topology::{Shell, Solid};

fn main() {
    let point_a = vertex(Point3::new(0.0, 0.0, 0.0));
    let line_a = tsweep(&point_a, Vector3::new(1.0, 0.0, 0.0));
    let square_a = tsweep(&line_a, Vector3::new(0.0, 1.0, 0.0));
    let cube_a = tsweep(&square_a, Vector3::new(0.0, 0.0, 1.0));

    // simplest case!
    let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    let line_b = tsweep(&point_b, Vector3::new(0.2, 0.0, 0.0));
    let square_b = tsweep(&line_b, Vector3::new(0.0, 0.2, 0.0));
    let cube_b: Solid<
        truck_meshalgo::prelude::cgmath::Point3<f64>,
        truck_modeling::Curve,
        truck_modeling::Surface,
    > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

    // one flush side!
    // let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    // let line_b = tsweep(&point_b, Vector3::new(0.6, 0.0, 0.0));
    // let square_b = tsweep(&line_b, Vector3::new(0.0, 0.2, 0.0));
    // let cube_b: Solid<
    //     truck_meshalgo::prelude::cgmath::Point3<f64>,
    //     truck_modeling::Curve,
    //     truck_modeling::Surface,
    // > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

    // two flush sides!
    // let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    // let line_b = tsweep(&point_b, Vector3::new(0.6, 0.0, 0.0));
    // let square_b = tsweep(&line_b, Vector3::new(0.0, 0.6, 0.0));
    // let cube_b: Solid<
    //     truck_meshalgo::prelude::cgmath::Point3<f64>,
    //     truck_modeling::Curve,
    //     truck_modeling::Surface,
    // > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

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
) -> Option<Solid<Point3, C, Surface>> {
    println!("Okay let's fuse!");

    let solid0_boundaries = solid0.boundaries();
    let solid1_boundaries = solid1.boundaries();
    assert!(solid0_boundaries.len() == 1);
    assert!(solid1_boundaries.len() == 1);

    let boundary0 = &solid0_boundaries[0];
    let boundary1 = &solid1_boundaries[0];
    let fusable_faces = find_coplanar_face_pairs(boundary0, boundary1, true);
    println!("fusable_faces: {:?}", fusable_faces);

    let secondary_mergeable_faces = find_coplanar_face_pairs(boundary0, boundary1, false);
    println!("secondary_mergeable_faces: {:?}", secondary_mergeable_faces);

    // There's only one fused solid at the end. Create it by cloning solid0
    // and then removing the fusable face from it.
    // let mut combined = solid0.clone();
    let mut combined = boundary0.clone();
    // a solid is just a struct with a field called boundaries which is a vec of Shells.
    // and in our case there's only one relevant shell, so let's work with shells not solids.

    // Meanwhile, make a copy of solid1 and remove the fusable face from it too.
    let mut boundary1_copy = boundary1.clone();

    // Then, add every face from solid1 to the combined solid.
    combined.append(&mut boundary1_copy);

    // Lastly, merge the two fusable faces together. This is complicated because
    // one might be bigger than the other, or they might be the same size, or
    // they might overlap somewhat. We'll need to figure out how to merge them.

    // Then add that merged face to the solid and we've fused!

    // After that, we need to merge the secondary_mergeable_faces together.

    // And then we're done!
    Some(Solid::new(vec![combined]))
}

fn find_coplanar_face_pairs<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    boundary0: &Shell<Point3, C, Surface>,
    boundary1: &Shell<Point3, C, Surface>,
    flip_second: bool,
) -> Vec<(usize, usize)> {
    let mut coplanar_faces: Vec<(usize, usize)> = vec![];
    for (face_0_idx, face_0) in boundary0.face_iter().enumerate() {
        let surface_0 = face_0.oriented_surface();

        match surface_0 {
            Surface::Plane(p0) => {
                for (face_1_idx, face_1) in boundary1.face_iter().enumerate() {
                    let mut surface_1 = face_1.oriented_surface();

                    if flip_second {
                        surface_1 = surface_1.inverse();
                    }

                    match surface_1 {
                        Surface::Plane(p1) => {
                            if are_coplanar(p0, p1) {
                                coplanar_faces.push((face_0_idx, face_1_idx));
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    coplanar_faces
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
