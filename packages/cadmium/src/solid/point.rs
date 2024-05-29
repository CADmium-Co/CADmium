use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};
use truck_polymesh::Point3 as PolyTruckPoint3;
use isotope::primitives::point2::Point2 as ISOPoint2;
use tsify::Tsify;

use crate::archetypes::{Plane, Vector3};

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

		pub fn from_plane_point(plane: &Plane, point: &ISOPoint2) -> Point3 {
        let o = plane.origin.clone();
        let x = plane.primary.clone();
        let y = plane.secondary.clone();

        let pt3 = o.plus(x.times(point.x())).plus(y.times(point.y()));
        Point3::new(pt3.x, pt3.y, pt3.z)
    }
}

impl Into<PolyTruckPoint3> for Point3 {
		fn into(self) -> PolyTruckPoint3 {
				PolyTruckPoint3 {
						x: self.x,
						y: self.y,
						z: self.z,
				}
		}
}

impl Add for Point3 {
		type Output = Point3;

		fn add(self, other: Point3) -> Point3 {
				Point3 {
						x: self.x + other.x,
						y: self.y + other.y,
						z: self.z + other.z,
						hidden: false,
				}
		}
}

impl Sub for Point3 {
		type Output = Point3;

		fn sub(self, other: Point3) -> Point3 {
				Point3 {
						x: self.x - other.x,
						y: self.y - other.y,
						z: self.z - other.z,
						hidden: false,
				}
		}
}

impl PartialEq for Point3 {
		fn eq(&self, other: &Self) -> bool {
				self.x == other.x && self.y == other.y && self.z == other.z
		}
}
