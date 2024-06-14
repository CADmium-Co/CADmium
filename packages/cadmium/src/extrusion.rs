use geo::Contains;
use geo::InteriorPoint;
use geo::Polygon;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use truck_polymesh::InnerSpace;
use truck_polymesh::Invertible;
use truck_polymesh::Tolerance;
use truck_shapeops::ShapeOpsCurve;
use truck_shapeops::ShapeOpsSurface;
use truck_topology::Shell;

use crate::archetypes::{Point3, Vector3};
use crate::project::{RealPlane, RealSketch};
use crate::sketch::{arc_to_points, Face, Sketch};

use truck_modeling::{Plane, Point3 as TruckPoint3, Surface};

use truck_topology::Solid as TruckSolid;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Extrusion {
    pub sketch_id: String,
    pub face_ids: Vec<u64>,
    pub length: f64,
    pub offset: f64,
    pub direction: Direction,
    pub mode: ExtrusionMode,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum ExtrusionMode {
    New,
    Add(Vec<String>),
    Remove(Vec<String>),
}

impl Extrusion {
    pub fn new(
        sketch_id: String,
        face_ids: Vec<u64>,
        length: f64,
        offset: f64,
        direction: Direction,
        mode: ExtrusionMode,
    ) -> Self {
        Extrusion {
            sketch_id,
            face_ids,
            length,
            offset,
            direction,
            mode,
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Direction {
    Normal,
    NegativeNormal,
    Specified(Vector3),
}

pub fn find_enveloped_shapes(faces: &Vec<Face>) -> Vec<(usize, usize)> {
    let mut retval = vec![];
    for (a, face_a) in faces.iter().enumerate() {
        for (b, face_b) in faces.iter().enumerate() {
            if a == b {
                continue;
            }
            let face_b_exterior = &face_b.exterior.canonical_form();

            // check if b's exterior is equal to any of a's holes
            for (_hole_index, hole) in face_a.holes.iter().enumerate() {
                let hole = hole.canonical_form();

                if face_b_exterior.equals(&hole) {
                    retval.push((b, a)); // (small, big)
                }
            }
        }
    }

    return retval;
}

pub fn merge_faces(faces: &Vec<Face>, real_sketch: &RealSketch) -> Vec<Face> {
    // create  new sketch using these faces
    let sketch = Sketch::from_faces(faces, real_sketch);

    let (mut all_sketch_faces, _unused_segments) = sketch.find_faces();

    // check whether each of these new faces should returned or not by picking a
    // random point on the new face and then checking every one of the original faces
    // to see if it contains that point. If so, we can keep that new face
    let mut faces_to_remove: Vec<usize> = vec![];
    let old_faces_as_polygons: Vec<Polygon> = faces
        .iter()
        .map(|face| sketch.face_as_polygon(face))
        .collect::<Vec<_>>();
    for (new_face_idx, face) in all_sketch_faces.iter().enumerate() {
        // println!("\nNew face: {}: {:?}", new_face_idx, face);

        let as_geo_polygon = sketch.face_as_polygon(face);
        // println!("as_geo_polygon: {:?}", as_geo_polygon);

        let random_point_on_face = as_geo_polygon
            .interior_point()
            .expect("Every polygon should be able to yield an interior point");
        // println!("Random point on face: {:?}", random_point_on_face);

        let mut located = false;
        for (_old_face_idx, old_face) in old_faces_as_polygons.iter().enumerate() {
            if old_face.contains(&random_point_on_face) {
                // println!(
                //     "Old face {} contains point {:?}",
                //     old_face_idx, random_point_on_face
                // );
                // this means the old face contains the random point on the new face
                // so we can keep this new face
                located = true;
                break;
            }
        }
        if !located {
            // println!(
            //     "Random point from new face {} is not contained by any old faces",
            //     new_face_idx
            // );
            faces_to_remove.push(new_face_idx);
        }
    }

    // remove the faces that we don't want
    faces_to_remove.sort();
    faces_to_remove.reverse();
    // println!("New Faces to remove: {:?}", faces_to_remove);
    for face_to_remove in faces_to_remove {
        all_sketch_faces.remove(face_to_remove);
    }

    // println!("Merge faces 2 output: {}", faces.len());
    all_sketch_faces
}

// pub fn find_adjacent_shapes(faces: &Vec<Face>) -> Option<(usize, usize, Vec<usize>, Vec<usize>)> {
//     for (a, face_a) in faces.iter().enumerate() {
//         for (b, face_b) in faces.iter().enumerate() {
//             if a == b || a > b {
//                 continue;
//             }

//             let adjacent_edges = face_a.exterior.adjacent_edges(&face_b.exterior);

//             match adjacent_edges {
//                 None => continue,
//                 Some(matched_edges) => return Some((a, b, matched_edges.0, matched_edges.1)),
//             }
//         }
//     }

//     None
// }

// pub fn merge_faces(faces: Vec<Face>) -> Vec<Face> {
//     let mut faces = faces.clone();
//     // adjacency:
//     // check if this shape's exterior is adjacent to any other shape's exterior
//     // if so, merge them into a single shape by deleting any shared sides
//     // and recomputing the faces

//     while let Some((a, b, a_indices, b_indices)) = find_adjacent_shapes(&faces) {
//         println!("touching_shapes: {:?}", (a, b, a_indices, b_indices));
//         let face_a = &faces[a];
//         let face_b = &faces[b];

//         match (&face_a.exterior, &face_b.exterior) {
//             (Ring::Segments(segments_a), Ring::Segments(segments_b)) => {
//                 let mut face_a_location = 0;
//                 let mut face_b_location = 0;
//                 let mut pulling_from_a = true;
//                 let mut new_exterior_segments: Vec<Segment> = vec![];

//                 loop {
//                     if pulling_from_a {
//                         let segment = segments_a[face_a_location].clone();
//                         new_exterior_segments.push(segment);
//                         face_a_location += 1;
//                     } else {
//                         // pull from b
//                         let segment = segments_b[face_b_location].clone();
//                         new_exterior_segments.push(segment);
//                         face_b_location += 1;
//                     }
//                 }
//             }
//             _ => panic!("Only Rings made of Segments can have adjacent edges!"),
//         }

//         // let mut new_face = Face {
//         //     exterior: new_exterior_segments,
//         //     holes: vec![],
//         // };

//         // remove face a and face b
//         // add new_face

//         break;
//     }

//     // envelopment:
//     // check if this shape's exterior is equal to any other shape's hole
//     // if so, merge them into a single shape by deleting that hole from the
//     // other shape, and adding this shape's holes to that shape's holes
//     while let Some((a, b, c)) = find_enveloped_shapes(&faces) {
//         // this means a's exterior is equal to one of b's holes. Hole c in particular
//         let face_a = &faces[a];
//         let face_b = &faces[b];

//         // to fix this we need to remove the information contained in a's exterior completely.
//         // to do that we remove the c indexed hole from face_b's list of holes
//         let mut b_new_holes = face_b.holes.clone();
//         b_new_holes.remove(c);
//         b_new_holes.append(&mut face_a.holes.clone());

//         let mut new_face_b = Face {
//             exterior: face_b.exterior.clone(),
//             holes: b_new_holes,
//         };

//         let mut new_faces = faces.clone();

//         // replace the larger face with our modified face
//         new_faces[b] = new_face_b.clone();

//         // remove the smaller face from the list of faces
//         new_faces.remove(a);
//         faces = new_faces;
//     }

//     faces
// }

pub fn find_transit(
    real_plane: &RealPlane,
    start: &Point3,
    end: &Point3,
    center: &Point3,
    clockwise: bool,
) -> Point3 {
    // let radius = start.distance_to(center);

    let start = real_plane.plane.project(start);
    let end = real_plane.plane.project(end);
    let center = real_plane.plane.project(center);

    let pts = arc_to_points(&start, &end, &center, clockwise);

    let transit = &pts[pts.len() / 2];

    let transit_3d = real_plane.plane.unproject(&transit);
    transit_3d
}

pub fn fuse<C: ShapeOpsCurve<S> + std::fmt::Debug, S: ShapeOpsSurface + std::fmt::Debug>(
    solid0: &TruckSolid<TruckPoint3, C, Surface>,
    solid1: &TruckSolid<TruckPoint3, C, Surface>,
) -> Option<TruckSolid<TruckPoint3, C, Surface>> {
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
    Some(TruckSolid::new(vec![combined]))
}

fn find_coplanar_face_pairs<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    boundary0: &Shell<TruckPoint3, C, Surface>,
    boundary1: &Shell<TruckPoint3, C, Surface>,
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

#[cfg(test)]
mod tests {
    use crate::project::tests::create_test_project;
    use crate::project::Project;

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn create_project_solid() {
        // Demonstrate creating a project and then realizing one solid
        let p = create_test_project();

        // now get solids? save as obj or stl or step?
        let workbench = p.workbenches.get(0).unwrap();
        let realization = workbench.realize(100);
        let solids = realization.solids;
        assert!(solids.len() == 1);
    }

    #[test]
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
            let realization = workbench.realize(100);
            let solids = realization.solids;
            println!("[{}] solids: {:?}", file, solids.len());

            assert_eq!(solids.len(), *expected_solids); // doesn't work yet!
        }
    }

    #[test]
    fn step_export() {
        let p = create_test_project();
        let workbench = &p.workbenches[0 as usize];
        let realization = workbench.realize(1000);
        let keys = Vec::from_iter(realization.solids.keys());

        realization.save_solid_as_step_file(keys[0], "pkg/test.step");
        realization.save_solid_as_obj_file(keys[0], "pkg/test.obj", 0.001);
    }
}
