use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tsify_next::Tsify;

use crate::IDType;

use super::{Identifiable, MessageHandler, ProjectMessageHandler};

#[derive(Tsify, Debug, Clone)]
#[tsify(from_wasm_abi)]
pub struct IDWrap<T: Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi> {
    pub id: u64,
    pub inner: T,
}

impl<T: Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi> IDWrap<T> {
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

// First level message handler
impl<'p, T, U> ProjectMessageHandler for IDWrap<T>
where
    T: MessageHandler<Parent = U> + Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi,
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
    T: MessageHandler<Parent = C> + Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi,
    C: Identifiable<Parent = P>,
    P: Identifiable<Parent = crate::project::Project>,
{
    type Parent = C::Parent;
    fn handle_message(&self, parent: Self::Parent) -> anyhow::Result<Option<IDType>> {
        let prnt = C::from_parent_id(&parent, self.id)?;
        self.inner.handle_message(prnt)
    }
}

impl<'de, T, C> Deserialize<'de> for IDWrap<T>
where
    T: MessageHandler<Parent = C> + Serialize + for<'dh> Deserialize<'dh> + wasm_bindgen::convert::RefFromWasmAbi,
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
            T: MessageHandler<Parent = C> + Serialize + for<'dh> Deserialize<'dh> + wasm_bindgen::convert::RefFromWasmAbi,
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

impl<T, C> Serialize for IDWrap<T>
where
    T: MessageHandler<Parent = C> + Serialize + for<'de> Deserialize<'de> + wasm_bindgen::convert::RefFromWasmAbi,
    C: Identifiable,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Create a serializer map with the appropriate capacity
        let mut map = serializer.serialize_map(Some(1))?;
        // Add the id field using the name from the Identifiable trait
        map.serialize_entry(C::ID_NAME, &self.id)?;
        // Add the inner object fields
        serde_json::to_value(&self.inner)
            .map_err(serde::ser::Error::custom)?
            .as_object()
            .ok_or_else(|| serde::ser::Error::custom("Expected object"))?
            .iter().try_for_each(|(k, v)| {
                map.serialize_entry(k, v)?;
                Ok(())
            })?;
        map.end()
    }
}
