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
    // topological_naming();
    stacked_cubes();
}

fn stacked_cubes() {
    let mut el = EvolutionLog::new();

    let project_id = el.append(Operation::CreateProject {
        nonce: "Project 1".to_string(),
    });
    el.append(Operation::SetProjectName {
        project_id: project_id.clone(),
        name: "Main Project".to_string(),
    });

    let workspace_id = el.append(Operation::CreateWorkspace {
        project_id: project_id.clone(),
        nonce: "Workspace 1".to_string(),
    });
    el.append(Operation::SetWorkspaceName {
        workspace_id: workspace_id.clone(),
        name: "Main Workspace".to_string(),
    });

    // Create the Top Plane
    let top_plane_id = el.append(Operation::CreatePlane {
        nonce: "the top plane".to_string(),
        workspace_id: workspace_id.clone(),
    });
    el.append(Operation::SetPlaneName {
        plane_id: top_plane_id.clone(),
        name: "Top".to_string(),
    });
    let set_plane = el.append(Operation::SetPlane {
        plane_id: top_plane_id.clone(),
        plane: Plane::top(),
    });

    // Create the sketch
    let sketch_id = el.append(Operation::CreateSketch {
        nonce: "top sketch".to_string(),
        workspace_id: workspace_id.clone(),
    });
    el.append(Operation::SetSketchName {
        sketch_id: sketch_id.clone(),
        name: "Sketch1".to_string(),
    });
    el.append(Operation::SetSketchPlane {
        sketch_id: sketch_id.clone(),
        plane_id: top_plane_id.clone(),
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
    // Add a handle to pull the extrusion from
    let handle_id = el.append(Operation::AddSketchHandle {
        sketch_id: sketch_id.clone(),
        position: (20.0, 20.0),
    });

    // extrude the square
    let extrusion_id = el.append(Operation::CreateExtrusion {
        workspace_id: workspace_id.clone(),
        nonce: "first extrusion".to_string(),
    });
    el.append(Operation::SetExtrusionName {
        extrusion_id: extrusion_id.clone(),
        name: "Extrude1".to_string(),
    });
    el.append(Operation::SetExtrusionSketch {
        extrusion_id: extrusion_id.clone(),
        sketch_id: sketch_id.clone(),
    });
    el.append(Operation::SetExtrusionHandles {
        extrusion_id: extrusion_id.clone(),
        handles: vec![handle_id.clone()],
    });
    el.append(Operation::SetExtrusionDepth {
        extrusion_id: extrusion_id.clone(),
        depth: 100.0,
    });

    el.git_log();
}
