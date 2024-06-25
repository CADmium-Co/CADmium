//! CADmium is a library for building parametric CAD applications in Rust.
//!
//! Its main target is the web, as a WASM module, but it can also be used in native applications.
//!
//! The library is mostly an interface and interoperability layer for the [`ISOtope`](https://github.com/CADmium-Co/ISOtope)
//! 2D constraint solver and the [`Truck`](https://github.com/ricosjp/truck) CAD kernel.
//!
//! It is designed to be used in a functional way, where the state
//! of the application is stored in a single data structure in the global memory.
//!
//! ## Architecture
//!
//! The high-level architecture is as follows (tree-like, breadth-first):
//!
//! - A [`Project`] is the main data structure that holds every kind of
//!   information about the project - mainly an array of [`Workbench`]es.
//! - A [`Workbench`] holds both the history and the internal state of the
//!   result of the operations in the project.
//!   The following arrays of structs are held:
//! - A [`Step`] is a single operation that takes place in a [`Workbench`].
//!   Comprised of a [`Message`], its [`StepHash`] and its [`StepResult`],
//!   an array of steps is also known as the history of the workbench.
//! - A [`Point3`] represents a point in 3D space. In the context of direct
//!   [`Workbench`] descendant, it's a free-standing point, not part of a sketch
//!   or solid.
//! - A [`Plane`] is a 2D plane that can be used to create sketches.
//! - An [`ISketch`] is a 2D sketch that can be used to create 3D models.
//!   It holds an ISOtope `Sketch` and a list of [`Compound`]s (a way to
//!   describe complex 2D shapes using a set of ISOtope primitives and constraints).
//! - A [`Feature`] is a 3D operation that mostly results in a 3D object -
//!   either by creating or modifying it. For example, an [`Extrusion`] is a feature.
//!
//! ## Usage
//! The way to interact with CADmium is through messages. A message is a single,
//! pre-defined operation that can be applied to a project. For example, a message
//! could be `ProjectRename` or `FeatureExtrusionAdd`, both variants of the
//! [`Message`] enum.
//!
// TODO: Give a better example (e.g. a simple sketch and extrusion)
//! ```rust
//! use cadmium::{create_project, get_project, send_message};
//! use cadmium::message::Message;
//! use cadmium::project::ProjectRename;
//!
//! let project_id = create_project("My Project");
//! let mut project = get_project(project_id).unwrap();
//! let message = Message::ProjectRename(ProjectRename { new_name: "New Name".to_string() });
//! let result = send_message(project_id, &message);
//! assert!(result.success);
//!
//! let project = get_project(project_id).unwrap();
//! assert_eq!(project.name, "New Name");
//! ```
//!
//! ## WASM Usage
//!
//! CADmium is designed to be used in the browser as a WebAssembly module.
//! It can be compiled with `wasm-pack` and automatically produces a TypeScript
//! definition file that can be used in a web application.
// TODO: Add a javascript example
//!
//! [`Compound`]: crate::isketch::compound::Compound
//! [`Extrusion`]: crate::feature::extrusion::Extrusion
//! [`Feature`]: crate::feature::Feature
//! [`ISketch`]: crate::isketch::ISketch
//! [`Point3`]: crate::feature::point::Point3
//! [`Plane`]: crate::archetypes::Plane
//! [`Step`]: crate::step::Step
//! [`StepHash`]: crate::step::StepHash
//! [`StepResult`]: crate::step::StepResult
//! [`Workbench`]: crate::workbench::Workbench

use std::cell::RefCell;
use std::collections::BTreeMap;

use error::CADmiumError;
use message::{Message, MessageResult};
use step::{History, StepHash};
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

/// Creates a new [`Project`](project::Project) and returns the index of the project in the global project list
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

/// Returns a concrete [`Project`](project::Project) from the global project list.
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

/// Sends a message to a [`Project`](project::Project) and returns the result
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

/// Returns the history of a [`Workbench`](workbench::Workbench) as a [`History`] object
#[wasm_bindgen]
pub fn get_workbench_oplog(
	project_index: usize,
	workbench_index: usize,
) -> Result<History, String> {
	PROJECTS.with(|projects_ref| {
		let projects = projects_ref.borrow();
		let p = projects
			.get(project_index)
			.ok_or(CADmiumError::ProjectIDNotFound(project_index).to_string())?;
		let wb_cell = p
			.get_workbench_by_id(workbench_index as u64)
			.map_err(|e| e.to_string())?;
		let wb = wb_cell.borrow();

		Ok(History(wb.history.clone()))
	})
}

/// Returns the event tree of a [`Workbench`](workbench::Workbench) as a serialized [`LoroDoc`](loro::LoroDoc) object
// TODO: Add ability to retrieve partial event trees
#[wasm_bindgen]
pub fn get_workbench_evtree(
	project_index: usize,
	workbench_index: usize,
) -> Result<Vec<u8>, String> {
	PROJECTS.with(|projects_ref| {
		let projects = projects_ref.borrow();
		let p = projects
			.get(project_index)
			.ok_or(CADmiumError::ProjectIDNotFound(project_index).to_string())?;
		let wb_cell = p
			.get_workbench_by_id(workbench_index as u64)
			.map_err(|e| e.to_string())?;
		let wb = wb_cell.borrow();

		Ok(wb.evtree.export())
	})
}

// TODO: Add ability to retrieve partial event trees
#[wasm_bindgen]
pub fn set_workbench_evtree(
	project_index: usize,
	workbench_index: usize,
	evtree: Vec<u8>,
) -> Result<(), String> {
	PROJECTS.with(|projects_ref| {
		let projects = projects_ref.borrow();
		let p = projects
			.get(project_index)
			.ok_or(CADmiumError::ProjectIDNotFound(project_index).to_string())?;
		let wb_cell = p
			.get_workbench_by_id(workbench_index as u64)
			.map_err(|e| e.to_string())?;
		let wb = wb_cell.borrow();
		wb.evtree.import(&evtree).map_err(|e| e.to_string())?;

		Ok(())
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

	// #[wasm_bindgen]
	// pub fn get_workbench(&self, workbench_index: u32) -> workbench::Workbench {
	// 	// TODO: Use get() and return a Result
	// 	self.native
	// 		.workbenches
	// 		.get(workbench_index as usize)
	// 		.unwrap()
	// 		.borrow()
	// 		.clone() // This single call pollutes Clone derives for all MessageHandlers
	// }

	#[wasm_bindgen]
	pub fn send_message(&mut self, message: &Message) -> MessageResult {
		message.handle(&mut self.native).into()
	}
}
