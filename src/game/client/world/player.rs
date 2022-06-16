use std::fmt;

use cgmath::Point3;
use serde::{de::{self, Deserialize, Deserializer, Visitor, SeqAccess}, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub name     : String,
    pub position : Point3<f32>,
}

impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field { Name, Position }
        struct PlayerVisitor;

        impl<'de> Visitor<'de> for PlayerVisitor {
            type Value = Player;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Player")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let name = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let position = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                return Ok(Player { name, position });
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "position"];
        deserializer.deserialize_struct("Player", FIELDS, PlayerVisitor)
    }
}