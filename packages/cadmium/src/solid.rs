use std::collections::HashMap;
use std::f64::consts::PI;

use serde::{Deserialize, Serialize};
use tsify::Tsify;

use truck_meshalgo::prelude::OptimizingFilter;
use truck_meshalgo::tessellation::MeshableShape;
use truck_meshalgo::tessellation::MeshedShape;
use truck_polymesh::obj;
use truck_polymesh::Rad;
use truck_stepio::out;

use crate::archetypes::Vector3;
use crate::extrusion::find_transit;
use crate::extrusion::merge_faces;
use crate::extrusion::Direction;
use crate::extrusion::Extrusion;
use crate::project::{RealPlane, RealSketch};
use crate::sketch::Vector2;
use crate::sketch::{Face, Ring, Segment};

use truck_modeling::{builder, builder::translated, Edge, Face as TruckFace, Vertex, Wire};

use truck_polymesh::Point3 as TruckPoint3;
use truck_polymesh::Vector3 as TruckVector3;
use truck_topology::Solid as TruckSolid;

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
    pub truck_solid: TruckSolid<
        truck_polymesh::cgmath::Point3<f64>,
        truck_modeling::Curve,
        truck_modeling::Surface,
    >,
}

impl Solid {
    pub fn from_truck_solid(
        name: String,
        truck_solid: TruckSolid<
            truck_meshalgo::prelude::cgmath::Point3<f64>,
            truck_modeling::Curve,
            truck_modeling::Surface,
        >,
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
        let mesh = solid.truck_solid.triangulation(0.01).to_polygon();

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

    pub fn from_extrusion(
        name: String,
        plane: &RealPlane,
        sketch: &RealSketch,
        extrusion: &Extrusion,
    ) -> HashMap<String, Self> {
        let mut retval = HashMap::new();

        let extrusion_direction = match &extrusion.direction {
            Direction::Normal => plane.plane.tertiary.clone(),
            Direction::NegativeNormal => plane.plane.tertiary.times(-1.0),
            Direction::Specified(vector) => vector.clone(),
        };

        let extrusion_vector = extrusion_direction.times(extrusion.length - extrusion.offset);
        let offset_vector = extrusion_direction.times(extrusion.offset);

        let vector = TruckVector3::new(extrusion_vector.x, extrusion_vector.y, extrusion_vector.z);
        let offset_vector = TruckVector3::new(offset_vector.x, offset_vector.y, offset_vector.z);

        // Sometimes the chosen faces are touching, or one even envelops another. Let's
        // merge those faces together so that we have single solid wherever possible
        let unmerged_faces: Vec<Face> = extrusion
            .face_ids
            .iter()
            .map(|face_id| sketch.faces.get(*face_id as usize).unwrap().clone())
            .collect();
        let merged_faces = merge_faces(&unmerged_faces, sketch);

        for (f_index, face) in merged_faces.iter().enumerate() {
            // let face = sketch.faces.get(*face_id as usize).unwrap();

            // println!("face: {:?}", face);
            let exterior = &face.exterior;
            let mut wires: Vec<Wire> = Vec::new();

            // the exterior wire comes first
            wires.push(Self::to_wire(plane, sketch, extrusion, exterior));

            // then the interior wires
            for interior in &face.holes {
                wires.push(Self::to_wire(plane, sketch, extrusion, interior).inverse());
            }

            let face = builder::try_attach_plane(&wires).unwrap();

            let truck_solid = builder::tsweep(&face, vector);
            let truck_solid = translated(&truck_solid, offset_vector);

            let solid = Solid::from_truck_solid(format!("{}:{}", name, f_index), truck_solid);

            retval.insert(format!("{}:{}", name, f_index), solid);
        }

        retval
    }

    pub fn to_wire(
        plane: &RealPlane,
        sketch: &RealSketch,
        _extrusion: &Extrusion,
        exterior: &Ring,
    ) -> Wire {
        match exterior {
            Ring::Circle(circle) => {
                println!("circle: {:?}", circle);

                let center = sketch.points.get(&circle.center).unwrap();
                let center_point = TruckPoint3::new(center.x, center.y, center.z);

                let top = sketch.points.get(&circle.top).unwrap();
                let top_point = TruckPoint3::new(top.x, top.y, top.z);

                let vector = TruckVector3::new(
                    plane.plane.tertiary.x,
                    plane.plane.tertiary.y,
                    plane.plane.tertiary.z,
                );

                // we actually achieve this with an rsweep!
                let vertex = builder::vertex(top_point);
                let circle = builder::rsweep(&vertex, center_point, vector, Rad(2.0 * PI));
                circle
            }
            Ring::Segments(segments) => {
                // println!("segments: {:?}", segments);
                // let mut builder = builder::FaceBuilder::new();
                let mut vertices: HashMap<u64, Vertex> = HashMap::new();

                // First just collect all the points as Truck Vertices
                // This is important because for shapes to be closed,
                // Truck requires that the start point IS the end point
                for segment in segments.iter() {
                    match segment {
                        Segment::Line(line) => {
                            let start = sketch.points.get(&line.start).unwrap();
                            let start_vertex =
                                builder::vertex(TruckPoint3::new(start.x, start.y, start.z));
                            let end = sketch.points.get(&line.end).unwrap();
                            let end_vertex = builder::vertex(TruckPoint3::new(end.x, end.y, end.z));
                            vertices.insert(line.start, start_vertex);
                            vertices.insert(line.end, end_vertex);
                        }
                        Segment::Arc(arc) => {
                            let start = sketch.points.get(&arc.start).unwrap();
                            let start_vertex =
                                builder::vertex(TruckPoint3::new(start.x, start.y, start.z));
                            let end = sketch.points.get(&arc.end).unwrap();
                            let end_vertex = builder::vertex(TruckPoint3::new(end.x, end.y, end.z));
                            let center = sketch.points.get(&arc.center).unwrap();
                            let center_vertex =
                                builder::vertex(TruckPoint3::new(center.x, center.y, center.z));
                            vertices.insert(arc.start, start_vertex);
                            vertices.insert(arc.end, end_vertex);
                            vertices.insert(arc.center, center_vertex);
                        }
                    }
                }

                let mut edges: Vec<Edge> = Vec::new();
                // Now add the segments to the wire
                for segment in segments.iter() {
                    match segment {
                        Segment::Line(line) => {
                            let start_vertex = vertices.get(&line.start).unwrap();
                            let end_vertex = vertices.get(&line.end).unwrap();
                            let edge = builder::line(start_vertex, end_vertex);
                            edges.push(edge);
                        }
                        Segment::Arc(arc) => {
                            let start_point = sketch.points.get(&arc.start).unwrap();
                            let end_point = sketch.points.get(&arc.end).unwrap();
                            let center_point = sketch.points.get(&arc.center).unwrap();
                            let transit = find_transit(
                                plane,
                                start_point,
                                end_point,
                                center_point,
                                arc.clockwise,
                            );

                            let start_vertex = vertices.get(&arc.start).unwrap();
                            let end_vertex = vertices.get(&arc.end).unwrap();
                            let transit_point = TruckPoint3::new(transit.x, transit.y, transit.z);

                            // center point is not a vertex, but a point
                            let edge = builder::circle_arc(start_vertex, end_vertex, transit_point);
                            edges.push(edge);
                        }
                    }
                }

                let wire = Wire::from_iter(edges.into_iter());
                wire
            }
        }
    }

    pub fn to_obj_string(&self, tolerance: f64) -> String {
        let mesh = self.truck_solid.triangulation(tolerance).to_polygon();
        let mut buf = Vec::new();
        obj::write(&mesh, &mut buf).unwrap();
        let string = String::from_utf8(buf).unwrap();
        string
    }

    pub fn save_as_obj(&self, filename: &str, tolerance: f64) {
        let mesh = self.truck_solid.triangulation(tolerance).to_polygon();
        let file = std::fs::File::create(filename).unwrap();
        obj::write(&mesh, file).unwrap();
    }

    pub fn to_step_string(&self) -> String {
        let compressed = self.truck_solid.compress();
        let step_string = out::CompleteStepDisplay::new(
            out::StepModel::from(&compressed),
            out::StepHeaderDescriptor {
                organization_system: "cadmium-shape-to-step".to_owned(),
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
