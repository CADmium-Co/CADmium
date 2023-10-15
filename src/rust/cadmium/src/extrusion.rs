use serde::{Deserialize, Serialize};

use crate::project::Project;
use crate::project::Vector3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extrusion {
    pub sketch_name: String,
    pub face_ids: Vec<u64>,
    pub length: f64,
    pub offset: f64,
    pub direction: Vector3,
    // TODO: add a "mode" field for "new" vs "add" vs "remove"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_project() {
        let mut p = Project::new("Test Extrusion");
        p.add_defaults();

        let mut wb = p.get_workbench_mut("Workbench 1").unwrap();

        let e = wb.add_extrusion(
            "Ext 1",
            Extrusion {
                sketch_name: "Sketch 1".to_owned(),
                face_ids: vec![0],
                length: 1.0,
                offset: 0.0,
                direction: Vector3::new(0.0, 0.0, 1.0),
            },
        );

        // now get solids? save as obj or stl or step?
    }
}
