use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use xxhash_rust::xxh3::xxh3_64;

use crate::message::{Identifiable, Message};
use crate::workbench::Workbench;

pub mod actions;
pub mod evtree;
mod hash;
mod result;
pub mod sketch_action;

pub use hash::StepHash;
pub use result::StepResult;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct History(pub Vec<Rc<RefCell<Step>>>);

/// A step describes a single operation that takes place in a [`Workbench`].
///
/// An operation is often a transformation of the part in the workbench,
/// but it can also be a transformation of the meta-data of the workbench itself.
///
/// Each step is comprised by a [`Message`] that describes the operation,
/// the [`StepHash`] of it and the [`StepResult`] of the operation.
///
/// It is safe to assume that the step `data` field will never change.
#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
	hash: StepHash,
	pub name: String,
	suppressed: bool,
	data: Message,
	result: StepResult,
	timestamp: SystemTime,
	author: String,
}

impl Step {
	pub fn new(data: Message, result: StepResult) -> Self {
		let timestamp = SystemTime::now();
		let author = "Anonymous".to_string();

		let message_data = serde_json::to_string(&data).unwrap();
		let timestamp_str = serde_json::to_string(&timestamp).unwrap();
		let author = serde_json::to_string(&author).unwrap();
		let hash_data = [
			message_data.as_bytes(),
			timestamp_str.as_bytes(),
			author.as_bytes(),
		]
		.concat();
		let hash = StepHash::from_int(xxh3_64(&hash_data));

		Self {
			hash,
			name: format!("{}-{}", data, hash),
			suppressed: false,
			data,
			result,
			timestamp,
			author,
		}
	}

	pub fn hash(&self) -> StepHash {
		self.hash
	}

	pub fn result(&self) -> &StepResult {
		&self.result
	}

	pub fn suppress(&mut self) {
		self.suppressed = true;
	}

	pub fn unsuppress(&mut self) {
		self.suppressed = false;
	}

	pub fn suppressed(&self) -> bool {
		self.suppressed
	}

	pub fn timestamp(&self) -> SystemTime {
		self.timestamp
	}
}

impl Display for Step {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}-{}", self.data, self.hash)
	}
}

impl Identifiable for Rc<RefCell<Step>> {
	type Parent = Rc<RefCell<Workbench>>;
	const ID_NAME: &'static str = "step_id";

	fn from_parent_id(parent: &Self::Parent, hash: StepHash) -> anyhow::Result<Self> {
		Ok(parent
			.borrow()
			.get_step_by_hash(hash)
			.ok_or(anyhow::anyhow!(
				"No step with hash {} exists in the current workbench",
				hash
			))?
			.clone())
	}
}
