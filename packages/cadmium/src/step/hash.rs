use std::fmt::Display;

use crate::message::Message;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use xxhash_rust::xxh3::xxh3_64;

/// This represents a hash of a step.
/// It's really just a wrapper around a u64, but it's used to ensure that the hash
/// is always calculated in the same way and doesn't get mixed
/// with internal indexes or ids.
/// The only way to construct a new one is to calculate a hash of a message.
///
/// We also implement Copy as this is just a u64.
#[derive(Tsify, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[repr(transparent)]
pub struct StepHash(#[tsify(type = "string")] u64);

impl StepHash {
    pub fn into_int(&self) -> u64 {
        self.0
    }

    pub const fn from_int(val: u64) -> Self {
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
        write!(f, "{}", self.0)
    }
}

// Serialize as string
impl Serialize for StepHash {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}

// Deserialize from string
impl<'de> Deserialize<'de> for StepHash {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self(s.parse().unwrap()))
    }
}
