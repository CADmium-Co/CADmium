use isotope::primitives::point2::Point2 as ISOPoint2;
use tsify::Tsify;
use serde::{Deserialize, Serialize};
use truck_modeling::Plane as TruckPlane;
use truck_modeling::InnerSpace;

use crate::solid::point::Point3;
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
pub struct Point2 {
    pub x: f64,
    pub y: f64,
    pub hidden: bool,
}

impl Into<ISOPoint2> for Point2 {
    fn into(self) -> ISOPoint2 {
        ISOPoint2::new(self.x, self.y)
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
