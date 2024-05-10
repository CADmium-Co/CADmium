use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::process::id;
use std::vec;
use truck_polymesh::faces;

use crate::project::{Plane, PlaneDescription, Project, StepData, Workbench};
use crate::sketch::Face;

pub type Sha = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpLog {
    commits: Vec<Commit>,
    commits_by_sha: HashMap<Sha, usize>,
}

impl OpLog {
    pub fn new() -> Self {
        Self {
            commits: vec![],
            commits_by_sha: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        let creation_commit = Commit::init();
        self.commits_by_sha
            .insert(creation_commit.id.clone(), self.commits.len());
        self.commits.push(creation_commit);
    }

    pub fn append(&mut self, parent: &Sha, operation: Operation) -> Commit {
        let op_hash = operation.hash();
        let parent = parent.clone();
        let new_commit = Commit {
            id: id_from_op_and_parent(&operation, &parent, self.commits.len()),
            operation,
            content_hash: op_hash,
            parent,
        };

        self.commits_by_sha
            .insert(new_commit.id.clone(), self.commits.len());
        self.commits.push(new_commit.clone());

        new_commit
    }

    pub fn last(&self) -> Option<Commit> {
        match self.commits.last() {
            Some(commit) => Some(commit.clone()),
            None => None,
        }
    }

    pub fn get_length(&self) -> usize {
        self.commits.len()
    }
}

fn id_from_op_and_parent(operation: &Operation, parent: &Sha, nonce: usize) -> Sha {
    let h = operation.hash();
    let mut hasher = Sha256::new();
    hasher.update(format!("{h}-{parent}-{nonce}").as_bytes());
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvolutionLog {
    pub cursor: Sha,
    pub oplog: OpLog,
    pub project: Project,
    pub workbenches: HashMap<Sha, usize>,
    pub planes: HashMap<Sha, (usize, String)>,
    pub sketches: HashMap<Sha, (usize, String)>,
    pub extrusions: HashMap<Sha, (usize, usize)>,
}

impl EvolutionLog {
    pub fn new() -> Self {
        let mut ol = OpLog::new();
        ol.init();
        Self {
            cursor: ol.last().unwrap().id.clone(),
            oplog: ol,
            project: Project::new("Untitled"),
            workbenches: HashMap::new(),
            planes: HashMap::new(),
            sketches: HashMap::new(),
            extrusions: HashMap::new(),
        }
    }

    pub fn append(&mut self, operation: Operation) -> Sha {
        self.cursor = self.oplog.append(&self.cursor, operation.clone()).id;

        match operation {
            Operation::CreateWorkbench { nonce } => {
                let w = Workbench::new(&nonce);
                self.project.workbenches.push(w);
                let index = self.project.workbenches.len() - 1;
                self.workbenches.insert(self.cursor.clone(), index);
                // self.workbenches_inverse.insert(index, self.cursor.clone());
            }
            Operation::SetWorkbenchName { workbench_id, name } => {
                let workbench_index = self.workbenches.get(&workbench_id).unwrap();
                self.project.workbenches[*workbench_index].name = name.clone();
            }
            Operation::CreatePlane {
                nonce,
                workbench_id,
            } => {
                let workbench_index = self.workbenches.get(&workbench_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_index).unwrap();
                let plane_id = wb.add_plane("Untitled-Plane", Plane::front());
                self.planes
                    .insert(self.cursor.clone(), (*workbench_index, plane_id));
            }
            Operation::SetPlaneName { plane_id, name } => {
                // the plane_id passed in is a SHA, we need to look up the actual plane_id
                let (workbench_idx, step_id) = self.planes.get(&plane_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_idx).unwrap();
                let step_idx = wb.step_id_from_unique_id(step_id).unwrap();
                wb.history.get_mut(step_idx as usize).unwrap().name = name.to_owned();
            }
            Operation::SetPlane { plane_id, plane } => {
                let (workbench_idx, step_id) = self.planes.get(&plane_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_idx).unwrap();
                let step_idx = wb.step_id_from_unique_id(step_id).unwrap();
                let step = wb.history.get_mut(step_idx as usize).unwrap();
                let new_plane = plane; // this is just to change the name to avoid a collision
                if let StepData::Plane { plane, .. } = &mut step.data {
                    *plane = new_plane.clone();
                } else {
                    unreachable!()
                };
            }
            Operation::CreateSketch {
                nonce,
                workbench_id,
            } => {
                let workbench_index = self.workbenches.get(&workbench_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_index).unwrap();
                let sketch_id = wb.add_blank_sketch("Untitled-Sketch");
                self.sketches
                    .insert(self.cursor.clone(), (*workbench_index, sketch_id));
            }
            Operation::SetSketchName { sketch_id, name } => {
                let (workbench_idx, step_id) = self.sketches.get(&sketch_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_idx).unwrap();
                let step_idx = wb.step_id_from_unique_id(step_id).unwrap();
                wb.history.get_mut(step_idx as usize).unwrap().name = name.to_owned();
            }
            Operation::SetSketchPlane {
                sketch_id,
                plane_id,
            } => {
                let (workbench_idx_sketch, sketch_id) = self.sketches.get(&sketch_id).unwrap();
                let (workbench_idx_plane, plane_id) = self.planes.get(&plane_id).unwrap();
                assert_eq!(workbench_idx_sketch, workbench_idx_plane);
                let mut wb = self
                    .project
                    .workbenches
                    .get_mut(*workbench_idx_plane)
                    .unwrap();
                let step_idx = wb.step_id_from_unique_id(sketch_id).unwrap();
                let step = wb.history.get_mut(step_idx as usize).unwrap();
                if let StepData::Sketch {
                    plane_description, ..
                } = &mut step.data
                {
                    *plane_description = PlaneDescription::PlaneId(plane_id.clone());
                } else {
                    unreachable!()
                };
            }
            Operation::AddSketchLine {
                sketch_id,
                start,
                end,
            } => {
                let (workbench_idx, sketch_id) = self.sketches.get(&sketch_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_idx).unwrap();
                let step_idx = wb.step_id_from_unique_id(sketch_id).unwrap();
                let step = wb.history.get_mut(step_idx as usize).unwrap();
                if let StepData::Sketch { sketch, .. } = &mut step.data {
                    sketch.add_line_segment(start.0, start.1, end.0, end.1);
                } else {
                    unreachable!()
                };
            }
            Operation::CreateExtrusion {
                workbench_id,
                nonce,
            } => {
                let workbench_idx = self.workbenches.get(&workbench_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_idx).unwrap();

                let extrusion = crate::extrusion::Extrusion {
                    sketch_id: "".to_owned(),
                    face_ids: vec![],
                    face_shas: vec![],
                    length: 25.0,
                    offset: 0.0,
                    direction: crate::extrusion::Direction::Normal,
                    mode: crate::extrusion::ExtrusionMode::New,
                };
                wb.add_extrusion("Untitled Extrusion", extrusion);
                let step_id = wb.history.len() - 1;
                self.extrusions
                    .insert(self.cursor.clone(), (*workbench_idx, step_id as usize));
            }
            Operation::SetExtrusionName { extrusion_id, name } => {
                let (workbench_idx, extrusion_idx) = self.extrusions.get(&extrusion_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_idx).unwrap();
                wb.history.get_mut(*extrusion_idx).unwrap().name = name.clone();
            }
            Operation::SetExtrusionDepth {
                extrusion_id,
                depth,
            } => {
                let (workbench_idx, extrusion_idx) = self.extrusions.get(&extrusion_id).unwrap();
                let mut wb = self.project.workbenches.get_mut(*workbench_idx).unwrap();
                if let StepData::Extrusion { extrusion, .. } = &mut wb.history[*extrusion_idx].data
                {
                    extrusion.length = depth;
                } else {
                    unreachable!()
                };
            }

            _ => {}
        }

        self.cursor.clone()
    }

    pub fn find_faces(&mut self, workbench_id: &Sha, sketch_id: &Sha) -> Sha {
        let (workbench_idx, sketch_id) = self.sketches.get(sketch_id).unwrap();
        // let workbench_sha = self.workbenches_inverse.get(workbench_idx).unwrap();
        let wb = self.project.workbenches.get(*workbench_idx).unwrap();

        let step_idx = wb.step_id_from_unique_id(sketch_id).unwrap();
        let step = wb.history.get(step_idx as usize).unwrap();

        let mut new_face_ops = Vec::new();
        if let StepData::Sketch { sketch, .. } = &step.data {
            let (faces, _unused_segments) = sketch.find_faces();
            for face in faces {
                let face_op = Operation::CreateFace {
                    workbench_id: workbench_id.clone(),
                    sketch_id: sketch_id.clone(),
                    face: face.clone(),
                };
                println!("Face Op: {:?}", face_op);
                new_face_ops.push(face_op);
            }
        } else {
            unreachable!()
        };

        for face_op in new_face_ops {
            self.append(face_op);
        }

        self.cursor.clone()
    }

    pub fn pretty_print(&self) {
        for commit in &self.oplog.commits {
            println!("{}", commit.pretty_print());
        }
    }

    pub fn to_tree(&self) -> CommitNode {
        // Build a tree of commits using CommitNode
        let mut commit_node_table: HashMap<String, CommitNode> = HashMap::new();
        for commit in &self.oplog.commits {
            commit_node_table.insert(
                commit.id.clone(),
                CommitNode {
                    commit: commit.id.clone(),
                    children: vec![],
                },
            );
        }
        for commit in &self.oplog.commits {
            let parent = commit.parent.clone();
            if parent == "" {
                // special treatment for the root node
                continue;
            }
            let mut parent_commit_node = commit_node_table.get_mut(&parent).unwrap();
            parent_commit_node.children.push(commit.id.clone());
        }

        let root_node = commit_node_table.get(&self.oplog.commits[0].id).unwrap();
        root_node.clone()
    }

    pub fn git_log(&self) {
        // Build a tree of commits using CommitNode
        let mut commit_node_table: HashMap<String, CommitNode> = HashMap::new();
        for commit in &self.oplog.commits {
            commit_node_table.insert(
                commit.id.clone(),
                CommitNode {
                    commit: commit.id.clone(),
                    children: vec![],
                },
            );
        }
        for commit in &self.oplog.commits {
            let parent = commit.parent.clone();
            if parent == "" {
                // special treatment for the root node
                continue;
            }
            let mut parent_commit_node = commit_node_table.get_mut(&parent).unwrap();
            parent_commit_node.children.push(commit.id.clone());
            // println!(
            //     "Parent now has: {} children",
            //     parent_commit_node.children.len()
            // )
        }

        let root_node = commit_node_table.get(&self.oplog.commits[0].id).unwrap();

        let commit_table = self
            .oplog
            .commits
            .iter()
            .map(|commit| (commit.id.clone(), commit))
            .collect::<HashMap<Sha, &Commit>>();

        // const OTHER_CHILD: &str = "│   "; // prefix: pipe
        // const OTHER_ENTRY: &str = "├── "; // connector: tee
        // const FINAL_CHILD: &str = "    "; // prefix: no more siblings
        // const FINAL_ENTRY: &str = "└── "; // connector: elbow

        println!("Root:");
        visit(&root_node.commit, "", &commit_table, &commit_node_table);

        fn visit(
            sha: &Sha,
            prefix: &str,
            commit_table: &HashMap<String, &Commit>,
            commit_node_table: &HashMap<String, CommitNode>,
        ) {
            let commit = commit_table.get(sha).unwrap();
            let commit_node = commit_node_table.get(sha).unwrap();
            println!("{}* {}", prefix, commit);

            if commit_node.children.len() == 0 {
                return;
            } else if commit_node.children.len() == 1 {
                visit(
                    &commit_node.children[0],
                    &prefix,
                    commit_table,
                    commit_node_table,
                );
            } else if commit_node.children.len() == 2 {
                println!("{}|\\", prefix);
                visit(
                    &commit_node.children[0],
                    &format!("| {}", prefix),
                    commit_table,
                    commit_node_table,
                );
                visit(
                    &commit_node.children[1],
                    &prefix,
                    commit_table,
                    commit_node_table,
                );
            }
        }
    }

    pub fn checkout(&mut self, sha: Sha) -> Result<String, String> {
        // check that the sha exists in the oplog before doing this
        for commit in &self.oplog.commits {
            if commit.id == sha {
                self.cursor = sha;
                return Ok(self.cursor.clone());
            }
        }
        Err(format!("SHA {} not found in oplog", sha))
    }

    pub fn cherry_pick(&mut self, sha: Sha) -> Result<String, String> {
        // check that the sha exists in the oplog before doing this
        for commit in &self.oplog.commits {
            if commit.id == sha {
                let new_operation = commit.operation.clone();
                let mut new_commit_id = self.append(new_operation.clone());

                // If the original commit created an entity, we'll need to create an alias commit
                if new_operation.is_create() {
                    new_commit_id = self.append(Operation::Alias {
                        original: sha,
                        new: new_commit_id.clone(),
                    });
                }

                return Ok(new_commit_id);
            }
        }
        Err(format!("SHA {} not found in oplog", sha))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub operation: Operation,
    pub content_hash: Sha,
    pub parent: Sha,
    pub id: Sha, // this is the SHA of "operation + parent"
}

impl Commit {
    pub fn init() -> Self {
        let init_op = Operation::Create {
            nonce: "Hello World".to_string(), // TODO: replace with actual seeded random string
        };
        let parent_sha = "".to_owned();
        Self {
            id: id_from_op_and_parent(&init_op, &parent_sha, 0),
            content_hash: init_op.hash(),
            operation: init_op,
            parent: parent_sha,
        }
    }

    pub fn pretty_print(&self) -> String {
        // truncate to just the first 10 chars of self.id
        format!("{}: {}", &self.id[..10], self.operation.pretty_print())
    }
}

impl std::fmt::Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", &self.id[..6], self.operation)
    }
}

#[derive(Debug, Clone)]
pub struct CommitNode {
    pub commit: Sha,
    pub children: Vec<Sha>,
}

impl std::fmt::Display for CommitNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.commit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Create {
        nonce: String,
    },
    Describe {
        description: String,
        commit: Sha,
    },
    Alias {
        original: Sha,
        new: Sha,
    },

