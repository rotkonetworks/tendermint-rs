//! AppHash serialization with validation

use alloc::borrow::Cow;

use serde::{de, ser, Deserialize, Deserializer, Serializer};
use subtle_encoding::{base64, hex};

use crate::{prelude::*, AppHash};

/// Deserialize an AppHash from JSON.
///
/// CometBFT 0.34/0.37 emit the app hash as uppercase hex. CometBFT 0.38
/// changed the wire format to base64. To support both peers from the
/// same client codebase, try hex first and fall back to base64.
pub fn deserialize<'de, D>(deserializer: D) -> Result<AppHash, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<Cow<'_, str>>::deserialize(deserializer)?.unwrap_or(Cow::Borrowed(""));
    if let Ok(h) = AppHash::from_hex_upper(&s) {
        return Ok(h);
    }
    let bytes = base64::decode(s.as_bytes()).map_err(|e| {
        de::Error::custom(format!(
            "apphash is neither uppercase hex nor base64: {e}"
        ))
    })?;
    AppHash::try_from(bytes).map_err(de::Error::custom)
}

/// Serialize from AppHash into hexstring
pub fn serialize<S>(value: &AppHash, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_bytes = hex::encode_upper(value.as_ref());
    let hex_string = String::from_utf8(hex_bytes).map_err(ser::Error::custom)?;
    // Serialize as Option<String> for symmetry with deserialize
    serializer.serialize_some(&hex_string)
}
