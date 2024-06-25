use log::{debug, info};
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::archetypes::{Plane, PlaneDescription};
use crate::feature::point::Point3;
use crate::feature::Feature;
use crate::isketch::ISketch;
use crate::step::evtree::EvTree;
use crate::step::{Step, StepHash, StepResult};
use crate::IDType;

use crate::message::*;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

/// A workbench is the main collection of set of objects that are being worked on.
///
/// CADmium is mostly designed around it and acts in objects that are descendants of the workbench.
#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[non_exhaustive]
pub struct Workbench {
	/// The workbench name - mainly used for display purposes.
	pub name: String,
	/// A list of steps that have been taken in the workbench - it's append only and fork-able
	pub history: Vec<Rc<RefCell<Step>>>,
	pub evtree: EvTree,

	/// Free-standing points in 3D space - not part of sketches
	#[serde(skip)]
	pub points: BTreeMap<IDType, Rc<RefCell<Point3>>>,
	/// The next ID to use for a point
	#[serde(skip)]
	pub points_next_id: IDType,

	/// Planes that can be used for sketches
	#[serde(skip)]
	pub planes: BTreeMap<IDType, Rc<RefCell<Plane>>>,
	/// The next ID to use for a plane
	#[serde(skip)]
	pub planes_next_id: IDType,

	/// Sketches that are part of the workbench
	#[serde(skip)]
	pub sketches: BTreeMap<IDType, Rc<RefCell<ISketch>>>,
	/// The next ID to use for a sketch
	#[serde(skip)]
	pub sketches_next_id: IDType,
	/// Features that are part of the workbench (e.g. [`Extrusion`])
	///
	/// [`Extrusion`]: crate::feature::extrusion::Extrusion
	#[serde(skip)]
	pub features: BTreeMap<IDType, Rc<RefCell<Feature>>>,
	/// The next ID to use for a feature
	#[serde(skip)]
	pub features_next_id: IDType,
}

impl Workbench {
	/// Create a new workbench with a given name
	pub(crate) fn new(name: &str) -> Self {
		info!("Creating new workbench: {}", name);
		Workbench {
			name: name.to_owned(),
			history: vec![],
			evtree: EvTree::default(),

			points: BTreeMap::new(),
			points_next_id: 0,
			planes: BTreeMap::new(),
			planes_next_id: 0,

			sketches: BTreeMap::new(),
			sketches_next_id: 0,
			features: BTreeMap::new(),
			features_next_id: 0,
		}
	}

	/// Records the given message as a [`Step`] in the workbench history and evolution tree
	///
	/// <div class="warning">
	///
	/// Does NOT call the message handler itself, only appends it to the history
	///
	/// </div>
	pub fn add_message_step(&mut self, message: &Message, node: StepResult) {
		let step = Step::new(message.clone(), node);
		self.evtree.push(step.hash());
		self.history.push(Rc::new(RefCell::new(step)));
	}

	/// Returns a [`Step`] by its [`StepHash`]
	///
	/// <div class="warning">
	///
	/// Does NOT check for hash collision (i.e. two steps with the same hash)
	///
	/// </div>
	pub fn get_step_by_hash(&self, hash: StepHash) -> Option<Rc<RefCell<Step>>> {
		debug!(
			"Looking for step with hash {} in hashes {:?}",
			hash,
			self.history
				.iter()
				.map(|step| step.borrow().hash())
				.collect::<Vec<_>>()
		);
		self.history
			.iter()
			.find(|step| step.borrow().hash() == hash)
			.cloned()
	}
}

impl Identifiable for Rc<RefCell<Workbench>> {
	type Parent = crate::project::Project;
	const ID_NAME: &'static str = "workbench_id";

