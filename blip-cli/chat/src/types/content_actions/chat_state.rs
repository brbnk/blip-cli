use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatState {
    #[serde(rename = "state")]
    pub state: String,

    #[serde(rename = "interval", deserialize_with = "deserialize_u32_from_string_or_number")]
    pub interval: u32,
}

fn deserialize_u32_from_string_or_number<'de,D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or a number representing a u32")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.parse::<u32>().map_err(|_| E::custom(format!("cannot parse '{}' as u32", value)))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if value <= u32::MAX as u64 {
                Ok(value as u32)
            } else {
                Err(E::custom(format!("number {} is too large for u32", value)))
            }
        }
    }

    deserializer.deserialize_any(StringOrNumberVisitor)
}