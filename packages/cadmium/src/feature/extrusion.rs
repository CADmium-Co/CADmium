use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use truck_modeling::builder;
use tsify_next::Tsify;

use super::prelude::*;

use crate::archetypes::Vector3;
use crate::isketch::face::{FaceSelector, Selector};
use crate::isketch::ISketch;
use crate::message::{Identifiable, MessageHandler};
use crate::workbench::Workbench;
use crate::{interop, IDType};

use super::get_isoface_wires;
use super::{Feature, SolidLike};

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
    pub faces: Selector,
    pub sketch: Rc<RefCell<ISketch>>,
    pub length: f64,
    pub offset: f64,
    pub direction: Direction,
    pub mode: Mode,
}

impl Extrusion {
    pub fn new(
        faces: Vec<IDType>,
        sketch: Rc<RefCell<ISketch>>,
        length: f64,
        offset: f64,
        direction: Direction,
        mode: Mode,
    ) -> Self {
        Extrusion {
            faces: Selector::from_face_ids(&sketch.clone().borrow(), faces),
            sketch,
            length,
            offset,
            direction,
            mode,
        }
    }
}

impl SolidLike for Extrusion {
    fn references(&self) -> Vec<Rc<RefCell<Feature>>> {
        // self.faces.iter().map(|f| FeatureCell::Face(f.clone())).collect()
        todo!("Extrusion::references")
    }

    fn to_feature(&self) -> Feature {
        Feature::Extrusion(self.clone())
    }

    fn get_truck_solids(&self) -> anyhow::Result<Vec<TruckClosedSolid>> {
        let sketch = self.sketch.borrow();
        let plane = sketch.plane.borrow().clone();

        let extrusion_direction = match &self.direction {
            Direction::Normal => plane.tertiary.clone(),
            Direction::NegativeNormal => plane.tertiary.times(-1.0),
            Direction::Specified(vector) => vector.clone(),
        };

        let extrusion_vector = extrusion_direction.times(self.length - self.offset);
        let offset_vector = extrusion_direction.times(self.offset);
        let extrusion_tvector =
            TruckVector3::new(extrusion_vector.x, extrusion_vector.y, extrusion_vector.z);
        let offset_tvector = TruckVector3::new(offset_vector.x, offset_vector.y, offset_vector.z);

        Ok(self
            .faces
            .get_selected_faces(&sketch)
            .iter()
            .map(|f| {
                let wires = get_isoface_wires(self.sketch.clone(), &f).unwrap();
                let face = builder::try_attach_plane(&wires).unwrap();

                // Can we calculate ALL the wires at once and not iter-sweep?
                let sweep = builder::tsweep(&face, extrusion_tvector);

                builder::translated(&sweep, offset_tvector)
            })
            .collect())
    }
}

impl<'a> TryFrom<&'a mut Feature> for &'a mut Extrusion {
    type Error = anyhow::Error;

    // The Feature enum has only 1 variant for now but that will change soon
    #[allow(irrefutable_let_patterns)]
    fn try_from(value: &'a mut Feature) -> Result<Self, Self::Error> {
        let Feature::Extrusion(ref mut extrusion) = value else {
            return Err(anyhow::anyhow!("Failed to convert Feature to Extrusion"));
        };

        Ok(extrusion)
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct Add {
    pub sketch_id: IDType,
    pub faces: Vec<IDType>,
    pub length: f64,
    pub offset: f64,
    pub direction: Direction,
    pub mode: Mode,
}

impl MessageHandler for Add {
    // Parent to workbench to add to solids and be able to reference the sketch
    type Parent = Rc<RefCell<Workbench>>;
    fn handle_message(
        &self,
        workbench_ref: Self::Parent,
    ) -> anyhow::Result<Option<(IDType, interop::Node)>> {
        let sketch = <Rc<RefCell<ISketch>>>::from_parent_id(&workbench_ref, self.sketch_id)?;
        let mut workbench = workbench_ref.borrow_mut();

        let extrusion = Extrusion::new(
            self.faces.clone(),
            sketch.clone(),
            self.length,
            self.offset,
            self.direction.clone(),
            self.mode.clone(),
        );
        let extrusion_cell = Rc::new(RefCell::new(extrusion.to_feature()));

        let id = workbench.features_next_id;
        workbench.features.insert(id, extrusion_cell);
        workbench.features_next_id += 1;
        let id = workbench.features_next_id - 1;

        Ok(Some((id, interop::Node::Solid(extrusion.to_solids()?))))
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
            let solids = &workbench.features;
            println!("[{}] solids: {:?}", file, solids.len());

            assert_eq!(solids.len(), *expected_solids); // doesn't work yet!
        }
    }

    #[test]
    fn step_export() {
        let p = create_test_project();
        let workbench_ref = p.get_workbench_by_id(0).unwrap();
        let workbench = workbench_ref.borrow();
        let feature = workbench.features.get(&0).unwrap().borrow();
        let solid = &feature.as_solid_like().to_solids().unwrap()[0];

        solid.save_as_step("pkg/test.step");
        solid.save_as_obj("pkg/test.obj", 0.001);
    }
}
