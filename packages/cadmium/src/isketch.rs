use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use isotope::decompose::face::Face;
use isotope::primitives::line::Line;
use isotope::primitives::point2::Point2 as ISOPoint2;
use isotope::primitives::PrimitiveCell;
use isotope::sketch::Sketch;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::archetypes::{Plane, Point2, Point3};
use crate::error::CADmiumError;
use crate::IDType;

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
    // TODO: Make it private with a setter
    pub plane: Rc<RefCell<Plane>>,

    sketch: Rc<RefCell<Sketch>>,
    points_3d: BTreeMap<u64, Point3>,

    // TODO: Make the faces private
    pub faces: Vec<Face>,
}

impl ISketch {
    // TODO: Maybe pass the plane as refcell?
    pub fn new(plane: Rc<RefCell<Plane>>) -> Self {
        // The key difference between Sketch and RealSketch is that Sketch lives
        // in 2D and RealSketch lives in 3D. So we need to convert the points

        let mut real_sketch = Self {
            plane,
            points_3d: BTreeMap::new(),
            // primitives: sketch.borrow().primitives().iter().map(|(id, prim)| (*id, prim.borrow().to_primitive())).collect(),
            // constraints: sketch.borrow().constraints().iter().map(|c| c.borrow().get_type()).collect(),
            sketch: Rc::new(RefCell::new(Sketch::new())),
            faces: vec![],
        };

        for (id, point) in real_sketch.sketch.borrow().get_all_points().iter() {
            real_sketch.points_3d.insert(*id, Self::calculate_point_3d(&plane, point));
        }

        real_sketch
    }

    /// Helper function to go from an isotope point2D to a point_3D, as calculated during new
    pub fn get_point_3d(&self, point: Rc<RefCell<ISOPoint2>>) -> Result<(u64, Point3), CADmiumError> {
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

    pub fn sketch(&self) -> Rc<RefCell<Sketch>> {
        self.sketch.clone()
    }

    fn calculate_point_3d(plane_cell: &Rc<RefCell<Plane>>, point: &ISOPoint2) -> Point3 {
        let plane = plane_cell.borrow();
        let o = plane.origin.clone();
        let x = plane.primary.clone();
        let y = plane.secondary.clone();

        let pt3 = o.plus(x.times(point.x())).plus(y.times(point.y()));
        Point3::new(pt3.x, pt3.y, pt3.z)
    }
}

impl ISketch {
    pub(super) fn add_sketch_point(&mut self, point: Point2) -> Result<IDType, anyhow::Error> {
        let iso_point = PrimitiveCell::Point2(Rc::new(RefCell::new(point.into())));

        let mut sketch = self.sketch.borrow_mut();
        let point_id = sketch.add_primitive(iso_point)?;
        self.points_3d.insert(point_id, Self::calculate_point_3d(&self.plane, &point.into()));
        Ok(point_id)
    }

    pub(super) fn add_sketch_arc(&mut self, center: IDType, radius: f64, clockwise: bool, start_angle: f64, end_angle: f64) -> Result<IDType, anyhow::Error> {
        let mut sketch = self.sketch.borrow_mut();

        let center_point = if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(center).unwrap() {
            point
        } else {
            return Err(anyhow::anyhow!("Center point is not a point"));
        };

        let arc = PrimitiveCell::Arc(Rc::new(RefCell::new(isotope::primitives::arc::Arc::new(center_point.clone(), radius, clockwise, start_angle, end_angle))));

        let point_id = sketch.add_primitive(arc)?;
        Ok(point_id)
    }

    pub(super) fn add_sketch_circle(&mut self, center: IDType, radius: f64) -> Result<IDType, anyhow::Error> {
        let mut sketch = self.sketch.borrow_mut();

        let center_point = if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(center).unwrap() {
            point
        } else {
            return Err(anyhow::anyhow!("Center point is not a point"));
        };

        let circle = PrimitiveCell::Circle(Rc::new(RefCell::new(isotope::primitives::circle::Circle::new(center_point.clone(), radius))));

        let point_id = sketch.add_primitive(circle)?;
        Ok(point_id)
    }

    pub(super) fn add_sketch_line(&mut self, start: IDType, end: IDType) -> Result<IDType, anyhow::Error> {
        let mut sketch = self.sketch.borrow_mut();

        let start_point = if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(start).unwrap() {
            point
        } else {
            return Err(anyhow::anyhow!("Start point is not a point"));
        };
        let end_point = if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(end).unwrap() {
            point
        } else {
            return Err(anyhow::anyhow!("End point is not a point"));
        };

        let line = PrimitiveCell::Line(Rc::new(RefCell::new(Line::new(start_point.clone(), end_point.clone()))));

        let point_id = sketch.add_primitive(line)?;
        Ok(point_id)
    }
}
