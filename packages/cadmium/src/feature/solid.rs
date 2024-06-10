use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use truck_meshalgo::prelude::OptimizingFilter;
use truck_meshalgo::tessellation::MeshableShape;
use truck_meshalgo::tessellation::MeshedShape;
use truck_polymesh::obj;
use truck_stepio::out;

use crate::archetypes::Vector2;
use crate::archetypes::Vector3;

use super::prelude::*;

#[derive(Tsify, Debug, Serialize, Deserialize, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[repr(transparent)]
pub struct SolidArray(pub Vec<Solid>);

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
        mesh.put_together_same_attrs(MESH_TOLERANCE);

        // the mesh is prepared for obj export, but we need to convert it
        // to a format compatible for rendering
        // We have to brute force this. Go through every single triangle
        // and emit three positions, three normals, and three uvs.
        let mut index = 0_usize;
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

            if let truck_modeling::geometry::Surface::Plane(p) = oriented_surface {
                let this_face_normal = p.normal();

                if (normal.x - this_face_normal.x).abs() < 0.0001
                    && (normal.y - this_face_normal.y).abs() < 0.0001
                    && (normal.z - this_face_normal.z).abs() < 0.0001
                {
                    candidate_faces.push(face.clone());
                }
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
        mesh.put_together_same_attrs(MESH_TOLERANCE);
        let mut buf = Vec::new();
        obj::write(&mesh, &mut buf).unwrap();

        String::from_utf8(buf).unwrap()
    }

    pub fn save_as_obj(&self, filename: &str, tolerance: f64) {
        let mut mesh = self.truck_solid.triangulation(tolerance).to_polygon();
        mesh.put_together_same_attrs(MESH_TOLERANCE);
        let file = std::fs::File::create(filename).unwrap();
        obj::write(&mesh, file).unwrap();
    }

    pub fn to_step_string(&self) -> String {
        let compressed = self.truck_solid.compress();

        out::CompleteStepDisplay::new(
            out::StepModel::from(&compressed),
            out::StepHeaderDescriptor {
                organization_system: "cadmium-shape-to-step".to_owned(),
                ..Default::default()
            },
        )
        .to_string()
    }

    pub fn save_as_step(&self, filename: &str) {
        let step_text = self.to_step_string();
        let mut step_file = std::fs::File::create(filename).unwrap();
        std::io::Write::write_all(&mut step_file, step_text.as_ref()).unwrap();
    }

}