    CreateProject {
        nonce: String,
    },
    SetProjectName {
        project_id: Sha,
        name: String,
    },

    CreateWorkbench {
        nonce: String,
    },
    SetWorkbenchName {
        workbench_id: Sha,
        name: String,
    },

    CreatePlane {
        nonce: String,
        workbench_id: Sha,
    },
    SetPlaneName {
        plane_id: Sha,
        name: String,
    },
    SetPlane {
        plane_id: Sha,
        plane: Plane,
    },

    CreateSketch {
        nonce: String,
        workbench_id: Sha,
    },
    SetSketchName {
        sketch_id: Sha,
        name: String,
    },
    SetSketchPlane {
        sketch_id: Sha,
        plane_id: Sha,
    },

    AddSketchRectangle {
        sketch_id: Sha,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    AddSketchCircle {
        sketch_id: Sha,
        x: f64,
        y: f64,
        radius: f64,
    },
    AddSketchLine {
        sketch_id: Sha,
        start: (f64, f64),
        end: (f64, f64),
    },
    AddSketchHandle {
        sketch_id: Sha,
        position: (f64, f64),
    },
    FinalizeSketch {
        workbench_id: Sha,
        sketch_id: Sha,
    },

    CreateFace {
        workbench_id: Sha,
        sketch_id: Sha,
        face: Face,
    },

    CreateExtrusion {
        workbench_id: Sha,
        nonce: String,
    },
    SetExtrusionName {
        extrusion_id: Sha,
        name: String,
    },
    SetExtrusionSketch {
        extrusion_id: Sha,
        sketch_id: Sha,
    },
    SetExtrusionHandles {
        extrusion_id: Sha,
        handles: Vec<Sha>,
    },
    SetExtrusionDepth {
        extrusion_id: Sha,
        depth: f64,
    },
}

impl Operation {
    pub fn is_create(&self) -> bool {
        match self {
            Operation::CreatePlane { .. } => true,
            Operation::CreateSketch { .. } => true,
            Operation::CreateExtrusion { .. } => true,
            _ => false,
        }
    }

