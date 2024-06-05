use std::cell::RefCell;
use std::rc::Rc;

use isotope::decompose::face::Face;
use serde::{Deserialize, Serialize};
use truck_modeling::builder;
use tsify_next::Tsify;

use super::prelude::*;

use crate::archetypes::Vector3;
use crate::isketch::ISketch;
use crate::message::MessageHandler;
use crate::workbench::Workbench;
use crate::IDType;

use super::get_isoface_wires;
use super::Feature;
use super::FeatureCell;
use super::SolidLike;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Mode {
    New,
    Add(Vec<IDType>),
    Remove(Vec<IDType>),
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Direction {
    Normal,
    NegativeNormal,
    Specified(Vector3),
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Extrusion {
    pub faces: Vec<Face>,
    pub sketch: Rc<RefCell<ISketch>>,
    pub length: f64,
    pub offset: f64,
    pub direction: Direction,
    pub mode: Mode,
}

impl Extrusion {
    pub fn new(
        faces: Vec<Face>,
        sketch: Rc<RefCell<ISketch>>,
        length: f64,
        offset: f64,
        direction: Direction,
        mode: Mode,
    ) -> Self {
        Extrusion {
            faces,
            sketch,
            length,
            offset,
            direction,
            mode,
        }
    }
}

impl SolidLike for Extrusion {
    fn references(&self) -> Vec<FeatureCell> {
        // self.faces.iter().map(|f| FeatureCell::Face(f.clone())).collect()
        todo!("Extrusion::references")
    }

    fn to_feature(&self) -> Feature {
        Feature::Extrusion(self.clone())
    }

    fn get_truck_solids(&self) -> anyhow::Result<Vec<TruckClosedSolid>> {
        let plane = self.sketch.borrow().plane.borrow().clone();

        let extrusion_direction = match &self.direction {
            Direction::Normal => plane.tertiary.clone(),
            Direction::NegativeNormal => plane.tertiary.times(-1.0),
            Direction::Specified(vector) => vector.clone(),
        };

        let extrusion_vector = extrusion_direction.times(self.length - self.offset);
        let offset_vector = extrusion_direction.times(self.offset);
        let extrusion_tvector = TruckVector3::new(extrusion_vector.x, extrusion_vector.y, extrusion_vector.z);
        let offset_tvector = TruckVector3::new(offset_vector.x, offset_vector.y, offset_vector.z);

        Ok(self.faces
            .iter()
            .map(|f| {
                let wires = get_isoface_wires(self.sketch.clone(), f).unwrap();
                let face = builder::try_attach_plane(&wires).unwrap();

                // Can we calculate ALL the wires at once and not iter-sweep?
                let sweep = builder::tsweep(&face, extrusion_tvector);


                builder::translated(&sweep, offset_tvector)
            }).collect())
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct Add {
    pub sketch_id: IDType,
    pub faces: Vec<Face>, // TODO: This should be a list of face IDs
    pub length: f64,
    pub offset: f64,
    pub direction: Direction,
    pub mode: Mode,
}

impl MessageHandler for Add {
    // Parent to workbench to add to solids and be able to reference the sketch
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(&self, workbench_ref: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let mut workbench = workbench_ref.borrow_mut();
        let sketch = workbench.get_sketch_by_id(self.sketch_id)?;

        let extrusion = Extrusion::new(
            self.faces.clone(),
            sketch.clone(),
            self.length,
            self.offset,
            self.direction.clone(),
            self.mode.clone(),
        );

        // TODO: This is incorrect. We should adding Features to the workbench, not solids
        // Until then we can't update or remove as we don't know which solids are associated with this extrusion
        extrusion.to_solids()?.iter().for_each(|solid| {
            let id = workbench.solids_next_id;
            workbench.solids.insert(id, Rc::new(RefCell::new(solid.clone())));
            workbench.solids_next_id += 1;
        });

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::project::tests::create_test_project;
    use crate::project::Project;

    #[test]
    #[ignore = "uses old filetype"]
    fn project_from_files() {
        let file_list = [
            // this file contains three shapes which are adjacent to each other and
            // thus should result in a single output solid
            ("src/test_inputs/three_adjacent_faces.cadmium", 1),
            // this file contains one square nested inside another
            // and thus should result in a single output solid
            ("src/test_inputs/nested_squares.cadmium", 1),
            // this file contains one circle nested inside another
            // and thus should result in a single output solid
            ("src/test_inputs/nested_circles.cadmium", 1),
            ("src/test_inputs/two_Es.cadmium", 1),
            ("src/test_inputs/lots_of_nesting.cadmium", 4),
        ];

        for (file, expected_solids) in file_list.iter() {
            let contents = std::fs::read_to_string(file).unwrap();

            // deserialize the contents into a Project
            let p: Project = serde_json::from_str(&contents).unwrap();

            // get a realization
            let workbench_ref = p.get_workbench_by_id(0).unwrap();
            let workbench = workbench_ref.borrow();
            let solids = &workbench.solids;
            println!("[{}] solids: {:?}", file, solids.len());

            assert_eq!(solids.len(), *expected_solids); // doesn't work yet!
        }
    }

    #[test]
    fn step_export() {
        let p = create_test_project();
        let workbench_ref = p.get_workbench_by_id(0).unwrap();
        let workbench = workbench_ref.borrow();
        let solid = workbench.solids.get(&0).unwrap().borrow();

        solid.save_as_step("pkg/test.step");
        solid.save_as_obj("pkg/test.obj", 0.001);
    }
}
