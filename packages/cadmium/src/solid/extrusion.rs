use std::cell::RefCell;
use std::rc::Rc;

use isotope::decompose::face::Face;
use serde::{Deserialize, Serialize};
use truck_modeling::builder;
use tsify::Tsify;

use truck_polymesh::InnerSpace;
use truck_polymesh::Invertible;
use truck_polymesh::Tolerance;
use truck_shapeops::ShapeOpsCurve;
use truck_shapeops::ShapeOpsSurface;
use truck_topology::Shell;

use super::prelude::*;

use crate::archetypes::Vector3;
use crate::isketch::ISketch;
use crate::IDType;

use super::get_isoface_wires;
use super::Feature;
use super::FeatureCell;
use super::SolidLike;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Mode {
    New,
    Add(Vec<IDType>),
    Remove(Vec<IDType>),
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Direction {
    Normal,
    NegativeNormal,
    Specified(Vector3),
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Extrusion {
    pub faces: Vec<Face>,
    pub sketch: Rc<RefCell<ISketch>>,
    pub length: f64,
    pub offset: f64,
    pub direction: Direction,
    pub mode: Mode,
}

impl Extrusion {
    pub fn new(
        faces: Vec<Face>,
        sketch: Rc<RefCell<ISketch>>,
        length: f64,
        offset: f64,
        direction: Direction,
        mode: Mode,
    ) -> Self {
        Extrusion {
            faces,
            sketch,
            length,
            offset,
            direction,
            mode,
        }
    }
}

impl SolidLike for Extrusion {
    fn references(&self) -> Vec<FeatureCell> {
        // self.faces.iter().map(|f| FeatureCell::Face(f.clone())).collect()
        todo!("Extrusion::references")
    }

    fn to_feature(&self) -> Feature {
        Feature::Extrusion(self.clone())
    }

    fn get_truck_solids(&self) -> anyhow::Result<Vec<TruckClosedSolid>> {
        let plane = self.sketch.borrow().plane.borrow().clone();

        let extrusion_direction = match &self.direction {
            Direction::Normal => plane.tertiary.clone(),
            Direction::NegativeNormal => plane.tertiary.times(-1.0),
            Direction::Specified(vector) => vector.clone(),
        };

        let extrusion_vector = extrusion_direction.times(self.length - self.offset);
        let offset_vector = extrusion_direction.times(self.offset);
        let extrusion_tvector = TruckVector3::new(extrusion_vector.x, extrusion_vector.y, extrusion_vector.z);
        let offset_tvector = TruckVector3::new(offset_vector.x, offset_vector.y, offset_vector.z);

        Ok(self.faces
            .iter()
            .map(|f| {
                let wires = get_isoface_wires(self.sketch.clone(), f).unwrap();
                let face = builder::try_attach_plane(&wires).unwrap();

                // Can we calculate ALL the wires at once and not iter-sweep?
                let sweep = builder::tsweep(&face, extrusion_tvector);
                let translated = builder::translated(&sweep, offset_tvector);

                translated
            }).collect())
    }
}

pub fn find_enveloped_shapes(faces: &Vec<Face>) -> Vec<(usize, usize)> {
    let mut retval = vec![];
    for (a, face_a) in faces.iter().enumerate() {
        for (b, face_b) in faces.iter().enumerate() {
            if a == b {
                continue;
            }

            // check if b's exterior is equal to any of a's holes
            for (_hole_index, hole) in face_a.holes.iter().enumerate() {
                if hole == face_b {
                    retval.push((b, a)); // (small, big)
                }
            }
        }
    }

    return retval;
}

pub fn find_adjacent_shapes(faces: &Vec<Face>) -> Option<(usize, usize, Vec<usize>, Vec<usize>)> {
    for (a, face_a) in faces.iter().enumerate() {
        for (b, face_b) in faces.iter().enumerate() {
            if a == b || a > b {
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

// pub fn find_transit(
//     real_plane: &RealPlane,
//     start: &Point3,
//     end: &Point3,
//     center: &Point3,
//     clockwise: bool,
// ) -> Point3 {
//     // let radius = start.distance_to(center);

//     let start = real_plane.plane.project(start);
//     let end = real_plane.plane.project(end);
//     let center = real_plane.plane.project(center);

//     let pts = arc_to_points(&start, &end, &center, clockwise);

//     let transit = &pts[pts.len() / 2];

//     let transit_3d = real_plane.plane.unproject(&transit);
//     transit_3d
// }

pub fn fuse<C: ShapeOpsCurve<S> + std::fmt::Debug, S: ShapeOpsSurface + std::fmt::Debug>(
    solid0: &TruckTopoSolid<TruckPoint3, C, TruckSurface>,
    solid1: &TruckTopoSolid<TruckPoint3, C, TruckSurface>,
) -> Option<TruckTopoSolid<TruckPoint3, C, TruckSurface>> {
    println!("Okay let's fuse!");

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
    println!("fusable_faces: {:?}", fusable_faces);

    let secondary_mergeable_faces = find_coplanar_face_pairs(boundary0, boundary1, false);
    println!("secondary_mergeable_faces: {:?}", secondary_mergeable_faces);

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

        match surface_0 {
            TruckSurface::Plane(p0) => {
                for (face_1_idx, face_1) in boundary1.face_iter().enumerate() {
                    let mut surface_1 = face_1.oriented_surface();

                    if flip_second {
                        surface_1 = surface_1.inverse();
                    }

                    match surface_1 {
                        TruckSurface::Plane(p1) => {
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

#[cfg(test)]
mod tests {
    use crate::project::Project;
    use crate::project::tests::create_test_project;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn create_project_solid() {
        let mut p = Project::new("Test Extrusion");

        // now get solids? save as obj or stl or step?
        let workbench = p.workbenches.get_mut(0).unwrap();
        let realization = workbench.realize(100).unwrap();
        let solids = realization.solids;
        println!("solids: {:?}", solids);
    }

    #[test]
    #[ignore = "test failing on CI"]
    fn project_from_files() {
        let file_list = [
            // this file contains three shapes which are adjacent to each other and
            // thus should result in a single output solid
            ("src/test_inputs/three_adjacent_faces.cadmium", 1),
            // this file contains one square nested inside another
            // and thus should result in a single output solid
            ("src/test_inputs/nested_squares.cadmium", 1),
            // this file contains one circle nested inside another
            // and thus should result in a single output solid
            ("src/test_inputs/nested_circles.cadmium", 1),
            ("src/test_inputs/two_Es.cadmium", 1),
            ("src/test_inputs/lots_of_nesting.cadmium", 4),
        ];

        for (file, expected_solids) in file_list.iter() {
            let contents = std::fs::read_to_string(file).unwrap();

            // deserialize the contents into a Project
            let mut p: Project = serde_json::from_str(&contents).unwrap();

            // get a realization
            let workbench = p.workbenches.get_mut(0).unwrap();
            let realization = workbench.realize(100).unwrap();
            let solids = realization.solids;
            println!("[{}] solids: {:?}", file, solids.len());

            assert_eq!(solids.len(), *expected_solids); // doesn't work yet!
        }
    }

    #[test]
    #[ignore = "test failing on CI"]
    fn step_export() {
        let mut p = create_test_project();
        let workbench = p.get_workbench_by_id_mut(0).unwrap();
        let realization = workbench.realize(1000).unwrap();
        let keys = Vec::from_iter(realization.solids.keys());

        realization.save_solid_as_step_file(*keys[0], "pkg/test.step");
        realization.save_solid_as_obj_file(*keys[0], "pkg/test.obj", 0.001);
    }

}
