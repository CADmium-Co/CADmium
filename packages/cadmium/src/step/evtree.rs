pub mod loro_serde {
	//! Serialization and deserialization implementation for `LoroDoc`.
	//!
	//! To be used with `#[serde(with = "loro_serde")]` attribute.

	use loro::LoroDoc;
	use serde::Deserialize;

	pub fn serialize<S: serde::Serializer>(
		value: &LoroDoc,
		serializer: S,
	) -> Result<S::Ok, S::Error> {
		serializer.serialize_bytes(&value.export_snapshot())
	}

	pub fn deserialize<'de, D: serde::Deserializer<'de>>(
		deserializer: D,
	) -> Result<LoroDoc, D::Error> {
		let bytes = Vec::<u8>::deserialize(deserializer)?;
		let doc = LoroDoc::new();

		doc.import(&bytes).map_err(serde::de::Error::custom)?;

		Ok(doc)
	}
}
