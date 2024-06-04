use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::error::CADmiumError;
use crate::message::ProjectMessageHandler;
use crate::workbench::Workbench;

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Assembly {
    name: String,
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Project {
    pub name: String,
    pub assemblies: Vec<Assembly>,
    pub workbenches: Vec<Rc<RefCell<Workbench>>>,
}

impl Project {
    pub fn new(name: &str) -> Self {
        let mut p = Project {
            name: name.to_owned(),
            assemblies: vec![],
            workbenches: vec![],
        };

        let wb = Workbench::new("Workbench 1");
        p.workbenches.push(Rc::new(RefCell::new(wb)));

        p
    }

    pub fn json(&self) -> String {
        let result = serde_json::to_string(self);
        match result {
            Ok(json) => json,
            Err(e) => format!("Error: {}", e),
        }
    }

    pub fn from_json(json: &str) -> Self {
        let result = serde_json::from_str(json);
        match result {
            Ok(p) => p,
            Err(e) => {
                println!("Error: {}", e);
                Project::new("Error")
            }
        }
    }

    // pub fn compute_constraint_errors(&mut self) {
    //     for workbench in self.workbenches.iter_mut() {
    //         for step in workbench.history.iter_mut() {
    //             match &mut step.data {
    //                 StepData::Sketch { sketch, .. } => {
    //                     sketch.compute_constraint_errors();
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    // }

    pub fn get_workbench_by_id(&self, id: u64) -> Result<Rc<RefCell<Workbench>>, CADmiumError> {
        self.workbenches
            .get(id as usize).cloned()
            .ok_or(CADmiumError::WorkbenchIDNotFound(id))
    }

    pub fn get_workbench_by_name(&self, name: &str) -> Result<Rc<RefCell<Workbench>>, CADmiumError> {
        self.workbenches
            .iter()
            .find(|wb| wb.borrow().name == name).cloned()
            .ok_or(CADmiumError::WorkbenchNameNotFound(name.to_string()))
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct ProjectRename {
    new_name: String,
}

impl ProjectMessageHandler for ProjectRename {
    fn handle_project_message(&self, project: &mut crate::project::Project) -> anyhow::Result<Option<crate::IDType>> {
        project.name = self.new_name.clone();
        Ok(None)
    }
}


#[cfg(test)]
pub mod tests {

    use crate::archetypes::PlaneDescription;
    
    use crate::isketch::AddLine;
    use crate::isketch::AddPoint;
    use crate::message::MessageHandler;
    use crate::solid::extrusion;
    use crate::solid::extrusion::Direction;
    use crate::solid::extrusion::Mode;
    use crate::workbench::AddSketch;

    use super::*;

    pub fn create_test_project() -> Project {
        let p = Project::new("Test Project");
        let wb = p.workbenches.first().unwrap();
        let plane_description = PlaneDescription::PlaneId(0);
        let sketch_id = AddSketch { plane_description }.handle_message(wb.clone()).unwrap().unwrap();
        let sketch = wb.borrow().get_sketch_by_id(sketch_id).unwrap();

        let ll = AddPoint { x: 0.0, y: 0.0 }.handle_message(sketch.clone()).unwrap().unwrap();
        let lr = AddPoint { x: 40.0, y: 0.0 }.handle_message(sketch.clone()).unwrap().unwrap();
        let ul = AddPoint { x: 0.0, y: 40.0 }.handle_message(sketch.clone()).unwrap().unwrap();
        let ur = AddPoint { x: 40.0, y: 40.0 }.handle_message(sketch.clone()).unwrap().unwrap();

        AddLine { start: ll, end: lr }.handle_message(sketch.clone()).unwrap();
        AddLine { start: lr, end: ur }.handle_message(sketch.clone()).unwrap();
        AddLine { start: ur, end: ul }.handle_message(sketch.clone()).unwrap();
        AddLine { start: ul, end: ll }.handle_message(sketch.clone()).unwrap();

        let faces = sketch.borrow().sketch().borrow().get_faces();
        extrusion::Add { sketch_id, faces, length: 25.0, offset: 0.0, direction: Direction::Normal, mode: Mode::New }.handle_message(wb.clone()).unwrap();

        p
    }

    #[test]
    fn one_extrusion() {
        let p = create_test_project();

        let workbench_ref = p.get_workbench_by_id(0).unwrap();
        let workbench = workbench_ref.borrow();
        let solids = &workbench.solids;
        println!("solids: {:?}", solids);

        assert_eq!(solids.len(), 1);
    }

    // #[test]
    // fn move_sketch() {
    //     let mut p = Project::new("Test Project");

    //     let right_plane_id = p.workbenches[0].plane_name_to_id("Right").unwrap();

    //     let message = &Message::SetSketchPlane {
    //         workbench_id: 0,
    //         sketch_id: "Sketch-0".to_owned(),
    //         plane_id: right_plane_id,
    //     };

    //     let result = p.handle_message(message);
    //     match result {
    //         Ok(res) => println!("{}", res),
    //         Err(e) => println!("{}", e),
    //     }
    //     // println!("{:?}", result);

    //     let realization = p.get_realization(0, 1000);
    // }

    // #[test]
    // fn rename_plane() {
    //     let mut p = create_test_project();

    //     let message = &Message::RenameStep {
    //         workbench_id: 0,
    //         step_id: 1,
    //         new_name: "Top-2".to_owned(),
    //     };

    //     let result = message.handle(&mut p);
    //     match result {
    //         Ok(res) => println!("{}", res),
    //         Err(e) => println!("{}", e),
    //     }
    //     // let realization = p.get_realization(0, 1000);
    // }

    #[test]
    #[ignore = "uses old filetype"]
    fn circle_crashing() {
        let file_contents =
            std::fs::read_to_string("src/test_inputs/circle_crashing_2.cadmium").unwrap();

        let p = Project::from_json(&file_contents);

        let workbench_ref = p.get_workbench_by_id(0).unwrap();
        let workbench = workbench_ref.borrow();
        println!("{:?}", workbench);
    }

    // #[test]
    // fn bruno() {
    //     let mut p = create_test_project();
    //     let wb = p.workbenches.get_mut(0).unwrap();

    //     let s2_id = wb.add_sketch_to_solid_face("Sketch-2", "Ext1:0", Vector3::new(0.0, 0.0, 1.0));
    //     let s2 = wb.get_sketch_mut("Sketch-2").unwrap();

    //     // smaller
    //     let ll = s2.add_point(12.0, 12.0);
    //     let lr = s2.add_point(32.0, 12.0);
    //     let ul = s2.add_point(12.0, 32.0);
    //     let ur = s2.add_point(32.0, 32.0);
    //     // bigger!
    //     // let ll = s2.add_point(-10.0, -10.0);
    //     // let lr = s2.add_point(50.0, -10.0);
    //     // let ul = s2.add_point(-10.0, 50.0);
    //     // let ur = s2.add_point(50.0, 50.0);
    //     s2.add_segment(ll, lr);
    //     s2.add_segment(lr, ur);
    //     s2.add_segment(ur, ul);
    //     s2.add_segment(ul, ll);

    //     // println!("S2: {:?}", s2);

    //     let extrusion2 = Extrusion::new(
    //         s2_id.to_owned(),
    //         vec![0],
    //         25.0,
    //         0.0,
    //         Direction::Normal,
    //         ExtrusionMode::Add(vec!["Ext1:0".to_string()]),
    //     );
    //     wb.add_extrusion("Ext2", extrusion2);

    //     wb.add_sketch_to_plane("Sketch 3", "Plane-1");
    //     let s3 = wb.get_sketch_mut("Sketch 3").unwrap();
    //     let center = s3.add_point(20.0, 15.0);
    //     s3.add_circle(center, 5.0);

    //     let extrusion3 = Extrusion::new(
    //         "Sketch-2".to_owned(),
    //         vec![0],
    //         50.0,
    //         0.0,
    //         Direction::NegativeNormal,
    //         ExtrusionMode::Remove(vec!["Ext1:0".to_string()]),
    //     );
    //     wb.add_extrusion("Ext3", extrusion3);

    //     let realization = p.get_realization(0, 1000);
    //     let solids = realization.solids;

    //     let num_solids = solids.len();
    //     println!("Num Solids: {:?}", num_solids);
    //     assert!(num_solids == 1);

    //     let final_solid = &solids["Ext1:0"];
    //     println!("Final solid: {:?}", final_solid.truck_solid);
    //     let mut mesh = final_solid.truck_solid.triangulation(0.02).to_polygon();
    //     mesh.put_together_same_attrs();
    //     let file = std::fs::File::create("pkg/bruno.obj").unwrap();
    //     obj::write(&mesh, file).unwrap();

    //     let file = std::fs::File::create("pkg/bruno.json").unwrap();
    //     serde_json::to_writer(file, &p).unwrap();
    // }

    // #[test]
    // fn secondary_extrusion_with_merge() {
    //     let mut p = create_test_project();
    //     let wb = p.workbenches.get_mut(0).unwrap();

    //     let s2_id = wb.add_sketch_to_solid_face("Sketch-2", "Ext1:0", Vector3::new(0.0, 0.0, 1.0));
    //     let s2 = wb.get_sketch_mut("Sketch-2").unwrap();

    //     // smaller
    //     let ll = s2.add_point(12.0, 0.0);
    //     let lr = s2.add_point(32.0, 0.0);
    //     let ul = s2.add_point(12.0, 32.0);
    //     let ur = s2.add_point(32.0, 32.0);
    //     s2.add_segment(ll, lr);
    //     s2.add_segment(lr, ur);
    //     s2.add_segment(ur, ul);
    //     s2.add_segment(ul, ll);

    //     // println!("S2: {:?}", s2);

    //     let extrusion2 = Extrusion::new(
    //         s2_id.to_owned(),
    //         vec![0],
    //         25.0,
    //         0.0,
    //         Direction::Normal,
    //         ExtrusionMode::Add(vec!["Ext1:0".to_string()]),
    //     );
    //     wb.add_extrusion("Ext2", extrusion2);

    //     let realization = p.get_realization(0, 1000);
    //     let solids = realization.solids;

    //     let num_solids = solids.len();
    //     println!("Num Solids: {:?}", num_solids);
    //     assert!(num_solids == 1);

    //     let final_solid = &solids["Ext1:0"];
    //     let mut mesh = final_solid.truck_solid.triangulation(0.02).to_polygon();
    //     mesh.put_together_same_attrs();
    //     let file = std::fs::File::create("secondary_extrusion.obj").unwrap();
    //     obj::write(&mesh, file).unwrap();

    //     let file = std::fs::File::create("secondary_extrusion.json").unwrap();
    //     serde_json::to_writer(file, &p).unwrap();
    // }
}
