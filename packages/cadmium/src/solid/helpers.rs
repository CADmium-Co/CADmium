use std::cell::RefCell;
use std::rc::Rc;

use geo::LineString;
use truck_modeling::{builder, Edge, Vertex, Wire};

use crate::isketch::ISketch;
use super::prelude::*;

pub fn geopoint_to_truckpoint(point: geo::Point<f64>, sketch: Rc<RefCell<ISketch>>) -> Result<TruckPoint3, anyhow::Error> {
    let sketch_ref = sketch.borrow();
    let sketch_point = sketch_ref.find_point_ref(point.x(), point.y()).ok_or(anyhow::anyhow!("geo::Point not found in sketch"))?;
    let point_3d = sketch_ref.get_point_3d(sketch_point)?.1;
    Ok(point_3d.into())
}

pub fn linestring_to_wire(line: &LineString, sketch: Rc<RefCell<ISketch>>) -> Result<Wire, anyhow::Error> {
    let mut vertices: Vec<Vertex> = Vec::new();
    for point in line.points() {
        let vertex = builder::vertex(geopoint_to_truckpoint(point, sketch.clone())?);
        vertices.push(vertex);
    }

    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..vertices.len() - 2 {
        let edge = builder::line(&vertices[i], &vertices[i + 1]);
        edges.push(edge);
    }

    // Close the loop by connecting the last vertex to the first
    let last_edge = builder::line(&vertices[vertices.len() - 2], &vertices[0]);
    edges.push(last_edge);

    Ok(Wire::from_iter(edges.into_iter()))
}

// It assumes that the feature will start from the same plane as the sketch
// To change this, geopoint_to_truckpoint should be modified to accept a plane
// and calculate the 3d point from that plane on-demand
pub fn get_isoface_wires(
    sketch: Rc<RefCell<ISketch>>,
    face: &ISOFace,
) -> Result<Vec<Wire>, anyhow::Error> {
    let polygon = face.as_polygon();
    let exterior = linestring_to_wire(polygon.exterior(), sketch.clone())?;
    let mut interiors = polygon
        .interiors()
        .iter()
        .map(|line| linestring_to_wire(line, sketch.clone()))
        .collect::<Result<Vec<_>, anyhow::Error>>()?;
    interiors.insert(0, exterior);

    Ok(interiors)
}
