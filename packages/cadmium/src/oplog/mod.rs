use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::process::id;

use crate::project::Plane;

pub type Sha = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpLog {
    commits: Vec<Commit>,
}

impl OpLog {
    pub fn new() -> Self {
        Self { commits: vec![] }
    }

    pub fn init(&mut self) {
        let creation_commit = Commit::init();
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionLog {
    pub cursor: Sha,
    pub oplog: OpLog, // TODO: work out the lifetimes here so that we can have multiple evolutionLogs at once?
}

impl EvolutionLog {
    pub fn new() -> Self {
        let mut ol = OpLog::new();
        ol.init();
        Self {
            cursor: ol.last().unwrap().id.clone(),
            oplog: ol,
        }
    }

    pub fn append(&mut self, operation: Operation) -> Sha {
        self.cursor = self.oplog.append(&self.cursor, operation).id;
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

    CreatePlane {
        nonce: String,
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

    CreateExtrusion {
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
            Operation::CreatePlane { nonce } => hasher.update(format!("{nonce}").as_bytes()),
            Operation::SetPlaneName { plane_id, name } => {
                hasher.update(format!("{plane_id}-{name}").as_bytes())
            }
            Operation::SetPlane { plane_id, plane } => {
                hasher.update(format!("{plane_id}-{plane:?}").as_bytes())
            }
            Operation::CreateSketch { nonce } => hasher.update(format!("{nonce}").as_bytes()),
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
            Operation::CreateExtrusion { nonce } => hasher.update(format!("{nonce}").as_bytes()),
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
            Operation::CreatePlane { nonce } => format!("CreatePlane: {}", nonce),
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
            Operation::CreateSketch { nonce } => format!("CreateSketch: {}", nonce),
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
            Operation::CreateExtrusion { nonce } => format!("CreateExtrusion: {}", nonce),
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