    pub fn hash(&self) -> Sha {
        let mut hasher = Sha256::new();

        hasher.update("cadmium".as_bytes()); // mm, salt
        match self {
            Operation::Create { nonce } => hasher.update(format!("{nonce}").as_bytes()),
            Operation::Describe {
                description,
                commit,
            } => hasher.update(format!("{description}-{commit}").as_bytes()),
            Operation::Alias { original, new } => {
                hasher.update(format!("{original}-{new}").as_bytes())
            }
            Operation::CreateProject { nonce } => hasher.update(format!("{nonce}").as_bytes()),
            Operation::SetProjectName { project_id, name } => {
                hasher.update(format!("{project_id}-{name}").as_bytes())
            }
            Operation::CreateWorkbench { nonce } => hasher.update(format!("{nonce}").as_bytes()),
            Operation::SetWorkbenchName { workbench_id, name } => {
                hasher.update(format!("{workbench_id}-{name}").as_bytes())
            }

            Operation::CreatePlane {
                nonce,
                workbench_id,
            } => hasher.update(format!("{nonce}-{workbench_id}").as_bytes()),
            Operation::SetPlaneName { plane_id, name } => {
                hasher.update(format!("{plane_id}-{name}").as_bytes())
            }
            Operation::SetPlane { plane_id, plane } => {
                hasher.update(format!("{plane_id}-{plane:?}").as_bytes())
            }
            Operation::CreateSketch {
                nonce,
                workbench_id,
            } => hasher.update(format!("{nonce}-{workbench_id}").as_bytes()),
            Operation::SetSketchName { sketch_id, name } => {
                hasher.update(format!("{sketch_id}-{name}").as_bytes())
            }
            Operation::SetSketchPlane {
                sketch_id,
                plane_id,
            } => hasher.update(format!("{sketch_id}-{plane_id}").as_bytes()),
            Operation::AddSketchRectangle {
                sketch_id,
                x,
                y,
                width,
                height,
            } => hasher.update(format!("{sketch_id}-{x}-{y}-{width}-{height}").as_bytes()),
            Operation::AddSketchCircle {
                sketch_id,
                x,
                y,
                radius,
            } => hasher.update(format!("{sketch_id}-{x}-{y}-{radius}").as_bytes()),
            Operation::AddSketchLine {
                sketch_id,
                start,
                end,
            } => hasher.update(format!("{sketch_id}-{start:?}-{end:?}").as_bytes()),
            Operation::AddSketchHandle {
                sketch_id,
                position,
            } => hasher.update(format!("{sketch_id}-{position:?}").as_bytes()),
            Operation::FinalizeSketch {
                sketch_id,
                workbench_id,
            } => hasher.update(format!("{sketch_id}-{workbench_id}").as_bytes()),
            Operation::CreateFace {
                workbench_id,
                sketch_id,
                face,
            } => hasher.update(format!("{workbench_id}-{sketch_id}-{face:?}").as_bytes()),

            Operation::CreateExtrusion {
                nonce,
                workbench_id,
            } => hasher.update(format!("{nonce}-{workbench_id}").as_bytes()),
            Operation::SetExtrusionName { extrusion_id, name } => {
                hasher.update(format!("{extrusion_id}-{name}").as_bytes())
            }
            Operation::SetExtrusionSketch {
                extrusion_id,
                sketch_id,
            } => hasher.update(format!("{extrusion_id}-{sketch_id}").as_bytes()),
            Operation::SetExtrusionHandles {
                extrusion_id,
                handles,
            } => {
                hasher.update(format!("{extrusion_id}").as_bytes());
                for sha in handles {
                    hasher.update(format!("{sha}").as_bytes())
                }
            }
            Operation::SetExtrusionDepth {
                extrusion_id,
                depth,
            } => hasher.update(format!("{extrusion_id}-{depth}").as_bytes()),
        }

        format!("{:x}", hasher.finalize())
    }

