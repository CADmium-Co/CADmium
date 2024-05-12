#![allow(dead_code, unused)]

use std::ops::{Sub, SubAssign};
use std::sync::Arc;

use cadmium::extrusion::fuse;
use cadmium::oplog::EvolutionLog;
use cadmium::oplog::Operation;
use cadmium::project::Plane;
use cadmium::{oplog, sketch, Realization};
use truck_meshalgo::filters::OptimizingFilter;
use truck_meshalgo::tessellation::{MeshableShape, MeshedShape};
use truck_modeling::builder::{translated, tsweep, vertex};
use truck_modeling::{Point3, Surface, Vector3};
use truck_polymesh::{obj, InnerSpace, Invertible, ParametricSurface, Tolerance};
use truck_shapeops::{and, or, ShapeOpsCurve, ShapeOpsSurface};
use truck_topology::{Shell, Solid};

fn main() {
    simple_cube();
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

    // el.git_log();
    // el.to_project();
}
