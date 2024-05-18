#![allow(dead_code, unused)]

use std::ops::{Sub, SubAssign};
use std::sync::Arc;

use cadmium::extrusion::fuse;
use cadmium::oplog::EvolutionLog;
use cadmium::oplog::Operation;
use cadmium::project::Plane;
use cadmium::{oplog, sketch, Realization};
use truck_meshalgo::analyzers::CalcVolume;
use truck_meshalgo::filters::OptimizingFilter;
use truck_meshalgo::tessellation::{MeshableShape, MeshedShape};
use truck_modeling::builder::{translated, tsweep, vertex};
use truck_modeling::{Point3, Surface, Vector3};
use truck_polymesh::{
    obj, InnerSpace, Invertible, ParametricSurface, ParametricSurface3D, Tolerance,
};
use truck_shapeops::{and, or, ShapeOpsCurve, ShapeOpsSurface};
use truck_topology::{Shell, Solid};

fn main() {
    stacked_cubes();
}

fn stacked_cubes() {
    let mut el = EvolutionLog::new();

    let workbench_id = el.append(Operation::CreateWorkbench {
        nonce: "Workbench 1".to_string(),
    });
    el.append(Operation::SetWorkbenchName {
        workbench_id: workbench_id.clone(),
        name: "Main Workbench".to_string(),
    });

    // Create the Top Plane
    let top_plane_id = el.append(Operation::CreatePlane {
        nonce: "the top plane".to_string(),
        workbench_id: workbench_id.clone(),
    });
    el.append(Operation::SetPlaneName {
        plane_id: top_plane_id.clone(),
        name: "Top".to_string(),
    });
    let set_plane = el.append(Operation::SetPlane {
        plane_id: top_plane_id.clone(),
        plane: Plane::top(),
    });
    let top_plane_real = el.realize_plane(&top_plane_id);

    // Create the sketch
    let sketch_id = el.append(Operation::CreateSketch {
        nonce: "top sketch".to_string(),
        workbench_id: workbench_id.clone(),
    });
    el.append(Operation::SetSketchName {
        sketch_id: sketch_id.clone(),
        name: "Original Sketch".to_string(),
    });
    el.append(Operation::SetSketchPlane {
        sketch_id: sketch_id.clone(),
        plane_id: top_plane_real.clone(),
    });

    // make a square
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (0.0, 0.0),
        end: (0.0, 100.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (0.0, 100.0),
        end: (100.0, 100.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (100.0, 100.0),
        end: (100.0, 0.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (100.0, 0.0),
        end: (0.0, 0.0),
    });
    let realized_sketch = el.realize_sketch(&sketch_id);

    // extrude the square
    let extrusion_id = el.append(Operation::CreateExtrusion {
        workbench_id: workbench_id.clone(),
        nonce: "first extrusion".to_string(),
    });
    el.append(Operation::SetExtrusionName {
        extrusion_id: extrusion_id.clone(),
        name: "Extrude1".to_string(),
    });
    el.append(Operation::SetExtrusionDepth {
        extrusion_id: extrusion_id.clone(),
        depth: 100.0,
    });
    el.append(Operation::SetExtrusionSketch {
        extrusion_id: extrusion_id.clone(),
        sketch_id: realized_sketch.clone(),
    });
    el.append(Operation::SetExtrusionFaces {
        extrusion_id: extrusion_id.clone(),
        faces: vec![0],
    });

    el.realize_extrusion(&extrusion_id);

    // Create a plane on the face whose normal points up
    let mut upward_face = None;
    for (face_sha, face) in el.faces.iter() {
        let surface = face.oriented_surface();
        let normal = surface.normal(0.0, 0.0);
        if normal.near(&Vector3::new(0.0, 0.0, 1.0)) {
            upward_face = Some(face.clone());
        }
    }
    let second_plane_id = el.append(Operation::CreatePlane {
        nonce: "the second plane".to_string(),
        workbench_id: workbench_id.clone(),
    });
    el.append(Operation::SetPlaneName {
        plane_id: second_plane_id.clone(),
        name: "Second Plane".to_string(),
    });
    match upward_face {
        Some(face) => {
            let set_plane = el.append(Operation::SetPlane {
                plane_id: second_plane_id.clone(),
                plane: Plane::from_truck_face(face),
            });
        }
        None => {
            println!("No upward face found!");
            unreachable!();
        }
    }
    let second_plane_real = el.realize_plane(&second_plane_id);

    // Create a second sketch on top of the second plane
    let second_sketch_id = el.append(Operation::CreateSketch {
        nonce: "second sketch".to_string(),
        workbench_id: workbench_id.clone(),
    });
    el.append(Operation::SetSketchName {
        sketch_id: second_sketch_id.clone(),
        name: "Second Sketch".to_string(),
    });
    el.append(Operation::SetSketchPlane {
        sketch_id: second_sketch_id.clone(),
        plane_id: second_plane_real.clone(),
    });

    // make a square
    el.append(Operation::AddSketchLine {
        sketch_id: second_sketch_id.clone(),
        start: (20.0, 20.0),
        end: (20.0, 80.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: second_sketch_id.clone(),
        start: (20.0, 80.0),
        end: (80.0, 80.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: second_sketch_id.clone(),
        start: (80.0, 80.0),
        end: (80.0, 20.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: second_sketch_id.clone(),
        start: (80.0, 20.0),
        end: (20.0, 20.0),
    });

    let second_realized_sketch = el.realize_sketch(&second_sketch_id);

    // extrude the second square
    let second_extrusion_id = el.append(Operation::CreateExtrusion {
        workbench_id: workbench_id.clone(),
        nonce: "second extrusion".to_string(),
    });
    el.append(Operation::SetExtrusionName {
        extrusion_id: second_extrusion_id.clone(),
        name: "Extrude2".to_string(),
    });
    el.append(Operation::SetExtrusionDepth {
        extrusion_id: second_extrusion_id.clone(),
        depth: 60.0,
    });
    el.append(Operation::SetExtrusionSketch {
        extrusion_id: second_extrusion_id.clone(),
        sketch_id: second_realized_sketch.clone(),
    });
    el.append(Operation::SetExtrusionFaces {
        extrusion_id: second_extrusion_id.clone(),
        faces: vec![0],
    });
    el.realize_extrusion(&second_extrusion_id);

    let mut small_solid_id = el.solids.keys().nth(0).unwrap().clone();
    let small_solid_volume = el.solids[&small_solid_id]
        .truck_solid
        .triangulation(0.1)
        .to_polygon()
        .volume();

    let mut big_solid_id = el.solids.keys().nth(1).unwrap().clone();
    let big_solid_volume = el.solids[&big_solid_id]
        .truck_solid
        .triangulation(0.1)
        .to_polygon()
        .volume();

    if big_solid_volume < small_solid_volume {
        (small_solid_id, big_solid_id) = (big_solid_id, small_solid_id);
    }

    el.append(Operation::FuseSolids {
        solid1: big_solid_id,
        solid2: small_solid_id,
    });
    el.git_log();

    for (solid_id, solid) in el.solids.iter() {
        solid.save_as_obj("fused.obj", 0.1);
        // let mut mesh = solid.truck_solid.triangulation(0.1).to_polygon();
        // mesh.put_together_same_attrs(0.1);
        // let v = mesh.volume();
        // println!("ID: {solid_id} volume: {v}");
    }
}

fn simple_cube() {
    let mut el = EvolutionLog::new();

    let workbench_id = el.append(Operation::CreateWorkbench {
        nonce: "Workbench 1".to_string(),
    });
    el.append(Operation::SetWorkbenchName {
        workbench_id: workbench_id.clone(),
        name: "Main Workbench".to_string(),
    });

    // Create the Top Plane
    let top_plane_id = el.append(Operation::CreatePlane {
        nonce: "the top plane".to_string(),
        workbench_id: workbench_id.clone(),
    });
    el.append(Operation::SetPlaneName {
        plane_id: top_plane_id.clone(),
        name: "Top".to_string(),
    });
    let set_plane = el.append(Operation::SetPlane {
        plane_id: top_plane_id.clone(),
        plane: Plane::top(),
    });
    let top_plane_real = el.realize_plane(&top_plane_id);

    // Create the sketch
    let sketch_id = el.append(Operation::CreateSketch {
        nonce: "top sketch".to_string(),
        workbench_id: workbench_id.clone(),
    });
    el.append(Operation::SetSketchName {
        sketch_id: sketch_id.clone(),
        name: "Original Sketch".to_string(),
    });
    el.append(Operation::SetSketchPlane {
        sketch_id: sketch_id.clone(),
        plane_id: top_plane_real.clone(),
    });

    // make a square
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (0.0, 0.0),
        end: (0.0, 100.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (0.0, 100.0),
        end: (100.0, 100.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (100.0, 100.0),
        end: (100.0, 0.0),
    });
    el.append(Operation::AddSketchLine {
        sketch_id: sketch_id.clone(),
        start: (100.0, 0.0),
        end: (0.0, 0.0),
    });
    let realized_sketch = el.realize_sketch(&sketch_id);

    // extrude the square
    let extrusion_id = el.append(Operation::CreateExtrusion {
        workbench_id: workbench_id.clone(),
        nonce: "first extrusion".to_string(),
    });
    el.append(Operation::SetExtrusionName {
        extrusion_id: extrusion_id.clone(),
        name: "Extrude1".to_string(),
    });
    el.append(Operation::SetExtrusionDepth {
        extrusion_id: extrusion_id.clone(),
        depth: 100.0,
    });

    el.append(Operation::SetExtrusionSketch {
        extrusion_id: extrusion_id.clone(),
        sketch_id: realized_sketch.clone(),
    });
    el.append(Operation::SetExtrusionFaces {
        extrusion_id: extrusion_id.clone(),
        faces: vec![0],
    });

    el.realize_extrusion(&extrusion_id);

    // print each solid
    for (solid_id, solid) in el.solids.iter() {
        println!("Solid: {:?}", solid);
        solid.save_as_obj("first_solid.obj", 0.01);
    }

    el.git_log();
    // el.to_project();
}
