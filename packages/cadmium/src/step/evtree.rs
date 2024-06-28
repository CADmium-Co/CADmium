use loro::LoroDoc;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use super::StepHash;

// TODO: Use a single global document and use the project + workbench ID as the key
pub const EVTREE_ID: &str = "evtree";
pub const EVTREE_META_ID: &str = "evtree_meta";
pub const EVTREE_CURRENT_PEER: &str = "current_peer";
pub const EVTREE_CURRENT_COUNTER: &str = "current_counter";
pub const EVTREE_HASH_META: &str = "hash";
// pub const EVTREE_PREV_ID: &str = "prev";
// pub const EVTREE_THIS_ID: &str = "this";

/// A thin wrapper around a `LoroDoc` that represents the evolution tree.
///
/// This is a list of the currently active steps in the project, as well as the
/// all the other possible versions - much like git.
///
/// It doesn't hold the actual steps, but rather the hashes of the steps.
#[derive(Debug, Default, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct EvTree {
	#[serde(with = "loro_serde")]
	doc: LoroDoc,
}

impl EvTree {
	/// Traverses the evolution tree from the current node until the root node
	/// and returns the referenced hashes.
	pub fn to_step_hashes(&self) -> anyhow::Result<Vec<StepHash>> {
		let mut step_hashes = Vec::<StepHash>::new();

		self.doc.get_list(EVTREE_ID).for_each(|(_, value_handler)| {
			let value = value_handler.as_value().unwrap();
			let hash = StepHash::try_from(value).unwrap();
			step_hashes.push(hash);
		});

		Ok(step_hashes)
	}

	/// Add a new step hash to the evolution tree.
	pub fn push(&mut self, hash: StepHash) {
		let list = self.doc.get_list(EVTREE_ID);
		list.push(hash).unwrap();
		self.doc.commit();
	}

	pub fn export(&self) -> Vec<u8> {
		self.doc.export_snapshot()
	}

	pub fn import(&self, bytes: &[u8]) -> anyhow::Result<()> {
		Ok(self.doc.import(bytes)?)
	}
}

pub mod loro_serde {
	//! Serialization and deserialization implementation for `LoroDoc`.
	//!
	//! To be used with `#[serde(with = "loro_serde")]` attribute.

	use loro::LoroDoc;
	use serde::Deserialize;

	pub fn serialize<S: serde::Serializer>(
		value: &LoroDoc,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		serializer.serialize_bytes(&value.export_snapshot())
	}

	pub fn deserialize<'de, D: serde::Deserializer<'de>>(
		deserializer: D,
	) -> Result<LoroDoc, D::Error> {
		let bytes = Vec::<u8>::deserialize(deserializer)?;
		let doc = LoroDoc::new();

		doc.import(&bytes).map_err(serde::de::Error::custom)?;

		Ok(doc)
	}
}
