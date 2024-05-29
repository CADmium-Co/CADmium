use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::fmt::Debug;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use truck_meshalgo::prelude::OptimizingFilter;
use truck_meshalgo::tessellation::MeshableShape;
use truck_meshalgo::tessellation::MeshedShape;
use truck_polymesh::obj;
use truck_stepio::out;

use crate::archetypes::Vector2;
use crate::archetypes::Vector3;

pub mod extrusion;
pub mod helpers;
pub mod point;
pub mod prelude;

use prelude::*;

const MESH_TOLERANCE: f64 = 0.1;

pub trait SolidLike: Debug {
    fn references(&self) -> Vec<FeatureCell>;
    fn get_truck_solids(&self) -> anyhow::Result<Vec<TruckClosedSolid>>;
    fn to_feature(&self) -> Feature;
}

impl dyn SolidLike {
    pub fn to_solids(&self) -> anyhow::Result<Vec<Solid>> {
        let truck_solids = self.get_truck_solids()?;

        Ok(truck_solids.iter().map(|truck_solid| {
            Solid::from_truck_solid("".to_owned(), truck_solid.clone())
        }).collect())
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Feature {
    Extrusion(extrusion::Extrusion),
}

impl Feature {
    pub fn as_solid_like(&self) -> &dyn SolidLike {
        match self {
            Feature::Extrusion(extrusion) => extrusion,
        }
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum FeatureCell {
    Extrusion(Rc<RefCell<extrusion::Extrusion>>),
}

impl FeatureCell {
    pub fn borrow(&self) -> Ref<dyn SolidLike> {
        match self {
            FeatureCell::Extrusion(e) => e.borrow(),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<dyn SolidLike > {
        match self {
            FeatureCell::Extrusion(e) => e.borrow_mut(),
        }
    }

    pub fn as_ptr(&self) -> *const dyn SolidLike {
        match self {
            FeatureCell::Extrusion(e) => e.as_ptr(),
        }
    }
}

impl PartialEq for FeatureCell {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.as_ptr(), other.as_ptr())
    }
}

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Solid {
    pub name: String,
    pub crc32: String,
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub indices: Vec<usize>,
    pub triangles: Vec<Vec<u64>>,
    pub truck_solid: TruckClosedSolid,
}

impl Solid {
    pub fn from_truck_solid(
        name: String,
        truck_solid: TruckClosedSolid,
    ) -> Self {
        let mut solid = Solid {
            name,
            crc32: "".to_owned(),
            vertices: vec![],
            normals: vec![],
            triangles: vec![],
            uvs: vec![],
            indices: vec![],
            truck_solid,
        };
        let mut mesh = solid.truck_solid.triangulation(MESH_TOLERANCE).to_polygon();
        mesh.put_together_same_attrs();

        // the mesh is prepared for obj export, but we need to convert it
        // to a format compatible for rendering
        // We have to brute force this. Go through every single triangle
        // and emit three positions, three normals, and three uvs.
        let mut index = 0 as usize;
        for face in mesh.tri_faces() {
            for v in face.iter() {
                let vertex_index = v.pos;
                let normal_index = v.nor.unwrap();
                let uv_index = v.uv.unwrap();
                let vertex = mesh.positions()[vertex_index];
                let normal = mesh.normals()[normal_index];
                let uv = mesh.uv_coords()[uv_index];

                let pt = Vector3::new(vertex.x, vertex.y, vertex.z);
                solid.vertices.push(pt);
                solid
                    .normals
                    .push(Vector3::new(normal.x, normal.y, normal.z));
                solid.uvs.push(Vector2::new(uv.x, uv.y));
                solid.indices.push(index);

                index += 1;
            }
        }

        // compute the crc32 of the vertices
        let mut hasher = crc32fast::Hasher::new();
        for vertex in solid.vertices.iter() {
            hasher.update(&vertex.x.to_be_bytes());
            hasher.update(&vertex.y.to_be_bytes());
            hasher.update(&vertex.z.to_be_bytes());
        }
        solid.crc32 = format!("{:x}", hasher.finalize());

        solid
    }

    pub fn get_face_by_normal(&self, normal: &Vector3) -> Option<TruckFace> {
        let truck_solid = &self.truck_solid;
        let boundaries = &truck_solid.boundaries()[0];

        let mut candidate_faces: Vec<TruckFace> = vec![];

        boundaries.face_iter().for_each(|face| {
            let oriented_surface = face.oriented_surface();

            match oriented_surface {
                truck_modeling::geometry::Surface::Plane(p) => {
                    let this_face_normal = p.normal();

                    if (normal.x - this_face_normal.x).abs() < 0.0001
                        && (normal.y - this_face_normal.y).abs() < 0.0001
                        && (normal.z - this_face_normal.z).abs() < 0.0001
                    {
                        candidate_faces.push(face.clone());
                    }
                }
                _ => {}
            }
        });

        match candidate_faces.len() {
            0 => None,
            1 => Some(candidate_faces[0].clone()),
            _ => panic!("More than one face with the same normal!"),
        }
    }

    pub fn to_obj_string(&self, tolerance: f64) -> String {
        let mut mesh = self.truck_solid.triangulation(tolerance).to_polygon();
        mesh.put_together_same_attrs();
        let mut buf = Vec::new();
        obj::write(&mesh, &mut buf).unwrap();
        let string = String::from_utf8(buf).unwrap();
        string
    }

    pub fn save_as_obj(&self, filename: &str, tolerance: f64) {
        let mut mesh = self.truck_solid.triangulation(tolerance).to_polygon();
        mesh.put_together_same_attrs();
        let file = std::fs::File::create(filename).unwrap();
        obj::write(&mesh, file).unwrap();
    }

    pub fn to_step_string(&self) -> String {
        let compressed = self.truck_solid.compress();
        let step_string = out::CompleteStepDisplay::new(
            out::StepModel::from(&compressed),
            out::StepHeaderDescriptor {
                origination_system: "cadmium-shape-to-step".to_owned(),
                ..Default::default()
            },
        )
        .to_string();
        step_string
    }

    pub fn save_as_step(&self, filename: &str) {
        let step_text = self.to_step_string();
        let mut step_file = std::fs::File::create(filename).unwrap();
        std::io::Write::write_all(&mut step_file, step_text.as_ref()).unwrap();
    }

}
