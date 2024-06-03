use std::fmt::Display;

use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::message::Message;
use crate::IDType;

#[derive(Tsify, Clone, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Step {
    pub(crate) id: IDType,
    pub(crate) name: String,
    pub(crate) suppressed: bool,
    pub(crate) data: Message,
}

impl Step {
    pub fn new(id: IDType, data: Message) -> Self {
        Step {
            id,
            name: "TODO".to_string(),
            suppressed: false,
            data,
        }
    }

    pub fn unique_id(&self) -> String {
        format!("{}", self)
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.name, self.data, self.id)
    }
}
