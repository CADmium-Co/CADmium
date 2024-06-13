use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use xxhash_rust::xxh3::xxh3_64;

use crate::message::Message;

/// This represents a hash of a step.
/// It's really just a wrapper around a u64, but it's used to ensure that the hash
/// is always calculated in the same way and doesn't get mixed
/// with internal indexes or ids.
/// The only way to construct a new one is to calculate a hash of a message.
///
/// We also implement Copy as this is just a u64.
#[derive(Tsify, Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(transparent)]
#[repr(transparent)]
pub struct StepHash(#[tsify(type = "bigint")] u64);

impl StepHash {
    pub fn give_the_int_im_not_stupid(&self) -> u64 {
        self.0
    }

    pub const fn take_the_int_im_not_stupid(val: u64) -> Self {
        Self(val)
    }
}

impl From<&Message> for StepHash {
    fn from(msg: &Message) -> Self {
        // Maybe encode to binary instead of json?
        let hash = xxh3_64(serde_json::to_string(msg).unwrap().as_bytes());
        Self(hash)
    }
}

impl Display for StepHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
