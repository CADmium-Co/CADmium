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

use crate::archetypes::{Plane, PlaneDescription};
use crate::error::CADmiumError;
use crate::solid::point::Point3;
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
}

impl ISketch {
    // TODO: Maybe pass the plane as refcell?
    pub fn new(plane: Rc<RefCell<Plane>>) -> Self {
        // The key difference between Sketch and RealSketch is that Sketch lives
        // in 2D and RealSketch lives in 3D. So we need to convert the points

        let mut real_sketch = Self {
            plane: plane.clone(),
            points_3d: BTreeMap::new(),
            // primitives: sketch.borrow().primitives().iter().map(|(id, prim)| (*id, prim.borrow().to_primitive())).collect(),
            // constraints: sketch.borrow().constraints().iter().map(|c| c.borrow().get_type()).collect(),
            sketch: Rc::new(RefCell::new(Sketch::new())),
        };

        for (id, point) in real_sketch.sketch.borrow().get_all_points().iter() {
            real_sketch.points_3d.insert(*id, Point3::from_plane_point(&plane.borrow().clone(), point));
        }

        real_sketch
    }

    pub fn from_plane_description(wb: &Workbench, plane_description: PlaneDescription) -> Self {
        let plane = match plane_description {
            PlaneDescription::PlaneId(plane_id) =>
                wb.planes.get(&plane_id).ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?,
            PlaneDescription::SolidFace { solid_id: _, normal: _ } => todo!("Implement SolidFace"),
        }.clone();
        Self::new(plane)
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

use crate::message::{Identifiable, MessageHandler};
use crate::workbench::Workbench;

impl Identifiable for Rc<RefCell<ISketch>> {
    type Parent = Rc<RefCell<Workbench>>;
    const ID_NAME: &'static str = "sketch_id";

    fn from_parent_id(parent: &Self::Parent, id: IDType) -> anyhow::Result<Self> {
        Ok(parent.borrow().sketches.get(&id).ok_or(anyhow::anyhow!(""))?.clone())
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddPoint {
    x: f64,
    y: f64,
}

impl MessageHandler for AddPoint {
    type Parent = Rc<RefCell<ISketch>>;
    fn handle_message(&self, sketch_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let iso_point = PrimitiveCell::Point2(Rc::new(RefCell::new(ISOPoint2::new(self.x, self.y))));

        let point_id = sketch_ref.borrow().sketch().borrow_mut().add_primitive(iso_point)?;
        // self.points_3d.insert(point_id, Point3::from_plane_point(&self.plane.borrow(), &point.into()));
        Ok(Some(point_id))
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddArc {
    center: IDType,
    radius: f64,
    clockwise: bool,
    start_angle: f64,
    end_angle: f64
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
    center: IDType,
    radius: f64,
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
    start: IDType,
    end: IDType,
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
