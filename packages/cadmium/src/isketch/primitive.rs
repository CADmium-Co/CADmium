use std::cell::RefCell;
use std::rc::Rc;

use cadmium_macros::message;
use isotope::primitives::line::Line;
use isotope::primitives::point2::Point2 as ISOPoint2;
use isotope::primitives::PrimitiveCell;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::message::MessageHandler;
use crate::IDType;
use crate::feature::point::Point3;

use super::ISketch;

#[message(ISketch, rename_parent = "Sketch")]
pub fn add_point(&mut self, x: f64, y: f64) -> anyhow::Result<Option<IDType>> {
    let iso_point = ISOPoint2::new(x, y);
    let iso_point_cell = PrimitiveCell::Point2(Rc::new(RefCell::new(iso_point.clone())));

    // TODO: On plane change the 3D points have to be recalculated
    let plane = self.plane.borrow().clone();
    let point_id = self.sketch().borrow_mut().add_primitive(iso_point_cell)?;
    self.points_3d.insert(point_id, Point3::from_plane_point(&plane, &iso_point));
    Ok(Some(point_id))
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddArc {
    pub center: IDType,
    pub radius: f64,
    pub clockwise: bool,
    pub start_angle: f64,
    pub end_angle: f64
}

impl MessageHandler for AddArc {
    type Parent = Rc<RefCell<ISketch>>;
    fn handle_message(&self, sketch_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let isketch = sketch_ref.borrow();
        let mut sketch = isketch.sketch.borrow_mut();

        let center_point = if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(self.center).unwrap() {
            point
        } else {
            return Err(anyhow::anyhow!("Center point is not a point"));
        };

        let arc = PrimitiveCell::Arc(Rc::new(RefCell::new(isotope::primitives::arc::Arc::new(center_point.clone(), self.radius, self.clockwise, self.start_angle, self.end_angle))));

        let point_id = sketch.add_primitive(arc)?;
        Ok(Some(point_id))
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddCircle {
    pub center: IDType,
    pub radius: f64,
}

impl MessageHandler for AddCircle {
    type Parent = Rc<RefCell<ISketch>>;
    fn handle_message(&self, sketch_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let isketch = sketch_ref.borrow();
        let mut sketch = isketch.sketch.borrow_mut();

        let center_point = if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(self.center).unwrap() {
            point
        } else {
            return Err(anyhow::anyhow!("Center point is not a point"));
        };

        let circle = PrimitiveCell::Circle(Rc::new(RefCell::new(isotope::primitives::circle::Circle::new(center_point.clone(), self.radius))));

        let point_id = sketch.add_primitive(circle)?;
        Ok(Some(point_id))
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddLine {
    pub start: IDType,
    pub end: IDType,
}

impl MessageHandler for AddLine {
    type Parent = Rc<RefCell<ISketch>>;
    fn handle_message(&self, sketch_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let isketch = sketch_ref.borrow();
        let mut sketch = isketch.sketch.borrow_mut();

        let start_point = if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(self.start).unwrap() {
            point
        } else {
            return Err(anyhow::anyhow!("Start point is not a point"));
        };
        let end_point = if let PrimitiveCell::Point2(point) = sketch.get_primitive_by_id(self.end).unwrap() {
            point
        } else {
            return Err(anyhow::anyhow!("End point is not a point"));
        };

        let line = PrimitiveCell::Line(Rc::new(RefCell::new(Line::new(start_point.clone(), end_point.clone()))));

        let point_id = sketch.add_primitive(line)?;
        Ok(Some(point_id))
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct DeletePrimitive {
    id: IDType,
}

impl MessageHandler for DeletePrimitive {
    type Parent = Rc<RefCell<ISketch>>;
    fn handle_message(&self, sketch_ref: Rc<RefCell<ISketch>>) -> anyhow::Result<Option<IDType>> {
        let isketch = sketch_ref.borrow();
        let mut sketch = isketch.sketch.borrow_mut();

        sketch.delete_primitive(self.id)?;
        Ok(None)
    }
}
