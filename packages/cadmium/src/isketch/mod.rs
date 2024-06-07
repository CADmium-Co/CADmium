use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use isotope::decompose::face::Face;
use isotope::primitives::point2::Point2 as ISOPoint2;
use isotope::primitives::PrimitiveCell;
use isotope::sketch::Sketch;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;


use crate::IDType;
use crate::archetypes::{Plane, PlaneDescription};
use crate::error::CADmiumError;
use crate::feature::point::Point3;
use crate::message::Identifiable;
use crate::workbench::Workbench;

pub mod compound;
pub mod compound_rectangle;
pub mod face;
pub mod primitive;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ISketch {
    // TODO: Make it private with a setter
    pub plane: Rc<RefCell<Plane>>,

    sketch: Rc<RefCell<Sketch>>,
    compounds: BTreeMap<u64, Rc<RefCell<compound::Compound>>>,
    compounds_next_id: u64,
    points_3d: BTreeMap<u64, Point3>,
}

impl ISketch {
    pub fn new(plane: Rc<RefCell<Plane>>) -> Self {
        // The key difference between Sketch and RealSketch is that Sketch lives
        // in 2D and RealSketch lives in 3D. So we need to convert the points

        let mut real_sketch = Self {
            plane: plane.clone(),
            points_3d: BTreeMap::new(),
            compounds: BTreeMap::new(),
            compounds_next_id: 0,
            sketch: Rc::new(RefCell::new(Sketch::new())),
        };

        for (id, point) in real_sketch.sketch.borrow().get_all_points().iter() {
            real_sketch.points_3d.insert(*id, Point3::from_plane_point(&plane.borrow().clone(), point));
        }

        real_sketch
    }

    pub fn try_from_plane_description(wb: &Workbench, plane_description: &PlaneDescription) -> anyhow::Result<Self> {
        let plane = match plane_description {
            PlaneDescription::PlaneId(plane_id) =>
                wb.planes.get(plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?,
            PlaneDescription::SolidFace { solid_id: _, normal: _ } => todo!("Implement SolidFace"),
        }.clone();
        Ok(Self::new(plane))
    }

    /// Helper function to go from an isotope point2D to a point_3D, as calculated during new
    pub fn get_point_3d(&self, point: Rc<RefCell<ISOPoint2>>) -> Result<(u64, Point3), CADmiumError> {
        let cell = PrimitiveCell::Point2(point.clone());
        let point_id = self.sketch.borrow().get_primitive_id(&cell).unwrap();

        if let Some(result) = self.points_3d.get(&point_id) {
            Ok((point_id, result.clone()))
        } else {
            // TODO: While I'd like to calculate and add the point_3d here, we'll pollute everything with mut
            // let point_3d = Point3::from_plane_point(&self.plane.borrow(), &point.borrow());

            // Ok((point_id,
            //     self.points_3d
            //         .insert(point_id, point_3d)
            //         .ok_or(CADmiumError::Point3DCalculationFailed)?))
            Err(CADmiumError::Point3DCalculationFailed)
        }
    }

    pub fn sketch(&self) -> Rc<RefCell<Sketch>> {
        self.sketch.clone()
    }

    pub fn faces(&self) -> Vec<Face> {
        // TODO: How do we keep track of faces vs IDs?
        self.sketch.borrow().get_merged_faces()
    }

    pub fn find_point_ref(&self, x: f64, y: f64) -> Option<Rc<RefCell<ISOPoint2>>> {
        self.sketch.borrow().primitives().iter().find_map(|(_, prim)| {
            if let PrimitiveCell::Point2(point_ref) = prim {
                let point = point_ref.borrow();
                if (point.x() - x).abs() < 0.0001 && (point.y() - y).abs() < 0.0001 {
                    Some(point_ref.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

impl Identifiable for Rc<RefCell<ISketch>> {
    type Parent = Rc<RefCell<Workbench>>;
    const ID_NAME: &'static str = "sketch_id";

    fn from_parent_id(parent: &Self::Parent, id: IDType) -> anyhow::Result<Self> {
        Ok(parent.borrow().sketches.get(&id).ok_or(anyhow::anyhow!(""))?.clone())
    }
}
