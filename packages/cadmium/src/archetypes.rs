use isotope::primitives::arc::Arc as ISOArc;
use isotope::primitives::circle::Circle as ISOCircle;
use isotope::primitives::line::Line as ISOLine;
use isotope::primitives::point2::Point2 as ISOPoint2;
use serde::{Deserialize, Serialize};
use truck_modeling::InnerSpace;
use truck_modeling::Plane as TruckPlane;
use tsify_next::Tsify;

use crate::feature::point::Point3;
use crate::step::StepHash;
use crate::IDType;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PlaneDescription {
    PlaneId(IDType),
    SolidFace { solid_id: IDType, normal: Vector3 },
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Plane {
    pub origin: Point3,
    pub primary: Vector3,
    pub secondary: Vector3,
    pub tertiary: Vector3, // aka Normal
}

impl Plane {
    /*

    z    y

    ^   ^
    |  /
    | /
    |/
    |-------->  x

    So "front" is xz plane with -y normal
    and "top" is xy plane with z normal
    and "right" is yz plane with x normal

     */

    pub fn new(origin: Point3, primary: Vector3, secondary: Vector3, tertiary: Vector3) -> Self {
        Plane {
            origin,
            primary,
            secondary,
            tertiary,
        }
    }

    pub fn front() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(1.0, 0.0, 0.0),
            secondary: Vector3::new(0.0, 0.0, 1.0),
            tertiary: Vector3::new(0.0, -1.0, 0.0),
        }
    }

    pub fn top() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(1.0, 0.0, 0.0),
            secondary: Vector3::new(0.0, 1.0, 0.0),
            tertiary: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn right() -> Self {
        Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            primary: Vector3::new(0.0, 1.0, 0.0),
            secondary: Vector3::new(0.0, 0.0, 1.0),
            tertiary: Vector3::new(1.0, 0.0, 0.0),
        }
    }

    pub fn from_truck(tp: TruckPlane) -> Self {
        let o = tp.origin();
        let u = tp.u_axis().normalize();
        let v = tp.v_axis().normalize();
        let n = tp.normal().normalize();
        Plane {
            origin: Point3::new(o.x, o.y, o.z),
            primary: Vector3::new(u.x, u.y, u.z),
            secondary: Vector3::new(v.x, v.y, v.z),
            tertiary: Vector3::new(n.x, n.y, n.z),
        }
    }

    pub fn project(&self, point: &Point3) -> ISOPoint2 {
        let minus_origin = point.minus(&self.origin);
        let x = minus_origin.dot(&self.primary);
        let y = minus_origin.dot(&self.secondary);
        ISOPoint2::new(x, y)
    }

    pub fn unproject(&self, point: &ISOPoint2) -> Point3 {
        let x = self.origin.plus(self.primary.times(point.x()));
        let y = self.origin.plus(self.secondary.times(point.y()));
        x.plus(y).to_point3()
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn to_point3(&self) -> Point3 {
        Point3::new(self.x, self.y, self.z)
    }

    pub fn times(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }

    pub fn plus(&self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Line3 {
    pub start: u64,
    pub end: u64,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Arc3 {
    pub center: u64,
    pub start: u64,
    pub end: u64,
    pub clockwise: bool,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Circle3 {
    pub center: u64,
    pub radius: f64,
    pub top: u64,
}

// --- ISOtope wrappers ---
pub trait FromSketchPrimitive<T> {
    fn from_sketch(sketch: &isotope::sketch::Sketch, primitive: &T) -> Self;
}
#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Point2 {
    pub x: f64,
    pub y: f64,
    pub hidden: bool,
}

impl From<Point2> for ISOPoint2 {
    fn from(val: Point2) -> Self {
        ISOPoint2::new(val.x, val.y)
    }
}

impl FromSketchPrimitive<ISOPoint2> for Point2 {
    fn from_sketch(_sketch: &isotope::sketch::Sketch, primitive: &ISOPoint2) -> Self {
        Self {
            x: primitive.x(),
            y: primitive.y(),
            hidden: false,
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Arc2 {
    pub center: IDType,
    pub radius: f64,
    pub clockwise: bool,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl FromSketchPrimitive<ISOArc> for Arc2 {
    fn from_sketch(sketch: &isotope::sketch::Sketch, primitive: &ISOArc) -> Self {
        let center = sketch
            .get_primitive_id(&isotope::primitives::PrimitiveCell::Point2(
                primitive.center(),
            ))
            .unwrap();
        Self {
            center,
            radius: primitive.radius(),
            clockwise: primitive.clockwise(),
            start_angle: primitive.start_angle(),
            end_angle: primitive.end_angle(),
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Circle2 {
    pub center: IDType,
    pub radius: f64,
}

impl FromSketchPrimitive<ISOCircle> for Circle2 {
    fn from_sketch(sketch: &isotope::sketch::Sketch, primitive: &ISOCircle) -> Self {
        let center = sketch
            .get_primitive_id(&isotope::primitives::PrimitiveCell::Point2(
                primitive.center(),
            ))
            .unwrap();
        Self {
            center,
            radius: primitive.radius(),
        }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Line2 {
    pub start: StepHash,
    pub end: StepHash,
}

impl FromSketchPrimitive<ISOLine> for Line2 {
    fn from_sketch(sketch: &isotope::sketch::Sketch, primitive: &ISOLine) -> Self {
        let start_prim = sketch
            .get_primitive_id(&isotope::primitives::PrimitiveCell::Point2(
                primitive.start(),
            ))
            .unwrap();
        let end_prim = sketch
            .get_primitive_id(&isotope::primitives::PrimitiveCell::Point2(primitive.end()))
            .unwrap();

        Self { start, end }
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum WrappedPrimitive {
    Point2(Point2),
    Line2(Line2),
    Arc2(Arc2),
    Circle2(Circle2),
}

impl WrappedPrimitive {
    pub fn from_sketch(
        sketch: &isotope::sketch::Sketch,
        primitive: &isotope::primitives::Primitive,
    ) -> Self {
        match primitive {
            isotope::primitives::Primitive::Point2(p) => {
                WrappedPrimitive::Point2(Point2::from_sketch(sketch, p))
            }
            isotope::primitives::Primitive::Line(l) => {
                WrappedPrimitive::Line2(Line2::from_sketch(sketch, l))
            }
            isotope::primitives::Primitive::Arc(a) => {
                WrappedPrimitive::Arc2(Arc2::from_sketch(sketch, a))
            }
            isotope::primitives::Primitive::Circle(c) => {
                WrappedPrimitive::Circle2(Circle2::from_sketch(sketch, c))
            }
        }
    }
}
