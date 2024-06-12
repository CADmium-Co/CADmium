use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use crate::IDType;

use super::node::NodeLike;
use super::Node;

#[derive(Tsify, Debug, Clone, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Graph {
    nodes: Vec<Node>,
    next_id: IDType,
}

impl NodeLike for Graph {
    fn add_link(&mut self, node: Node) {
        self.nodes.push(node);
    }

    fn links(&self) -> Vec<Node> {
        self.nodes.clone()
    }
}
