use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use isotope::decompose::face::Face;
use isotope::primitives::point2::Point2;
use isotope::primitives::PrimitiveCell;
use isotope::sketch::Sketch;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::archetypes::{Plane, Point3};
use crate::error::CADmiumError;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct IPlane {
    // TODO: Should hold its own ID
    // pub id: String,
    pub plane: Plane,
    pub name: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ISketch {
    pub plane_id: String,
    plane: Rc<RefCell<IPlane>>,

    // TODO: Make the sketch private
    pub sketch: Rc<RefCell<Sketch>>,
    points_3d: BTreeMap<u64, Point3>,

    // primitives: BTreeMap<u64, Rc<RefCell<Primitive>>>,
    // constraints: VecDeque<Rc<RefCell<ConstraintType>>>,

    // TODO: Make the faces private
    pub faces: Vec<Face>,
}

impl ISketch {
    // TODO: Maybe pass the plane as refcell?
    pub fn new(plane_id: &str, plane: &IPlane, sketch: Rc<RefCell<Sketch>>) -> Self {
        // The key difference between Sketch and RealSketch is that Sketch lives
        // in 2D and RealSketch lives in 3D. So we need to convert the points

        let mut real_sketch = Self {
            plane_id: plane_id.to_owned(),
            plane: Rc::new(RefCell::new(plane.clone())),
            points_3d: BTreeMap::new(),
            // primitives: sketch.borrow().primitives().iter().map(|(id, prim)| (*id, prim.borrow().to_primitive())).collect(),
            // constraints: sketch.borrow().constraints().iter().map(|c| c.borrow().get_type()).collect(),
            sketch: sketch,
            faces: vec![],
        };

        for (id, point) in real_sketch.sketch.borrow().get_all_points().iter() {
            real_sketch.points_3d.insert(*id, Self::calculate_point_3d(plane, point));
        }

        real_sketch
    }

    /// Helper function to go from an isotope point2D to a point_3D, as calculated during new
    pub fn get_point_3d(&self, point: Rc<RefCell<Point2>>) -> Result<(u64, Point3), CADmiumError> {
        let cell = PrimitiveCell::Point2(point.clone());
        let point_id = self.sketch.borrow().get_primitive_id(&cell).unwrap();

        if let Some(result) = self.points_3d.get(&point_id) {
            Ok((point_id, result.clone()))
        } else {
            // TODO: While I'd like to calculate and add the point_3d here, we'll pollute everything with mut
            // let point_3d = Self::calculate_point_3d(&self.plane.borrow(), &point.borrow());

            // Ok((point_id,
            //     self.points_3d
            //         .insert(point_id, point_3d)
            //         .ok_or(CADmiumError::Point3DCalculationFailed)?))
            Err(CADmiumError::Point3DCalculationFailed)
        }
    }

    fn calculate_point_3d(plane: &IPlane, point: &Point2) -> Point3 {
        let o = plane.plane.origin.clone();
        let x = plane.plane.primary.clone();
        let y = plane.plane.secondary.clone();

        let pt3 = o.plus(x.times(point.x())).plus(y.times(point.y()));
        Point3::new(pt3.x, pt3.y, pt3.z)
    }
}
