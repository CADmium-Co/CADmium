use highhash::Murmur3Hasher;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::process::id;

use crate::project::Plane;

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
            id: id_from_op_and_parent(&operation, &parent),
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

fn id_from_op_and_parent(operation: &Operation, parent: &Sha) -> Sha {
    let h = operation.hash();
    let mut hasher = Murmur3Hasher::default();
    hasher.write(format!("{h}-{parent}").as_bytes());
    format!("{:x}", hasher.finish())
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

    pub fn checkout(&mut self, sha: Sha) -> Result<(), String> {
        // check that the sha exists in the oplog before doing this
        for commit in &self.oplog.commits {
            if commit.id == sha {
                self.cursor = sha;
                return Ok(());
            }
        }
        Err(format!("SHA {} not found in oplog", sha))
    }

    pub fn cherry_pick(&mut self, sha: Sha) -> Result<(), String> {
        // check that the sha exists in the oplog before doing this
        for commit in &self.oplog.commits {
            if commit.id == sha {
                self.append(commit.operation.clone());
                return Ok(());
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
            id: id_from_op_and_parent(&init_op, &parent_sha),
            content_hash: init_op.hash(),
            operation: init_op,
            parent: parent_sha,
        }
    }

    pub fn pretty_print(&self) -> String {
        format!("{}: {}", self.id, self.operation.pretty_print())
    }
}

pub type Sha = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Create {
        nonce: String,
    },
    Describe {
        description: String,
        commit: Sha,
    },
    NewPlane {
        name: String,
        plane: Plane,
    },
    NewSketch {
        name: String,
        plane_name: String,
    },
    NewRectangle {
        sketch_name: String,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    NewExtrusion {
        name: String,
        sketch_name: String,
        click_x: f64,
        click_y: f64,
        depth: f64,
    },
    NewCircle {
        sketch_name: String,
        x: f64,
        y: f64,
        radius: f64,
    },
}

impl Operation {
    pub fn hash(&self) -> Sha {
        let mut hasher = Murmur3Hasher::default();
        hasher.write("cadmium".as_bytes()); // mm, salt
        match self {
            Operation::Create { nonce } => hasher.write(format!("{nonce}").as_bytes()),
            Operation::Describe {
                description,
                commit,
            } => hasher.write(format!("{description}-{commit}").as_bytes()),
            Operation::NewPlane { name, plane } => {
                hasher.write(format!("{name}-{plane:?}").as_bytes())
            }
            Operation::NewSketch { name, plane_name } => {
                hasher.write(format!("{name}-{plane_name:?}").as_bytes())
            }
            Operation::NewRectangle {
                sketch_name,
                x,
                y,
                width,
                height,
            } => hasher.write(format!("{sketch_name}-{x}-{y}-{width}-{height}").as_bytes()),
            Operation::NewExtrusion {
                name,
                sketch_name,
                click_x,
                click_y,
                depth,
            } => {
                hasher.write(format!("{name}-{sketch_name}-{click_x}-{click_y}-{depth}").as_bytes())
            }
            Operation::NewCircle {
                sketch_name,
                x,
                y,
                radius,
            } => hasher.write(format!("{sketch_name}-{x}-{y}-{radius}").as_bytes()),
        }

        format!("{:x}", hasher.finish())
    }

    pub fn pretty_print(&self) -> String {
        match self {
            Operation::Create { nonce } => format!("Create: {}", nonce),
            Operation::Describe {
                description,
                commit,
            } => format!("Describe: {} '{}'", commit, description),
            Operation::NewPlane { name, plane } => format!("NewPlane: '{}'", name),
            Operation::NewSketch { name, plane_name } => {
                format!("NewSketch: '{}' on plane '{}'", name, plane_name)
            }
            Operation::NewRectangle {
                sketch_name,
                x,
                y,
                width,
                height,
            } => format!(
                "NewRectangle: {} {} {} {} on '{}'",
                x, y, width, height, sketch_name
            ),
            Operation::NewExtrusion {
                name,
                sketch_name,
                click_x,
                click_y,
                depth,
            } => format!(
                "NewExtrusion: '{}' on '{}' ({},{}) depth: {}",
                name, sketch_name, click_x, click_y, depth
            ),
            Operation::NewCircle {
                sketch_name,
                x,
                y,
                radius,
            } => format!(
                "NewCircle: ({},{}) radius: {} on '{}'",
                x, y, radius, sketch_name
            ),
        }
    }
}
