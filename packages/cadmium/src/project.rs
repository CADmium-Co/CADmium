use std::cell::RefCell;
use std::rc::Rc;

use log::error;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::Plane;
use crate::error::CADmiumError;
use crate::message::idwrap::IDWrap;
use crate::message::ProjectMessageHandler;
use crate::step::StepHash;
use crate::workbench::{AddPlane, AddPoint, Workbench};

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Assembly {
    name: String,
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
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

        p.add_workbench("Workbench 1");

        p
    }

    pub fn add_workbench(&mut self, name: &str) {
        let wb = Workbench::new(name);
        let wb_cell = Rc::new(RefCell::new(wb));
        self.workbenches.push(wb_cell.clone());
        let wb_id = self.workbenches.len() as u64 - 1;
        let wb_hash = StepHash::from_int(wb_id);

        IDWrap {
            id: wb_hash.clone(),
            inner: AddPoint {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
        .handle_project_message(self)
        .unwrap()
        .unwrap();

        for plane_inst in [Plane::top(), Plane::front(), Plane::right()].iter() {
            IDWrap {
                id: wb_hash.clone(),
                inner: AddPlane {
                    plane: plane_inst.clone(),
                    width: 100.0,
                    height: 100.0,
                },
            }
            .handle_project_message(self)
            .unwrap()
            .unwrap();
        }

        let wb_ref = wb_cell.borrow();
        wb_ref.history.get(0).unwrap().borrow_mut().name = "Origin".to_string();
        wb_ref.history.get(1).unwrap().borrow_mut().name = "Top Plane".to_string();
        wb_ref.history.get(2).unwrap().borrow_mut().name = "Front Plane".to_string();
        wb_ref.history.get(3).unwrap().borrow_mut().name = "Right Plane".to_string();
        assert_eq!(wb_ref.history.len(), 4);
        assert_eq!(wb_ref.points.len(), 1);
        assert_eq!(wb_ref.planes.len(), 3);
        assert_eq!(wb_ref.points_next_id, 1);
        assert_eq!(wb_ref.planes_next_id, 3);
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
                error!("Error: {}", e);
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
            .get(id as usize)
            .cloned()
            .ok_or(CADmiumError::WorkbenchIDNotFound(id))
    }

    pub fn get_workbench_by_name(
        &self,
        name: &str,
    ) -> Result<Rc<RefCell<Workbench>>, CADmiumError> {
        self.workbenches
            .iter()
            .find(|wb| wb.borrow().name == name)
            .cloned()
            .ok_or(CADmiumError::WorkbenchNameNotFound(name.to_string()))
    }
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct ProjectRename {
    pub new_name: String,
}

impl ProjectMessageHandler for ProjectRename {
    fn handle_project_message(
        &self,
        project: &mut crate::project::Project,
    ) -> anyhow::Result<Option<StepHash>> {
        project.name = self.new_name.clone();
        Ok(None)
    }
}

#[cfg(test)]
pub mod tests {

    use crate::archetypes::PlaneDescription;

    use crate::feature::extrusion;
    use crate::feature::extrusion::{Direction, Mode};
    use crate::isketch::primitive::{AddLine, SketchAddPointMessage};
    use crate::message::idwrap::IDWrap;
    use crate::message::MessageHandler;
    use crate::step;
    use crate::workbench::{AddSketch, SetSketchPlane};

    use super::*;

    pub const WORKBENCH_HASH: StepHash = StepHash::from_int(0);

    pub fn create_test_project() -> Project {
        let mut p = Project::new("Test Project");
        let top_hash = p
            .workbenches
            .get(0)
            .unwrap()
            .borrow()
            .history
            .get(1)
            .unwrap()
            .borrow()
            .hash();
        let plane_description = PlaneDescription::PlaneId(top_hash);
        let sketch_id = IDWrap {
            id: WORKBENCH_HASH,
            inner: AddSketch { plane_description },
        }
        .handle_project_message(&mut p)
        .unwrap()
        .unwrap();

        add_test_rectangle(&mut p, sketch_id, 0.0, 0.0, 40.0, 40.0);

        IDWrap {
            id: WORKBENCH_HASH,
            inner: extrusion::Add {
                sketch_id,
                faces: vec![0],
                length: 25.0,
                offset: 0.0,
                direction: Direction::Normal,
                mode: Mode::New,
            },
        }
        .handle_project_message(&mut p)
        .unwrap()
        .unwrap();

        p
    }

    pub fn add_test_rectangle(
        p: &mut Project,
        sketch_id: StepHash,
        x_start: f64,
        y_start: f64,
        x_end: f64,
        y_end: f64,
    ) {
        let ll = IDWrap {
            id: WORKBENCH_HASH,
            inner: IDWrap {
                id: sketch_id,
                inner: SketchAddPointMessage {
                    x: x_start,
                    y: y_start,
                },
            },
        }
        .handle_project_message(p)
        .unwrap()
        .unwrap();
        let lr = IDWrap {
            id: WORKBENCH_HASH,
            inner: IDWrap {
                id: sketch_id,
                inner: SketchAddPointMessage {
                    x: x_end,
                    y: y_start,
                },
            },
        }
        .handle_project_message(p)
        .unwrap()
        .unwrap();
        let ul = IDWrap {
            id: WORKBENCH_HASH,
            inner: IDWrap {
                id: sketch_id,
                inner: SketchAddPointMessage {
                    x: x_start,
                    y: y_end,
                },
            },
        }
        .handle_project_message(p)
        .unwrap()
        .unwrap();
        let ur = IDWrap {
            id: WORKBENCH_HASH,
            inner: IDWrap {
                id: sketch_id,
                inner: SketchAddPointMessage { x: x_end, y: y_end },
            },
        }
        .handle_project_message(p)
        .unwrap()
        .unwrap();

        IDWrap {
            id: WORKBENCH_HASH,
            inner: IDWrap {
                id: sketch_id,
                inner: AddLine { start: ll, end: lr },
            },
        }
        .handle_project_message(p)
        .unwrap()
        .unwrap();
        IDWrap {
            id: WORKBENCH_HASH,
            inner: IDWrap {
                id: sketch_id,
                inner: AddLine { start: lr, end: ur },
            },
        }
        .handle_project_message(p)
        .unwrap()
        .unwrap();
        IDWrap {
            id: WORKBENCH_HASH,
            inner: IDWrap {
                id: sketch_id,
                inner: AddLine { start: ur, end: ul },
            },
        }
        .handle_project_message(p)
        .unwrap()
        .unwrap();
        IDWrap {
            id: WORKBENCH_HASH,
            inner: IDWrap {
                id: sketch_id,
                inner: AddLine { start: ul, end: ll },
            },
        }
        .handle_project_message(p)
        .unwrap()
        .unwrap();
    }

    #[test]
    fn one_extrusion() {
        let p = create_test_project();

        let workbench_ref = p.get_workbench_by_id(0).unwrap();
        let workbench = workbench_ref.borrow();
        let solids = &workbench.features;
        println!("solids: {:?}", solids);

        assert_eq!(solids.len(), 1);
    }

    #[test]
    fn move_sketch() {
        let p = create_test_project();
        let workbench_ref = p.get_workbench_by_id(0).unwrap();
        let front_hash = p
            .workbenches
            .get(0)
            .unwrap()
            .borrow()
            .history
            .get(2)
            .unwrap()
            .borrow()
            .hash();

        SetSketchPlane {
            sketch_id: 0,
            plane_description: PlaneDescription::PlaneId(front_hash),
        }
        .handle_message(workbench_ref.clone())
        .unwrap();
    }

    #[test]
    fn rename_step() {
        let p = create_test_project();
        let workbench_ref = p.get_workbench_by_id(0).unwrap();
        let workbench = workbench_ref.borrow();
        let new_name = "New Extrusion Name".to_string();
        let target = workbench.history.last().unwrap();

        step::actions::Rename {
            new_name: new_name.clone(),
        }
        .handle_message(target.clone())
        .unwrap();

        assert_eq!(target.borrow().name, new_name);
    }

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
