use loro::{LoroDoc, TreeID};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::error::CADmiumError;

use super::StepHash;

// TODO: Use a single global document and use the project + workbench ID as the key
pub const EVTREE_ID: &str = "evtree";
pub const EVTREE_HASH_META: &str = "hash";
// pub const EVTREE_PREV_ID: &str = "prev";
// pub const EVTREE_THIS_ID: &str = "this";

#[derive(Debug, Default, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct EvTree {
	#[serde(with = "loro_serde")]
	doc: LoroDoc,
}

impl EvTree {
	/// Traverses the evolution tree from the current node until the root node
	/// and returns the referenced hashes.
	pub fn to_step_hashes(&self, peer: u64, counter: i32) -> anyhow::Result<Vec<StepHash>> {
		let mut step_hashes = Vec::<StepHash>::new();
		let tree = self.doc.get_tree(EVTREE_ID);
		let mut this = TreeID::new(peer, counter);

		loop {
			let node_meta = tree.get_meta(this)?;
			let value = node_meta
				.get(EVTREE_HASH_META)
				.ok_or(CADmiumError::EvTreeHashNotFound(this.counter))?
				.left()
				.ok_or(CADmiumError::EvTreeHashIsContainer(this.counter))?;
			let hash: StepHash = (&value).try_into()?;
			step_hashes.push(hash);

			match tree.parent(&this) {
				Some(Some(parent)) => this = parent,
				_ => break,
			}
		}

		// The traversal was done from the leaf to the root, so we need to reverse the list
		step_hashes.reverse();
		Ok(step_hashes)
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
