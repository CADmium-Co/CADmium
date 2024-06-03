use cadmium_macros::MessageEnum;
use serde::{Deserialize, Serialize};
use tsify::Tsify;

pub mod prelude {
    use std::fmt;

    use serde::de::{self, MapAccess, Visitor};
    use serde::{Deserialize, Deserializer};
    use tsify::Tsify;

    pub use crate::IDType;

    #[derive(Tsify, Debug, Clone)]
    #[tsify(from_wasm_abi)]
    pub struct IDWrap<T: for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi> {
        pub id: u64,
        pub inner: T,
    }

    impl<T: for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi> IDWrap<T> {
        pub fn new(id: IDType, h: T) -> Self {
            Self {
                id,
                inner: h,
            }
        }

        pub fn id(&self) -> IDType {
            self.id
        }

        pub fn inner(&self) -> &T {
            &self.inner
        }
    }

    impl<'de, T, C> Deserialize<'de> for IDWrap<T>
    where
        T: MessageHandler<Parent = C> + for<'dh> Deserialize<'dh> + wasm_bindgen::convert::RefFromWasmAbi,
        C: Identifiable,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct IDWrapVisitor<T> {
                marker: std::marker::PhantomData<fn() -> T>,
            }

            impl<T> IDWrapVisitor<T> {
                fn new() -> Self {
                    IDWrapVisitor {
                        marker: std::marker::PhantomData,
                    }
                }
            }

            // Implementation of Visitor trait for IDWrapVisitor
            impl<'de, T, C> Visitor<'de> for IDWrapVisitor<IDWrap<T>>
            where
                T: MessageHandler<Parent = C> + for<'dh> Deserialize<'dh> + wasm_bindgen::convert::RefFromWasmAbi,
                C: Identifiable,
            {
                type Value = IDWrap<T>;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str(format!("a map with {}", C::ID_NAME).as_str())
                }

                fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut parent_id = None;
                    let mut inner_map = serde_json::Map::new();


                    // Loop through the map and extract fields
                    while let Some(key) = map.next_key::<String>()? {
                        println!("Key: {:?}", key);

                        if &key == C::ID_NAME {
                            if parent_id.is_some() {
                                return Err(de::Error::duplicate_field("parent_id"));
                            }
                            parent_id = Some(map.next_value()?);
                        } else {
                            // Collect the rest of the map entries for inner deserialization
                            let value: serde_json::Value = map.next_value()?;
                            inner_map.insert(key, value);
                        }
                    }

                    let id = parent_id.ok_or_else(|| de::Error::missing_field(C::ID_NAME))?;
                    let inner_value = serde_json::Value::Object(inner_map);
                    let inner = T::deserialize(inner_value).map_err(de::Error::custom)?;

                    // Construct the nested IDWrap structure
                    Ok(IDWrap {
                        id,
                        inner,
                    })
                }
            }

            deserializer.deserialize_map(IDWrapVisitor::new())
        }
    }

    // First level message handler
    impl<'p, T, U> ProjectMessageHandler for IDWrap<T>
    where
        T: MessageHandler<Parent = U> + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi,
        U: Identifiable<Parent = crate::project::Project>,
    {
        fn handle_project_message(&self, project: &mut crate::project::Project) -> anyhow::Result<Option<IDType>> {
            let prnt = U::from_parent_id(project, self.id)?;
            self.inner.handle_message(prnt)
        }
    }

    // Second level message handler
    impl<'p, T, C, P> MessageHandler for IDWrap<T>
    where
        T: MessageHandler<Parent = C> + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi,
        C: Identifiable<Parent = P>,
        P: Identifiable<Parent = crate::project::Project>,
    {
        type Parent = C::Parent;
        fn handle_message(&self, parent: Self::Parent) -> anyhow::Result<Option<IDType>> {
            let prnt = C::from_parent_id(&parent, self.id)?;
            self.inner.handle_message(prnt)
        }
    }

    pub trait Identifiable: Sized {
        type Parent;
        const ID_NAME: &'static str;
        fn from_parent_id(parent: &Self::Parent, id: IDType) -> Result<Self, anyhow::Error>;
    }

    pub trait ProjectMessageHandler: wasm_bindgen::convert::RefFromWasmAbi {
        fn handle_project_message(&self, project: &mut crate::project::Project) -> anyhow::Result<Option<IDType>>;
    }

    pub trait MessageHandler {
        type Parent: Identifiable;
        fn handle_message(&self, item: Self::Parent) -> anyhow::Result<Option<IDType>>;
    }
}

use prelude::*;

#[derive(MessageEnum, Tsify, Debug, Deserialize)]
#[tsify(from_wasm_abi)]
pub enum Message {
    ProjectRename(crate::project::ProjectRename),
    WorkbenchRename(IDWrap<crate::workbench::WorkbenchRename>),
    WorkbenchPointUpdate(IDWrap<IDWrap<crate::solid::point::WorkbenchPointUpdate>>),

    SketchAddPoint(IDWrap<IDWrap<crate::isketch::AddPoint>>),
    SketchAddArc(IDWrap<IDWrap<crate::isketch::AddArc>>),
    SketchAddCircle(IDWrap<IDWrap<crate::isketch::AddCircle>>),
    SketchAddLine(IDWrap<IDWrap<crate::isketch::AddLine>>),
    SketchDeletePrimitive(IDWrap<IDWrap<crate::isketch::DeletePrimitive>>),
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
