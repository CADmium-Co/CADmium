use std::cell::RefCell;
use std::collections::BTreeMap;

use error::CADmiumError;
use message::{Message, MessageResult};
use step::StepHash;
use tsify_next::declare;
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

pub mod archetypes;
pub mod error;
pub mod feature;
pub mod isketch;
pub mod message;
pub mod project;
pub mod step;
pub mod workbench;

/// The primary type used to describe an internal ID in CADmium
///
/// Could be an index to a vector, a key in a map, etc.
#[declare]
pub type IDType = u64;

thread_local! {
    /// This is a global map to keep track of hashes against local IDs
    /// The hash is the unique identifier for a step
    /// The ID could be any kind of ID, e.g. a isotope sketch primitive
    ///
    /// <div class="warning">
    ///
    /// Using this map in reverse (from local ID to hash) requires manual check logic
    /// (that the resulting hash is a step of the correct type, points to the correct parent, etc.)
    ///
    /// </div>
    static ID_MAP: RefCell<BTreeMap<StepHash, IDType>> = RefCell::new(BTreeMap::new());

    /// Global project list - this is the preferred way to store & access projects
    static PROJECTS: RefCell<Vec<project::Project>> = RefCell::new(Vec::new());
}

/// Creates a new [`project::Project`] and returns the index of the project in the global project list
///
/// # Examples
///
/// ```rust
/// use cadmium::{create_project, get_project};
///
/// let project_id = create_project("My Project");
/// let project = get_project(project_id).unwrap();
///
/// assert_eq!(project.name, "My Project");
/// ```
#[wasm_bindgen]
pub fn create_project(name: &str) -> usize {
    let p = project::Project::new(name);
    PROJECTS.with(|projects_ref| {
        let mut projects = projects_ref.borrow_mut();
        projects.push(p);
        projects.len() - 1
    })
}

/// Returns a concrete [`project::Project`] from the global project list.
///
/// A new project can be created with [`create_project`] function.
#[wasm_bindgen]
pub fn get_project(project_index: usize) -> Result<project::Project, String> {
    PROJECTS.with(|projects_ref| {
        let projects = projects_ref.borrow();
        Ok(projects
            .get(project_index)
            .ok_or(CADmiumError::ProjectIDNotFound(project_index).to_string())?
            .clone())
    })
}

/// Sends a message to a [`project::Project`] and returns the result
///
/// [`Message`]s are the primary way to interact with CADmium.
/// They describe any kind of action that can be taken on a project.
///
/// # Examples
///
/// ```rust
/// use cadmium::{create_project, get_project, send_message};
/// use cadmium::message::Message;
/// use cadmium::project::ProjectRename;
///
/// let project_id = create_project("My Project");
/// let message = Message::ProjectRename(ProjectRename { new_name: "New Name".to_string() });
/// let result = send_message(project_id, &message);
/// assert!(result.success);
///
/// let project = get_project(project_id).unwrap();
/// assert_eq!(project.name, "New Name");
/// ```
#[wasm_bindgen]
pub fn send_message(project_index: usize, message: &Message) -> MessageResult {
    PROJECTS.with(|projects_ref| {
        let mut projects = projects_ref.borrow_mut();
        let Some(mut p) = projects.get_mut(project_index as usize) else {
            return CADmiumError::ProjectIDNotFound(project_index).into();
        };

        message.handle(&mut p).into()
    })
}

/// Returns a concrete [`workbench::Workbench`] from a [`project::Project`].
#[wasm_bindgen]
pub fn get_workbench(
    project_index: usize,
    workbench_index: usize,
) -> Result<workbench::Workbench, String> {
    PROJECTS.with(|projects_ref| {
        let projects = projects_ref.borrow();
        let p = projects
            .get(project_index)
            .ok_or(CADmiumError::ProjectIDNotFound(project_index).to_string())?;
        let wb = p
            .workbenches
            .get(workbench_index)
            .ok_or(CADmiumError::WorkbenchIDNotFound(workbench_index as u64).to_string())?
            .borrow();
        Ok(wb.clone())
    })
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Project {
    native: project::Project,
}

#[wasm_bindgen]
impl Project {
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> Project {
        console_error_panic_hook::set_once();
        wasm_logger::init(wasm_logger::Config::default());

        Project {
            native: project::Project::new(name),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.native.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.native.name = name;
    }

    #[wasm_bindgen(getter)]
    pub fn json(&self) -> String {
        self.native.json()
    }

    #[wasm_bindgen]
    pub fn to_json(&self) -> String {
        self.native.json()
    }

    #[wasm_bindgen]
    pub fn from_json(json: String) -> Project {
        let p = project::Project::from_json(&json);
        Project { native: p }
    }

    #[wasm_bindgen]
    pub fn compute_constraint_errors(&mut self) {
        // self.native.compute_constraint_errors();
    }

    #[wasm_bindgen]
    pub fn get_workbench(&self, workbench_index: u32) -> workbench::Workbench {
        // TODO: Use get() and return a Result
        self.native
            .workbenches
            .get(workbench_index as usize)
            .unwrap()
            .borrow()
            .clone() // This single call pollutes Clone derives for all MessageHandlers
    }

    #[wasm_bindgen]
    pub fn send_message(&mut self, message: &Message) -> MessageResult {
        message.handle(&mut self.native).into()
    }
}
