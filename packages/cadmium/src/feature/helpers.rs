use std::cell::RefCell;
use std::rc::Rc;

use geo::LineString;
use isotope::decompose::face::Face;

use log::debug;
use truck_modeling::{builder, Edge, Vertex, Wire};
use truck_polymesh::InnerSpace;
use truck_polymesh::Invertible;
use truck_polymesh::Tolerance;
use truck_shapeops::{ShapeOpsCurve, ShapeOpsSurface};
use truck_topology::Shell;

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

    Ok(Wire::from_iter(edges))
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

pub fn find_enveloped_shapes(faces: &[Face]) -> Vec<(usize, usize)> {
    let mut retval = vec![];
    for (a, face_a) in faces.iter().enumerate() {
        for (b, face_b) in faces.iter().enumerate() {
            if a == b {
                continue;
            }

            // check if b's exterior is equal to any of a's holes
            for hole in face_a.holes.iter() {
                if hole == face_b {
                    retval.push((b, a)); // (small, big)
                }
            }
        }
    }

    retval
}

pub fn find_adjacent_shapes(faces: &[Face]) -> Option<(usize, usize, Vec<usize>, Vec<usize>)> {
    for (a, face_a) in faces.iter().enumerate() {
        for (b, face_b) in faces.iter().enumerate() {
            if a >= b {
                continue;
            }

            let adjacent_edges = face_a.exterior.adjacent_edges(&face_b.exterior);

            match adjacent_edges {
                None => continue,
                Some(matched_edges) => return Some((a, b, matched_edges.0, matched_edges.1)),
            }
        }
    }

    None
}

pub fn fuse<C: ShapeOpsCurve<S> + std::fmt::Debug, S: ShapeOpsSurface + std::fmt::Debug>(
    solid0: &TruckTopoSolid<TruckPoint3, C, TruckSurface>,
    solid1: &TruckTopoSolid<TruckPoint3, C, TruckSurface>,
) -> Option<TruckTopoSolid<TruckPoint3, C, TruckSurface>> {
    let solid0_boundaries = solid0.boundaries();
    let solid1_boundaries = solid1.boundaries();
    assert!(solid0_boundaries.len() == 1);
    assert!(solid1_boundaries.len() == 1);

    let boundary0 = &solid0_boundaries[0];
    let boundary1 = &solid1_boundaries[0];
    let fusable_faces = find_coplanar_face_pairs(boundary0, boundary1, true);
    assert!(fusable_faces.len() == 1);
    let fusable_faces = fusable_faces[0];
    // TODO: support the case where more than one is fusable
    debug!("fusable_faces: {:?}", fusable_faces);

    let secondary_mergeable_faces = find_coplanar_face_pairs(boundary0, boundary1, false);
    debug!("secondary_mergeable_faces: {:?}", secondary_mergeable_faces);

    // There's only one fused solid at the end. Create it by cloning solid0
    // and then removing the fusable face from it.
    let mut combined = boundary0.clone();
    combined.remove(fusable_faces.0);

    // Meanwhile, make a copy of solid1 and remove the fusable face from it too.
    let mut boundary1_copy = boundary1.clone();
    boundary1_copy.remove(fusable_faces.1);

    // Then, add every face from solid1 to the combined solid.
    combined.append(&mut boundary1_copy);

    // Lastly, merge the two fusable faces together. This is complicated because
    // one might be bigger than the other, or they might be the same size, or
    // they might overlap somewhat. We'll need to figure out how to merge them.
    // println!("How do I merge these two? {:?}", fusable_faces);
    // println!("First:");
    // for edge in boundary0[fusable_faces.0].edge_iter() {
    //     println!(
    //         "Edge: {:?} to {:?}",
    //         edge.front().get_point(),
    //         edge.back().get_point()
    //     );
    // }
    let mut outer_face = boundary0[fusable_faces.0].clone();
    let inner_face = boundary1[fusable_faces.1].clone();
    outer_face.add_boundary(inner_face.boundaries().first().unwrap().clone());

    // Then add that merged face to the solid and we've fused!
    combined.push(outer_face);

    // After that, we need to merge the secondary_mergeable_faces together.
    for (face_0_idx, face_1_idx) in secondary_mergeable_faces {
        let mut face_0 = boundary0[face_0_idx].clone();
        let face_1 = boundary1[face_1_idx].clone();
        face_0.add_boundary(face_1.boundaries().first().unwrap().clone());
        combined.push(face_0);
    }

    // And then we're done!
    // None
    Some(TruckTopoSolid::new(vec![combined]))
}

fn find_coplanar_face_pairs<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    boundary0: &Shell<TruckPoint3, C, TruckSurface>,
    boundary1: &Shell<TruckPoint3, C, TruckSurface>,
    flip_second: bool,
) -> Vec<(usize, usize)> {
    let mut coplanar_faces: Vec<(usize, usize)> = vec![];
    for (face_0_idx, face_0) in boundary0.face_iter().enumerate() {
        let surface_0 = face_0.oriented_surface();

        if let TruckSurface::Plane(p0) = surface_0 {
            for (face_1_idx, face_1) in boundary1.face_iter().enumerate() {
                let mut surface_1 = face_1.oriented_surface();

                if flip_second {
                    surface_1 = surface_1.inverse();
                }

                if let TruckSurface::Plane(p1) = surface_1 {
                    if are_coplanar(p0, p1) {
                        coplanar_faces.push((face_0_idx, face_1_idx));
                    }
                }
            }
        }
    }

    coplanar_faces
}

fn are_coplanar(p0: TruckPlane, p1: TruckPlane) -> bool {
    let normal0 = p0.normal();
    let normal1 = p1.normal();

    if !normal0.near(&normal1) {
        return false;
    }

    let difference = p0.origin() - p1.origin();
    let dot = normal0.dot(difference);
    dot.abs() < 0.0001
}
