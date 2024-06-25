use loro::{LoroDoc, LoroValue, TreeID};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::error::CADmiumError;

use super::StepHash;

// TODO: Use a single global document and use the project + workbench ID as the key
pub const EVTREE_ID: &str = "evtree";
pub const EVTREE_META_ID: &str = "evtree_meta";
pub const EVTREE_CURRENT_PEER: &str = "current_peer";
pub const EVTREE_CURRENT_COUNTER: &str = "current_counter";
pub const EVTREE_HASH_META: &str = "hash";
// pub const EVTREE_PREV_ID: &str = "prev";
// pub const EVTREE_THIS_ID: &str = "this";

#[derive(Debug, Serialize, Deserialize)]
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
		let tree = self.doc.get_tree(EVTREE_ID);
		let mut this = self.get_current_treeid();

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

	pub fn get_current_treeid(&self) -> TreeID {
		let meta_map = self.doc.get_map(EVTREE_META_ID);
		let LoroValue::I64(peer_i64) = meta_map.get(EVTREE_CURRENT_PEER).unwrap().left().unwrap()
		else {
			panic!("Peer ID is not an i64");
		};
		let peer = u64::from_ne_bytes(peer_i64.to_ne_bytes());
		let LoroValue::I64(counter) = meta_map
			.get(EVTREE_CURRENT_COUNTER)
			.unwrap()
			.left()
			.unwrap()
		else {
			panic!("Counter is not an i64");
		};
		TreeID::new(peer, counter as i32)
	}

	fn set_current_treeid(&self, treeid: TreeID) {
		let meta_map = self.doc.get_map(EVTREE_META_ID);
		meta_map
			.insert(
				EVTREE_CURRENT_PEER,
				i64::from_ne_bytes(treeid.peer.to_ne_bytes()),
			)
			.unwrap();
		meta_map
			.insert(EVTREE_CURRENT_COUNTER, treeid.counter)
			.unwrap();
	}

	pub fn append(&mut self, hash: StepHash) {
		let tree = self.doc.get_tree(EVTREE_ID);
		let new_this = tree.create(Some(self.get_current_treeid())).unwrap();
		let meta = tree.get_meta(new_this).unwrap();
		meta.insert(EVTREE_HASH_META, hash).unwrap();
		self.set_current_treeid(new_this);
	}
}

impl Default for EvTree {
	fn default() -> Self {
		let doc = LoroDoc::new();
		let tree = doc.get_tree(EVTREE_ID);
		let root = tree.create(None).unwrap();

		let evtree = Self { doc };
		evtree.set_current_treeid(root);

		evtree
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
