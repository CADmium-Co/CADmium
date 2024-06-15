use serde::ser::SerializeMap as _;
use serde::{Deserialize, Serialize, Serializer};

use crate::message::{Identifiable, MessageHandler};

use super::IDWrap;

impl<T, C> Serialize for IDWrap<T>
where
    T: MessageHandler<Parent = C>
        + Clone
        + Serialize
        + for<'de> Deserialize<'de>
        + wasm_bindgen::convert::RefFromWasmAbi,
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
            .iter()
            .try_for_each(|(k, v)| {
                map.serialize_entry(k, v)?;
                Ok(())
            })?;
        map.end()
    }
}