    pub fn pretty_print(&self) -> String {
        let num_chars = 6;
        match self {
            Operation::Create { nonce } => format!("Create: {}", nonce),
            Operation::Describe {
                description,
                commit,
            } => format!(
                "Describe: {} '{}'",
                commit.to_owned()[..num_chars].to_string(),
                description
            ),
            Operation::Alias { original, new } => {
                format!(
                    "Alias: from {} to {}",
                    original.to_owned()[..num_chars].to_string(),
                    new.to_owned()[..num_chars].to_string()
                )
            }
            Operation::CreateProject { nonce } => format!("CreateProject: {}", nonce),
            Operation::SetProjectName { project_id, name } => {
                format!(
                    "SetProjectName: {} '{}'",
                    project_id.to_owned()[..num_chars].to_string(),
                    name
                )
            }
            Operation::CreateWorkbench { nonce } => {
                format!("CreateWorkspace: {}", nonce)
            }
            Operation::SetWorkbenchName { workbench_id, name } => {
                format!(
                    "SetWorkspaceName: {} '{}'",
                    workbench_id.to_owned()[..num_chars].to_string(),
                    name
                )
            }
            Operation::CreatePlane {
                nonce,
                workbench_id,
            } => format!(
                "CreatePlane: {} {}",
                workbench_id.to_owned()[..num_chars].to_string(),
                nonce
            ),
            Operation::SetPlaneName { plane_id, name } => {
                format!(
                    "SetPlaneName: {} '{}'",
                    plane_id.to_owned()[..num_chars].to_string(),
                    name
                )
            }
            Operation::SetPlane { plane_id, plane } => {
                format!(
                    "SetPlane: {}",
                    plane_id.to_owned()[..num_chars].to_string(),
                    // plane
                )
            }
            Operation::CreateSketch {
                nonce,
                workbench_id,
            } => format!(
                "CreateSketch: {} {}",
                workbench_id.to_owned()[..num_chars].to_string(),
                nonce
            ),
            Operation::SetSketchName { sketch_id, name } => {
                format!(
                    "SetSketchName: {} '{}'",
                    sketch_id.to_owned()[..num_chars].to_string(),
                    name
                )
            }
            Operation::SetSketchPlane {
                sketch_id,
                plane_id,
            } => {
                format!(
                    "SetSketchPlane: {} {}",
                    sketch_id.to_owned()[..num_chars].to_string(),
                    plane_id.to_owned()[..num_chars].to_string()
                )
            }
            Operation::AddSketchRectangle {
                sketch_id,
                x,
                y,
                width,
                height,
            } => format!(
                "AddSketchRectangle: {} ({}, {}) {}x{}",
                sketch_id.to_owned()[..num_chars].to_string(),
                x,
                y,
                width,
                height
            ),
            Operation::AddSketchCircle {
                sketch_id,
                x,
                y,
                radius,
            } => format!(
                "AddSketchCircle: {} ({}, {}) r={}",
                sketch_id.to_owned()[..num_chars].to_string(),
                x,
                y,
                radius
            ),
            Operation::AddSketchLine {
                sketch_id,
                start,
                end,
            } => format!(
                "AddSketchLine: {} ({}, {}) to ({}, {})",
                sketch_id.to_owned()[..num_chars].to_string(),
                start.0,
                start.1,
                end.0,
                end.1
            ),
            Operation::AddSketchHandle {
                sketch_id,
                position,
            } => format!(
                "AddSketchHandle: {} ({}, {})",
                sketch_id.to_owned()[..num_chars].to_string(),
                position.0,
                position.1
            ),
            Operation::FinalizeSketch {
                sketch_id,
                workbench_id,
            } => {
                format!(
                    "FinalizeSketch: {} {}",
                    workbench_id.to_owned()[..num_chars].to_string(),
                    sketch_id.to_owned()[..num_chars].to_string()
                )
            }
            Operation::CreateFace {
                workbench_id,
                sketch_id,
                face,
            } => {
                format!(
                    "CreateFace: {} {} {:?}",
                    workbench_id.to_owned()[..num_chars].to_string(),
                    sketch_id.to_owned()[..num_chars].to_string(),
                    face
                )
            }
            Operation::CreateExtrusion {
                nonce,
                workbench_id,
            } => format!(
                "CreateExtrusion: {} {}",
                workbench_id.to_owned()[..num_chars].to_string(),
                nonce
            ),
            Operation::SetExtrusionName { extrusion_id, name } => {
                format!(
                    "SetExtrusionName: {} '{}'",
                    extrusion_id.to_owned()[..num_chars].to_string(),
                    name
                )
            }
            Operation::SetExtrusionSketch {
                extrusion_id,
                sketch_id,
            } => {
                format!(
                    "SetExtrusionSketch: {} {}",
                    extrusion_id.to_owned()[..num_chars].to_string(),
                    sketch_id.to_owned()[..num_chars].to_string()
                )
            }
            Operation::SetExtrusionHandles {
                extrusion_id,
                handles,
            } => {
                let mut click_str = String::new();
                for sha in handles {
                    click_str.push_str(&format!("{} ", sha.to_owned()[..num_chars].to_string()));
                }
                format!(
                    "SetExtrusionClicks: {} {}",
                    extrusion_id.to_owned()[..num_chars].to_string(),
                    click_str
                )
            }
            Operation::SetExtrusionDepth {
                extrusion_id,
                depth,
            } => {
                format!(
                    "SetExtrusionDepth: {} {}",
                    extrusion_id.to_owned()[..num_chars].to_string(),
                    depth
                )
            }
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.pretty_print())
    }
}
