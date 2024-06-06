use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::message::{Identifiable, MessageHandler};

use super::IDWrap;

impl<'de, T, C> Deserialize<'de> for IDWrap<T>
where
    T: MessageHandler<Parent = C> + Clone + Serialize + for<'dh> Deserialize<'dh> + wasm_bindgen::convert::RefFromWasmAbi,
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
            T: MessageHandler<Parent = C> + Clone + Serialize + for<'dh> Deserialize<'dh> + wasm_bindgen::convert::RefFromWasmAbi,
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
                    if key == C::ID_NAME {
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
