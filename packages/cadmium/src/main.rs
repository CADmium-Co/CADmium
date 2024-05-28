#![allow(dead_code, unused)]

use std::ops::{Sub, SubAssign};

use cadmium::extrusion::fuse;
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
    // let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    // let line_b = tsweep(&point_b, Vector3::new(0.2, 0.0, 0.0));
    // let square_b = tsweep(&line_b, Vector3::new(0.0, 0.2, 0.0));
    // let cube_b: Solid<
    //     truck_meshalgo::prelude::cgmath::Point3<f64>,
    //     truck_modeling::Curve,
    //     truck_modeling::Surface,
    // > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

    // one flush side!
    let point_b = vertex(Point3::new(0.4, 0.4, 1.0));
    let line_b = tsweep(&point_b, Vector3::new(0.6, 0.0, 0.0));
    let square_b = tsweep(&line_b, Vector3::new(0.0, 0.2, 0.0));
    let cube_b: Solid<
        truck_meshalgo::prelude::cgmath::Point3<f64>,
        truck_modeling::Curve,
        truck_modeling::Surface,
    > = tsweep(&square_b, Vector3::new(0.0, 0.0, 0.2));

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
    let combined = fuse(&cube_a, &cube_b).unwrap();

    println!(
        "combined_cube_or has {:?} shell boundaries",
        combined.boundaries().len()
    );

    let mut mesh = combined.triangulation(0.01).to_polygon();
    mesh.put_together_same_attrs(0.1);
    let file = std::fs::File::create("combined_cube.obj").unwrap();
    obj::write(&mesh, file).unwrap();
}
