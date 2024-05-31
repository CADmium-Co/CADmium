use cadmium_macros::MessageEnum;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::IDType;

pub mod prelude {
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};

    use crate::IDType;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct IDWrap<H>(pub IDType, pub H);

    pub trait MessageHandler<P: Serialize + DeserializeOwned + FromParentID> {
        fn handle_message(&self, item: &mut P) -> anyhow::Result<Option<IDType>>;
    }
    pub trait ProjectMessageHandler {
        fn handle_project_message(&self, project: &mut crate::project::Project) -> anyhow::Result<Option<IDType>>;
    }

    pub trait IntoChildID<C: Serialize + DeserializeOwned> {
        fn into_child(&mut self, id: IDType) -> anyhow::Result<&mut C>;
    }

    pub trait FromParentID: Serialize + DeserializeOwned {
        type Child;
        fn from_parent(parent: &mut Self::Child, id: IDType) -> anyhow::Result<&mut Self>;
    }
}

use prelude::*;
#[derive(MessageEnum, Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Message {
    WorkbenchRename(IDWrap<crate::workbench::WorkbenchRename>),
    WorkbenchPointUpdate(IDWrap<IDWrap<crate::solid::point::WorkbenchPointUpdate>>),
}

#[derive(Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MessageResult {
    pub success: bool,
    pub data: String,
}

impl From<anyhow::Result<Option<IDType>>> for MessageResult {
    fn from(result: anyhow::Result<Option<IDType>>) -> Self {
        match result {
            // TODO: The Success should be a stable enum
            Ok(msg) => Self {
                success: true,
                data: if let Some(id) = msg { id.to_string() } else { "null".to_string() }
            },
            Err(e) => Self {
                success: false,
                data: e.backtrace().to_string()
            },
        }
    }
}
