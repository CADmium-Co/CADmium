pub use super::helpers::*;
pub use super::solid::Solid;

pub const MESH_TOLERANCE: f64 = 0.1;

pub use isotope::decompose::face::Face as ISOFace;

pub use truck_modeling::Face as TruckFace;
pub use truck_modeling::Line as TruckLine;
pub use truck_modeling::Plane as TruckPlane;
pub use truck_modeling::Point1 as TruckPoint1;
pub use truck_modeling::Point2 as TruckPoint2;
pub use truck_modeling::Point3 as TruckPoint3;
pub use truck_modeling::Solid as TruckSolid;
pub use truck_modeling::Surface as TruckSurface;
pub use truck_modeling::Vector1 as TruckVector1;
pub use truck_modeling::Vector2 as TruckVector2;
pub use truck_modeling::Vector3 as TruckVector3;
pub use truck_modeling::Vertex as TruckVertex;

pub use truck_topology::Solid as TruckTopoSolid;

pub type TruckClosedSolid =
    TruckTopoSolid<truck_modeling::Point3, truck_modeling::Curve, truck_modeling::Surface>;
