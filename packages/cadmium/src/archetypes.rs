use tsify::Tsify;
use serde::{Deserialize, Serialize};
use truck_modeling::Plane as TruckPlane;
use truck_modeling::InnerSpace;

use crate::sketch::Point2;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum PlaneDescription {
    PlaneId(String),
    SolidFace { solid_id: String, normal: Vector3 },
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

    pub fn project(&self, point: &Point3) -> Point2 {
        let minus_origin = point.minus(&self.origin);
        let x = minus_origin.dot(&self.primary);
        let y = minus_origin.dot(&self.secondary);
        Point2::new(x, y)
    }

    pub fn unproject(&self, point: &Point2) -> Point3 {
        let x = self.origin.plus(self.primary.times(point.x));
        let y = self.origin.plus(self.secondary.times(point.y));
        x.plus(y).to_point3()
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
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub hidden: bool,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 {
            x,
            y,
            z,
            hidden: false,
        }
    }

    pub fn plus(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn minus(&self, other: &Point3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn distance_to(&self, other: &Point3) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
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