	fn from_parent_id(parent: &crate::project::Project, hash: StepHash) -> anyhow::Result<Self> {
		// For now at least there's no good way to differentiate between a workbench
		// ID and a step hash. The workbench can't be hash-indexed as it's always
		// changing
		let id = hash.into_int();
		Ok(parent.get_workbench_by_id(id)?)
	}
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddPoint {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl MessageHandler for AddPoint {
	type Parent = Rc<RefCell<Workbench>>;
	fn handle_message(
		&self,
		sketch_ref: Self::Parent,
	) -> anyhow::Result<Option<(IDType, StepResult)>> {
		let mut wb = sketch_ref.borrow_mut();

		let new_id = wb.points_next_id;
		let point = Rc::new(RefCell::new(Point3::new(self.x, self.y, self.z)));
		wb.points.insert(new_id, point.clone());
		wb.points_next_id += 1;
		Ok(Some((new_id, StepResult::Point(point))))
	}
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddPlane {
	pub plane: Plane,
	pub width: f64,
	pub height: f64,
}

impl MessageHandler for AddPlane {
	type Parent = Rc<RefCell<Workbench>>;
	fn handle_message(
		&self,
		sketch_ref: Self::Parent,
	) -> anyhow::Result<Option<(IDType, StepResult)>> {
		let mut wb = sketch_ref.borrow_mut();

		let new_id = wb.planes_next_id;
		let plane = Rc::new(RefCell::new(self.plane.clone()));
		wb.planes.insert(new_id, plane.clone());
		wb.planes_next_id += 1;
		Ok(Some((new_id, StepResult::Plane(plane))))
	}
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct AddSketch {
	pub plane_description: PlaneDescription,
}

impl MessageHandler for AddSketch {
	type Parent = Rc<RefCell<Workbench>>;
	fn handle_message(
		&self,
		workbench_ref: Self::Parent,
	) -> anyhow::Result<Option<(IDType, StepResult)>> {
		let mut wb = workbench_ref.borrow_mut();
		let sketch = ISketch::try_from_plane_description(&wb, &self.plane_description)?;

		let new_id = wb.sketches_next_id;
		let sketch_cell = Rc::new(RefCell::new(sketch));
		wb.sketches.insert(new_id, sketch_cell.clone());
		wb.sketches_next_id += 1;
		Ok(Some((new_id, StepResult::Sketch(sketch_cell.clone()))))
	}
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WorkbenchRename {
	pub new_name: String,
}

impl MessageHandler for WorkbenchRename {
	type Parent = Rc<RefCell<Workbench>>;
	fn handle_message(
		&self,
		workbench_ref: Self::Parent,
	) -> anyhow::Result<Option<(IDType, StepResult)>> {
		let mut workbench = workbench_ref.borrow_mut();
		workbench.name = self.new_name.clone();
		Ok(None)
	}
}

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(from_wasm_abi, into_wasm_abi)]
pub struct SetSketchPlane {
	pub sketch_id: IDType,
	pub plane_description: PlaneDescription,
}

impl MessageHandler for SetSketchPlane {
	type Parent = Rc<RefCell<Workbench>>;
	fn handle_message(
		&self,
		workbench_ref: Self::Parent,
	) -> anyhow::Result<Option<(IDType, StepResult)>> {
		let wb = workbench_ref.borrow();

		let plane = match self.plane_description {
			PlaneDescription::PlaneId(plane_hash) => {
				let plane_id = crate::ID_MAP
					.with_borrow(|m| m.get(&plane_hash).cloned())
					.ok_or(anyhow::anyhow!(
						"Failed to find plane with hash {}",
						plane_hash
					))?;
				wb.planes
					.get(&plane_id)
					.ok_or(anyhow::anyhow!("Failed to find plane with id {}", plane_id))?
			}
			PlaneDescription::SolidFace {
				solid_id: _,
				normal: _,
			} => todo!("Implement SolidFace"),
		}
		.clone();

		let sketch = wb.sketches.get(&self.sketch_id).ok_or(anyhow::anyhow!(
			"Failed to find sketch with id {}",
			self.sketch_id
		))?;
		sketch.borrow_mut().plane = plane;

		Ok(None)
	}
}
