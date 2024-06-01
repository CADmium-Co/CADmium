use cadmium_macros::MessageEnum;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

pub mod prelude {
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};

    pub use crate::IDType;

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
        type Parent;
        fn from_parent(parent: &mut Self::Parent, id: IDType) -> anyhow::Result<&mut Self>;
    }

    // impl<T, C> FromParentID for T
    // where
    //     T: Serialize + DeserializeOwned + IntoChildID<C>,
    //     C: Serialize + DeserializeOwned,
    // {
    //     type Child = C;

    //     fn from_parent(parent: &mut C, id: IDType) -> anyhow::Result<&mut Self> {
    //         parent.into_child(id).map(|child| child as &mut Self)
    //     }
    // }

    // Blanket implementation for `IntoChildID` where `FromParentID` is implemented
    // impl<T, P> IntoChildID<P> for T
    // where
    //     T: Serialize + DeserializeOwned,
    //     P: Serialize + DeserializeOwned + FromParentID<Child = T>,
    // {
    //     fn into_child(&mut self, id: IDType) -> anyhow::Result<&mut P> {
    //         <P as FromParentID>::from_parent(self, id)
    //     }
    // }
}

use prelude::*;

#[derive(MessageEnum, Tsify, Debug, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Message {
    ProjectRename(crate::project::ProjectRename),
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
